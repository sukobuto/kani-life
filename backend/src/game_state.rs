use crate::command::game_cycle_command::GameCycleCommand;
use crate::command::player_command::{MoveParam, MoveResult, PlayerCommand, SpawnParam, TurnParam};
use crate::command::{Command, CommandResponse};
use crate::crab::Crab;
use crate::food::Food;
use crate::geometry::Position;
use crate::token::Token;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GameState {
    size: u32,
    crabs: Vec<Crab>,
    foods: Vec<Food>,
}

impl GameState {
    pub(crate) fn new(size: u32) -> GameState {
        GameState {
            size,
            crabs: vec![],
            foods: vec![],
        }
    }

    pub(crate) fn proc_command(&mut self, command: &Command) -> CommandResponse {
        match command {
            Command::PlayerCommand(command) => self.proc_player_command(command),
            Command::GameCycleCommand(command) => self.proc_game_cycle_command(command),
        }
    }

    fn proc_game_cycle_command(&mut self, command: &GameCycleCommand) -> CommandResponse {
        match command {
            GameCycleCommand::SpawnFood => self.spawn_food(),
        }
    }

    fn spawn_food(&mut self) -> CommandResponse {
        if self.foods.len() >= 5 {
            return CommandResponse::ok(100, false);
        }
        let keep_out: Vec<Position> = self
            .crabs
            .iter()
            .map(|c| c.position)
            .chain(self.foods.iter().map(|f| f.position))
            .collect();
        let food = Food::spawn(3, self.size, &keep_out);
        self.foods.push(food.clone());
        CommandResponse::ok(5000, true)
    }

    fn proc_player_command(&mut self, command: &PlayerCommand) -> CommandResponse {
        match command {
            PlayerCommand::Ping => CommandResponse::pong(),
            PlayerCommand::Spawn(param) => self.spawn(param),
            PlayerCommand::Turn(param) => self.turn(param),
            PlayerCommand::Move(param) => self.r#move(param),
            // todo: implement other commands
            _ => CommandResponse::not_implemented(),
        }
    }

    fn spawn(&mut self, param: &SpawnParam) -> CommandResponse {
        let keep_out: Vec<Position> = self
            .crabs
            .iter()
            .map(|c| c.position)
            .chain(self.foods.iter().map(|f| f.position))
            .collect();
        let crab = Crab::spawn(&param.name, param.hue, self.size, &keep_out);
        self.crabs.push(crab.clone());
        CommandResponse::spawn(crab.into())
    }

    fn find_crab_mut(&mut self, token: &Token) -> Option<&mut Crab> {
        self.crabs.iter_mut().find(|c| c.get_token() == *token)
    }

    fn turn(&mut self, param: &TurnParam) -> CommandResponse {
        let Some(crab) = self.find_crab_mut(&param.token) else {
            return CommandResponse::crab_not_found();
        };
        crab.turn_mut(param.side);
        CommandResponse::turn()
    }

    fn r#move(&mut self, param: &MoveParam) -> CommandResponse {
        let size = self.size as i32;
        let Some(crab) = self.find_crab_mut(&param.token) else {
            return CommandResponse::crab_not_found();
        };
        // todo 別のカニがいる場合の処理
        if crab.r#move(param.side).position.is_inset(size, size) {
            crab.move_mut(param.side);
            // todo ごはんがあった場合の処理
            CommandResponse::r#move(MoveResult {
                success: true,
                // todo ポイントの計算
                point: 0,
                total_point: crab.point,
            })
        } else {
            CommandResponse::r#move(MoveResult {
                success: false,
                point: 0,
                total_point: crab.point,
            })
        }
    }
}
