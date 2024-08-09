use crate::geometry::Position;
use crate::token::Token;
use rand::Rng;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Food {
    id: Token,
    pub position: Position,
    size: u32,
}

impl Food {
    pub(crate) fn spawn(max_size: u32, game_field_size: u32, keep_out: &[Position]) -> Self {
        // keep_out を避けてランダムに Position を生成する
        let position = loop {
            let pos = Position::random(game_field_size, game_field_size);
            if !keep_out.contains(&pos) {
                break pos;
            }
        };
        Self {
            id: Token::new(),
            position,
            // random で生成されるので 1 以上
            size: rand::thread_rng().gen_range(1..=max_size),
        }
    }
}
