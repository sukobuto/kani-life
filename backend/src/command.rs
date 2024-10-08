pub(crate) mod game_cycle_command;
pub(crate) mod player_command;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum Command {
    PlayerCommand(player_command::PlayerCommand),
    GameCycleCommand(game_cycle_command::GameCycleCommand),
}

use player_command::{PaintResult, ScanResult, SpawnResult, WalkResult};
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub(crate) enum CommandResult {
    Ok,
    Pong,
    NotImplemented,
    CrabNotFound,
    Spawn(SpawnResult),
    Scan(ScanResult),
    Turn,
    Walk(WalkResult),
    Paint(PaintResult),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct CommandResponse {
    pub(crate) result: CommandResult,
    pub(crate) wait: u64,
    pub(crate) mutated: bool,
}

impl CommandResponse {
    pub(crate) fn ok(wait: u64, mutated: bool) -> Self {
        CommandResponse {
            result: CommandResult::Ok,
            wait,
            mutated,
        }
    }

    pub(crate) fn pong() -> Self {
        CommandResponse {
            result: CommandResult::Pong,
            wait: 0,
            mutated: false,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn not_implemented() -> Self {
        CommandResponse {
            result: CommandResult::NotImplemented,
            wait: 0,
            mutated: false,
        }
    }

    pub(crate) fn crab_not_found() -> Self {
        CommandResponse {
            result: CommandResult::CrabNotFound,
            wait: 0,
            mutated: false,
        }
    }

    pub(crate) fn spawn(result: SpawnResult) -> Self {
        CommandResponse {
            result: CommandResult::Spawn(result),
            wait: 0,
            mutated: true,
        }
    }

    pub(crate) fn scan(result: ScanResult) -> Self {
        CommandResponse {
            result: CommandResult::Scan(result),
            wait: 0,
            mutated: false,
        }
    }

    pub(crate) fn turn() -> Self {
        CommandResponse {
            result: CommandResult::Turn,
            wait: 100,
            mutated: true,
        }
    }

    pub(crate) fn walk(result: WalkResult) -> Self {
        CommandResponse {
            result: CommandResult::Walk(result),
            wait: 500,
            mutated: true,
        }
    }

    pub(crate) fn paint(result: PaintResult) -> Self {
        CommandResponse {
            result: CommandResult::Paint(result),
            wait: 100,
            mutated: true,
        }
    }
}
