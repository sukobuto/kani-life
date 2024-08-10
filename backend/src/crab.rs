use crate::command::player_command::SpawnResult;
use crate::geometry::{Direction, Position, Side};
use crate::token::Token;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Crab {
    pub(crate) name: String,
    #[serde(skip)]
    pub(crate) token: Token,
    pub(crate) hue: f32,
    pub(crate) point: i32,
    pub(crate) direction: Direction,
    pub(crate) position: Position,
}

impl Crab {
    pub(crate) fn spawn(name: &str, hue: f32, game_field_size: u32, keep_out: &[Position]) -> Self {
        // keep_out を避けてランダムに Position を生成する
        let position = loop {
            let pos = Position::random(game_field_size, game_field_size);
            if !keep_out.contains(&pos) {
                break pos;
            }
        };
        Crab {
            name: name.to_string(),
            token: Token::new(),
            hue,
            point: 0,
            direction: Direction::random(),
            position,
        }
    }

    pub(crate) fn get_token(&self) -> Token {
        self.token
    }

    pub(crate) fn walk(&self, side: Side) -> Self {
        Crab {
            position: self.position.walk(self.direction, side),
            ..self.clone()
        }
    }

    // 上記メソッドの mut 版
    pub(crate) fn turn_mut(&mut self, side: Side) {
        self.direction = self.direction.turn(side);
    }

    pub(crate) fn move_mut(&mut self, side: Side) {
        self.position = self.position.walk(self.direction, side);
    }
}

impl From<Crab> for SpawnResult {
    fn from(crab: Crab) -> Self {
        SpawnResult { token: crab.token }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn() {
        let crab = Crab::spawn("test", 0.0, 10, &[]);
        assert_eq!(crab.name, "test");
        assert_eq!(crab.hue, 0.0);
        assert_eq!(crab.point, 0);
        assert!(crab.position.is_inset(10, 10));
    }

    #[test]
    fn test_turn() {
        let mut crab = Crab {
            name: "test".to_string(),
            token: Token::new(),
            hue: 0.0,
            point: 0,
            direction: Direction::N,
            position: Position::new(0, 0),
        };
        crab.turn_mut(Side::Right);
        assert_eq!(crab.direction, Direction::E);
        assert_eq!(crab.position, Position::new(0, 0));
    }

    #[test]
    fn test_move() {
        let crab = Crab {
            name: "test".to_string(),
            token: Token::new(),
            hue: 0.0,
            point: 0,
            direction: Direction::N,
            position: Position::new(0, 0),
        };
        let crab_moved_right = crab.walk(Side::Right);
        assert_eq!(crab_moved_right.direction, Direction::N);
        assert_eq!(crab_moved_right.position, Position::new(1, 0));
        assert!(crab_moved_right.position.is_inset(10, 10));
        let crab_moved_left = crab.walk(Side::Left);
        assert_eq!(crab_moved_left.direction, Direction::N);
        assert_eq!(crab_moved_left.position, Position::new(-1, 0));
        assert!(!crab_moved_left.position.is_inset(10, 10));
    }
}
