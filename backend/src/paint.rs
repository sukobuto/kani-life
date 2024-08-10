use crate::geometry::Position;
use crate::token::Token;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Paint {
    pub position: Position,
    #[serde(skip)]
    pub crab_token: Token,
    pub hue: f32,
}

impl Paint {
    pub(crate) fn new(position: Position, crab_token: Token, hue: f32) -> Self {
        Self {
            position,
            crab_token,
            hue,
        }
    }
}
