use axum::http::StatusCode;
use axum::routing::post;
use axum::{
    extract::{Json, State},
    Router,
};
use socketioxide::layer::SocketIoLayer;
use socketioxide::{extract::SocketRef, SocketIo};
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc, oneshot, Mutex},
    time::sleep,
};
use tower_http::services::ServeDir;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct GameState {
    count: usize,
}

#[derive(Debug)]
struct GameCommandCase {
    command: GameCommand,
    /// ゲームプロセッサがコマンド送信元に結果を返すためのセンダー
    callback_tx: oneshot::Sender<GameCommandResponse>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum GameCommand {
    Increment,
    Decrement,
}

#[derive(Debug, Clone)]
struct GameCommandResponse {
    result: GameCommandResult,
    wait: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum GameCommandResult {
    Incremented(GameState),
    Decremented(GameState),
}

#[derive(Debug)]
struct GameManagementState {
    state: GameState,
}

#[derive(Debug, Clone)]
struct CommanderState {
    /// ゲームプロセッサにコマンドを送信するためのセンダー
    tx: mpsc::Sender<GameCommandCase>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (command_tx, command_rx) = mpsc::channel::<GameCommandCase>(100);

    let game_management_state = Arc::new(Mutex::new(GameManagementState {
        state: GameState { count: 0 },
    }));

    let commander_state = Arc::new(Mutex::new(CommanderState {
        tx: command_tx.clone(),
    }));

    // command_tx は GameCycle に渡すために clone している

    let (socket_layer, socket_io) = socket_layer(game_management_state.clone());

    command_processor(game_management_state.clone(), command_rx, socket_io);

    let app = Router::new()
        .route("/api/command", post(post_command))
        .with_state(commander_state)
        .nest_service("/", ServeDir::new("static"))
        .layer(socket_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn socket_layer(
    game_management_state: Arc<Mutex<GameManagementState>>,
) -> (SocketIoLayer, SocketIo) {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", |s: SocketRef| {
        s.on("get", |s: SocketRef| async move {
            println!("Received get event");
            let state = game_management_state.lock().await.state.clone();
            s.emit("state", state).expect("TODO: panic message");
        })
    });

    (layer, io)
}

/// Handle a command by enqueueing it and waiting for the result
async fn post_command(
    State(state): State<Arc<Mutex<CommanderState>>>,
    Json(command): Json<GameCommand>,
) -> Result<Json<GameCommandResult>, StatusCode> {
    println!("Posted command: {:?}", command);
    let (response_tx, response_rx) = oneshot::channel::<GameCommandResponse>();
    let command_tx = state.lock().await.tx.clone();
    let send_result = command_tx
        .send(GameCommandCase {
            command,
            callback_tx: response_tx,
        })
        .await;
    if let Err(e) = send_result {
        eprintln!("Failed to send command: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    // rx で受け取った結果を返す
    let response = response_rx.await;
    match response {
        Ok(response) => {
            println!("Received response: {:?}", response);
            if response.wait > 0 {
                sleep(Duration::from_millis(response.wait)).await;
            }
            Ok(Json(response.result))
        }
        Err(e) => {
            eprintln!("Failed to receive response: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn command_processor(
    game_management_state: Arc<Mutex<GameManagementState>>,
    mut command_rx: mpsc::Receiver<GameCommandCase>,
    io: SocketIo,
) {
    tokio::spawn(async move {
        while let Some(GameCommandCase {
            command,
            callback_tx,
        }) = command_rx.recv().await
        {
            println!("Received command: {:?}", command);
            let mut game_management_state = game_management_state.lock().await;
            match command {
                GameCommand::Increment => {
                    game_management_state.state.count += 1;
                    let callback_result = callback_tx.send(GameCommandResponse {
                        result: GameCommandResult::Incremented(game_management_state.state.clone()),
                        wait: 0,
                    });
                    if callback_result.is_err() {
                        eprintln!("Failed to send response");
                    }
                }
                GameCommand::Decrement => {
                    game_management_state.state.count -= 1;
                    let callback_result = callback_tx.send(GameCommandResponse {
                        result: GameCommandResult::Decremented(game_management_state.state.clone()),
                        wait: 100,
                    });
                    if callback_result.is_err() {
                        eprintln!("Failed to send response");
                    }
                }
            }
            if io
                .emit("state", game_management_state.state.clone())
                .is_err()
            {
                eprintln!("Failed to emit state");
                break;
            }
        }
    });
}
