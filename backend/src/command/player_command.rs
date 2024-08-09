use crate::geometry::{Position, Side};
use crate::token::Token;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub(crate) enum PlayerCommand {
    Ping,
    Spawn(SpawnParam),
    Scan(ScanParam),
    Turn(TurnParam),
    Move(MoveParam),
    Paint(PaintParam),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SpawnParam {
    pub name: String,
    pub hue: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SpawnResult {
    pub token: Token,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScanParam {
    pub token: Token,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScanResult {
    pub what_you_can_see: WhatYouCanSee,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum WhatYouCanSee {
    Food,
    Crab,
    Wall,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TurnParam {
    pub token: Token,
    pub side: Side,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MoveParam {
    pub token: Token,
    pub side: Side,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MoveResult {
    /// 移動に成功したか
    pub success: bool,
    /// ゲットしたごはんポイント
    pub point: i32,
    /// トータルのごはんポイント
    pub total_point: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PaintParam {
    pub token: Token,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PaintResult {
    pub your_paints: Vec<Position>,
}
