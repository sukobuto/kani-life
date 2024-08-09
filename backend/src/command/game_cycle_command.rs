#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum GameCycleCommand {
    SpawnFood,
}
