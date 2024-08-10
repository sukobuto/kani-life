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
        // すでに同じ名前のカニがいる場合はエラー
        if self.crabs.iter().any(|c| c.name == param.name) {
            return CommandResponse::crab_already_exists();
        }
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

    fn find_crab(&self, token: &Token) -> Option<&Crab> {
        self.crabs.iter().find(|c| c.get_token() == *token)
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
        let Some(crab) = self.find_crab(&param.token) else {
            return CommandResponse::crab_not_found();
        };
        let new_pos = crab.r#move(param.side).position;
        if !new_pos.is_inset(size, size) || self.find_crab_by_position(&new_pos).is_some() {
            return CommandResponse::r#move(MoveResult {
                success: false,
                point: 0,
                total_point: crab.point,
            });
        }
        let food = self.take_food_by_position(&new_pos);
        let crab = self.find_crab_mut(&param.token).unwrap();
        crab.move_mut(param.side);
        if let Some(food) = &food {
            crab.point += food.size as i32;
        }
        CommandResponse::r#move(MoveResult {
            success: true,
            point: food.map(|f| f.size as i32).unwrap_or(0),
            total_point: crab.point,
        })
    }

    fn find_crab_by_position(&self, position: &Position) -> Option<&Crab> {
        self.crabs.iter().find(|c| c.position == *position)
    }

    fn take_food_by_position(&mut self, position: &Position) -> Option<Food> {
        let index = self.foods.iter().position(|f| f.position == *position);
        if let Some(index) = index {
            return Some(self.foods.remove(index));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::CommandResult;
    use crate::geometry::{Direction, Side};

    #[test]
    fn test_crab_collides_to_wall() {
        let token = Token::new();
        //  +----+----+
        //  | 🦀 |    |
        //  +----+----+
        //  |    |    |
        //  +----+----+
        let mut state = GameState {
            size: 2,
            crabs: vec![Crab {
                name: "player".to_string(),
                token,
                hue: 0.0,
                point: 0,
                direction: Direction::N,
                position: Position::new(0, 0),
            }],
            foods: vec![],
        };

        // 右に一度移動できる
        let command = Command::PlayerCommand(PlayerCommand::Move(MoveParam {
            token,
            side: Side::Right,
        }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Move(MoveResult {
                success: true,
                point: 0,
                total_point: 0,
            })
        );
        assert_eq!(state.crabs[0].position, Position::new(1, 0));
        // 二度目は移動できない（壁にぶつかる）
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Move(MoveResult {
                success: false,
                point: 0,
                total_point: 0,
            })
        );
        assert_eq!(state.crabs[0].position, Position::new(1, 0));
    }

    #[test]
    fn test_crab_collides_to_other_crab() {
        let token = Token::new();
        //  +----+----+----+
        //  | 🦀 |    | 🦀 |
        //  +----+----+----+
        //  |    |    |    |
        //  +----+----+----+
        //  |    |    |    |
        //  +----+----+----+
        let mut state = GameState {
            size: 3,
            crabs: vec![
                Crab {
                    name: "player".to_string(),
                    token,
                    hue: 0.0,
                    point: 0,
                    direction: Direction::N,
                    position: Position::new(0, 0),
                },
                Crab {
                    name: "other".to_string(),
                    token: Token::new(),
                    hue: 0.0,
                    point: 0,
                    direction: Direction::N,
                    position: Position::new(2, 0),
                },
            ],
            foods: vec![],
        };

        // 右に一度移動できる
        let command = Command::PlayerCommand(PlayerCommand::Move(MoveParam {
            token,
            side: Side::Right,
        }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Move(MoveResult {
                success: true,
                point: 0,
                total_point: 0,
            })
        );
        assert_eq!(state.crabs[0].position, Position::new(1, 0));
        // 二度目は移動できない（他カニにぶつかる）
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Move(MoveResult {
                success: false,
                point: 0,
                total_point: 0,
            })
        );
        assert_eq!(state.crabs[0].position, Position::new(1, 0));
    }

    #[test]
    fn test_crab_eats_food() {
        let token = Token::new();
        //  +----+----+
        //  | 🦀 | 🍙 |
        //  +----+----+
        //  |    |    |
        //  +----+----+
        let mut state = GameState {
            size: 2,
            crabs: vec![Crab {
                name: "player".to_string(),
                token,
                hue: 0.0,
                point: 0,
                direction: Direction::N,
                position: Position::new(0, 0),
            }],
            foods: vec![Food {
                id: Token::new(),
                position: Position::new(1, 0),
                size: 1,
            }],
        };

        // 右に一度移動できる
        let command = Command::PlayerCommand(PlayerCommand::Move(MoveParam {
            token,
            side: Side::Right,
        }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Move(MoveResult {
                success: true,
                point: 1,
                total_point: 1,
            })
        );
        assert_eq!(state.crabs[0].position, Position::new(1, 0));
        assert_eq!(state.crabs[0].point, 1);
        assert_eq!(state.foods.len(), 0);
    }
}
