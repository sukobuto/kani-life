use crate::command::game_cycle_command::GameCycleCommand;
use crate::command::player_command::{
    PaintParam, PaintResult, PlayerCommand, ScanParam, ScanResult, SpawnParam, TurnParam,
    WalkParam, WalkResult, WhatYouCanSee,
};
use crate::command::{Command, CommandResponse};
use crate::crab::Crab;
use crate::food::Food;
use crate::geometry::Position;
use crate::paint::Paint;
use crate::token::Token;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GameState {
    size: u32,
    crabs: Vec<Crab>,
    foods: Vec<Food>,
    paints: Vec<Paint>,
}

impl GameState {
    pub(crate) fn new(size: u32) -> GameState {
        GameState {
            size,
            crabs: vec![],
            foods: vec![],
            paints: vec![],
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
            PlayerCommand::Walk(param) => self.walk(param),
            PlayerCommand::Scan(param) => self.scan(param),
            PlayerCommand::Paint(param) => self.paint(param),
        }
    }

    fn spawn(&mut self, param: &SpawnParam) -> CommandResponse {
        // すでに同じ名前のカニがいる場合は、同じ名前のカニを除去する
        // 除去せずエラーとするほうが安全だが、プログラミングハンズオンの性質的にリトライのしやすさを優先する
        if let Some(index) = self.crabs.iter().position(|c| c.name == param.name) {
            let old_token = self.crabs[index].get_token();
            self.crabs.remove(index);
            // 除去したカニのペイントも削除
            self.paints = self
                .paints
                .iter()
                .filter(|p| p.crab_token != old_token)
                .cloned()
                .collect();
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

    fn walk(&mut self, param: &WalkParam) -> CommandResponse {
        let size = self.size as i32;
        let Some(crab) = self.find_crab(&param.token) else {
            return CommandResponse::crab_not_found();
        };
        let new_pos = crab.walk(param.side).position;
        if !new_pos.is_inset(size, size) || self.find_crab_by_position(&new_pos).is_some() {
            return CommandResponse::walk(WalkResult {
                success: false,
                point: 0,
                total_point: crab.point,
            });
        }
        let food = self.take_food_by_position(&new_pos);
        let crab = self.find_crab_mut(&param.token).unwrap();
        crab.walk_mut(param.side);
        if let Some(food) = &food {
            crab.point += food.size as i32;
        }
        CommandResponse::walk(WalkResult {
            success: true,
            point: food.map(|f| f.size as i32).unwrap_or(0),
            total_point: crab.point,
        })
    }

    fn scan(&self, param: &ScanParam) -> CommandResponse {
        let size = self.size as i32;
        let Some(crab) = self.find_crab(&param.token) else {
            return CommandResponse::crab_not_found();
        };
        let mut pos = crab.position;
        let direction = crab.direction;
        while pos.is_inset(size, size) {
            pos = pos.forward(direction);
            if self.find_crab_by_position(&pos).is_some() {
                return CommandResponse::scan(ScanResult {
                    what_you_can_see: WhatYouCanSee::Crab,
                });
            }
            if self.foods.iter().any(|f| f.position == pos) {
                return CommandResponse::scan(ScanResult {
                    what_you_can_see: WhatYouCanSee::Food,
                });
            }
        }
        CommandResponse::scan(ScanResult {
            what_you_can_see: WhatYouCanSee::Wall,
        })
    }

    fn paint(&mut self, param: &PaintParam) -> CommandResponse {
        let Some(crab) = self.find_crab(&param.token) else {
            return CommandResponse::crab_not_found();
        };
        // ポイントがない場合は失敗
        if crab.point <= 0 {
            return CommandResponse::paint(PaintResult {
                success: false,
                your_paints: self
                    .paints
                    .iter()
                    .filter(|p| p.crab_token == param.token)
                    .map(|p| p.position)
                    .collect(),
                total_point: 0,
            });
        }
        let paint = Paint::new(crab.position, crab.token, crab.hue);
        // 同じ位置にあるペイントを削除
        if let Some(index) = self.paints.iter().position(|p| p.position == crab.position) {
            self.paints.remove(index);
        };
        self.paints.push(paint.clone());
        let paints = self
            .paints
            .iter()
            .filter(|p| p.crab_token == param.token)
            .map(|p| p.position)
            .collect();
        let crab = self.find_crab_mut(&param.token).unwrap();
        crab.point -= 1;
        CommandResponse::paint(PaintResult {
            success: true,
            your_paints: paints,
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
    use crate::command::player_command::{
        PaintParam, PaintResult, ScanParam, ScanResult, WhatYouCanSee,
    };
    use crate::command::CommandResult;
    use crate::geometry::{Direction, Side};

    #[test]
    fn test_crab_collides_to_wall() {
        let token = Token::new();
        //  +----+----+
        //  | 🦀 |    |  <- player
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
            paints: vec![],
        };

        // 右に一度移動できる
        let command = Command::PlayerCommand(PlayerCommand::Walk(WalkParam {
            token,
            side: Side::Right,
        }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Walk(WalkResult {
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
            CommandResult::Walk(WalkResult {
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
        //    ^player   ^other
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
            paints: vec![],
        };

        // Act

        // 右に一度移動できる
        let command = Command::PlayerCommand(PlayerCommand::Walk(WalkParam {
            token,
            side: Side::Right,
        }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Walk(WalkResult {
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
            CommandResult::Walk(WalkResult {
                success: false,
                point: 0,
                total_point: 0,
            })
        );
        assert_eq!(state.crabs[0].position, Position::new(1, 0));
        // 左に移動できる
        let command = Command::PlayerCommand(PlayerCommand::Walk(WalkParam {
            token,
            side: Side::Left,
        }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Walk(WalkResult {
                success: true,
                point: 0,
                total_point: 0,
            })
        );
        assert_eq!(state.crabs[0].position, Position::new(0, 0));
    }

    #[test]
    fn test_crab_eats_food() {
        let token = Token::new();
        //  +----+----+
        //  | 🦀 | 🍙 |  <- player and food
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
            paints: vec![],
        };

        // Act

        // 右に一度移動できる
        let command = Command::PlayerCommand(PlayerCommand::Walk(WalkParam {
            token,
            side: Side::Right,
        }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Walk(WalkResult {
                success: true,
                point: 1,
                total_point: 1,
            })
        );
        assert_eq!(state.crabs[0].position, Position::new(1, 0));
        assert_eq!(state.crabs[0].point, 1);
        assert_eq!(state.foods.len(), 0);
    }

    #[test]
    fn test_crab_turns() {
        let token = Token::new();
        //  +----+----+
        //  |    | 🦀 |  <- player
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
                position: Position::new(1, 0),
            }],
            foods: vec![],
            paints: vec![],
        };

        // Act

        // 右に旋回
        let command = Command::PlayerCommand(PlayerCommand::Turn(TurnParam {
            token,
            side: Side::Right,
        }));
        let response = state.proc_command(&command);
        assert_eq!(response.result, CommandResult::Turn);
        assert_eq!(state.crabs[0].direction, Direction::E);
        assert_eq!(state.crabs[0].position, Position::new(1, 0));

        // 右に移動
        let command = Command::PlayerCommand(PlayerCommand::Walk(WalkParam {
            token,
            side: Side::Right,
        }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Walk(WalkResult {
                success: true,
                point: 0,
                total_point: 0,
            })
        );
        assert_eq!(state.crabs[0].direction, Direction::E);
        assert_eq!(state.crabs[0].position, Position::new(1, 1));

        // 右に旋回(2回目)
        let command = Command::PlayerCommand(PlayerCommand::Turn(TurnParam {
            token,
            side: Side::Right,
        }));
        let response = state.proc_command(&command);
        assert_eq!(response.result, CommandResult::Turn);
        assert_eq!(state.crabs[0].direction, Direction::S);
        assert_eq!(state.crabs[0].position, Position::new(1, 1));

        // 右に移動
        let command = Command::PlayerCommand(PlayerCommand::Walk(WalkParam {
            token,
            side: Side::Right,
        }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Walk(WalkResult {
                success: true,
                point: 0,
                total_point: 0,
            })
        );
        assert_eq!(state.crabs[0].direction, Direction::S);
        assert_eq!(state.crabs[0].position, Position::new(0, 1));
    }

    #[test]
    fn test_crab_sees_wall_crab_and_food() {
        let token = Token::new();
        //  +----+----+----+
        //  |    | 🍙 | 🦀 |
        //  +----+----+----+
        //  |    | 🦀 | 🍙 |
        //  +----+----+----+
        //  | 🦀 |    |    |  <- player
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
                    position: Position::new(0, 2),
                },
                Crab {
                    name: "other".to_string(),
                    token: Token::new(),
                    hue: 0.0,
                    point: 0,
                    direction: Direction::N,
                    position: Position::new(1, 0),
                },
            ],
            foods: vec![Food {
                id: Token::new(),
                position: Position::new(2, 1),
                size: 1,
            }],
            paints: vec![],
        };

        // Act

        // スキャンすると壁が見える
        let command = Command::PlayerCommand(PlayerCommand::Scan(ScanParam { token }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Scan(ScanResult {
                what_you_can_see: WhatYouCanSee::Wall
            })
        );

        // 右に移動
        let command = Command::PlayerCommand(PlayerCommand::Walk(WalkParam {
            token,
            side: Side::Right,
        }));
        let _ = state.proc_command(&command);

        // スキャンすると他カニが見える
        let command = Command::PlayerCommand(PlayerCommand::Scan(ScanParam { token }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Scan(ScanResult {
                what_you_can_see: WhatYouCanSee::Crab
            })
        );

        // 右に移動
        let command = Command::PlayerCommand(PlayerCommand::Walk(WalkParam {
            token,
            side: Side::Right,
        }));
        let _ = state.proc_command(&command);

        // スキャンするとごはんが見える
        let command = Command::PlayerCommand(PlayerCommand::Scan(ScanParam { token }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Scan(ScanResult {
                what_you_can_see: WhatYouCanSee::Food
            })
        );
    }

    #[test]
    fn test_bug_of_scan() {
        let token = Token::new();
        //  +----+----+----+
        //  | 🦀↓|    |    | <- player
        //  +----+----+----+
        //  |    |    |    |
        //  +----+----+----+
        //  | 🍙 |    |    |
        //  +----+----+----+
        let mut state = GameState {
            size: 3,
            crabs: vec![Crab {
                name: "player".to_string(),
                token,
                hue: 0.0,
                point: 0,
                direction: Direction::S,
                position: Position::new(0, 0),
            }],
            foods: vec![Food {
                id: Token::new(),
                position: Position::new(0, 2),
                size: 1,
            }],
            paints: vec![],
        };

        // Act

        // スキャンするとご飯が見える
        let command = Command::PlayerCommand(PlayerCommand::Scan(ScanParam { token }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Scan(ScanResult {
                what_you_can_see: WhatYouCanSee::Food
            })
        );
    }

    #[test]
    fn test_crab_paints() {
        let token = Token::new();
        //  +----+----+
        //  | 🦀 |    |  <- player
        //  +----+----+
        //  |    |    |
        //  +----+----+
        let mut state = GameState {
            size: 2,
            crabs: vec![Crab {
                name: "player".to_string(),
                token,
                hue: 0.0,
                point: 1,
                direction: Direction::N,
                position: Position::new(0, 0),
            }],
            foods: vec![],
            paints: vec![],
        };

        // Act

        // ポイントを消費して真下のマスを塗る
        let command = Command::PlayerCommand(PlayerCommand::Paint(PaintParam { token }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Paint(PaintResult {
                success: true,
                your_paints: vec![Position::new(0, 0)],
                total_point: 0,
            })
        );

        // 右に一度移動できる
        let command = Command::PlayerCommand(PlayerCommand::Walk(WalkParam {
            token,
            side: Side::Right,
        }));
        let _ = state.proc_command(&command);
        assert_eq!(state.crabs[0].position, Position::new(1, 0));

        // ポイントがないので塗れない
        let command = Command::PlayerCommand(PlayerCommand::Paint(PaintParam { token }));
        let response = state.proc_command(&command);
        assert_eq!(
            response.result,
            CommandResult::Paint(PaintResult {
                success: false,
                your_paints: vec![Position::new(0, 0)],
                total_point: 0,
            })
        );
    }
}
