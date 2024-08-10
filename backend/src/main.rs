mod command;
mod crab;
mod food;
mod game_state;
mod geometry;
mod paint;
mod token;

use crate::command::{
    game_cycle_command::GameCycleCommand, player_command::PlayerCommand, Command, CommandResponse,
};
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

#[derive(Debug)]
struct GameCommandCase {
    command: Command,
    /// ゲームプロセッサがコマンド送信元に結果を返すためのセンダー
    callback_tx: oneshot::Sender<CommandResponse>,
}

#[derive(Debug, Clone)]
struct CommanderState {
    /// ゲームプロセッサにコマンドを送信するためのセンダー
    tx: mpsc::Sender<GameCommandCase>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (command_tx, command_rx) = mpsc::channel::<GameCommandCase>(100);

    let game_state = Arc::new(Mutex::new(game_state::GameState::new(30)));

    let commander_state = Arc::new(Mutex::new(CommanderState {
        tx: command_tx.clone(),
    }));

    let (socket_layer, socket_io) = socket_layer(game_state.clone());

    command_processor(game_state, command_rx, socket_io);
    game_cycle(command_tx.clone());

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

fn socket_layer(game_state: Arc<Mutex<game_state::GameState>>) -> (SocketIoLayer, SocketIo) {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", |s: SocketRef| {
        s.on("get", |s: SocketRef| async move {
            println!("Received get event");
            let state = game_state.lock().await.clone();
            s.emit("state", state).expect("TODO: panic message");
        })
    });

    (layer, io)
}

/// Handle a command by enqueueing it and waiting for the result
async fn post_command(
    State(state): State<Arc<Mutex<CommanderState>>>,
    Json(command): Json<PlayerCommand>,
) -> Result<Json<command::CommandResult>, StatusCode> {
    println!("Posted command: {:?}", command);
    let (response_tx, response_rx) = oneshot::channel::<CommandResponse>();
    let command_tx = state.lock().await.tx.clone();
    let send_result = command_tx
        .send(GameCommandCase {
            command: Command::PlayerCommand(command),
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

/// キュー (mpsc::channel) に積まれたコマンドを処理するループスレッド
fn command_processor(
    game_state: Arc<Mutex<game_state::GameState>>,
    mut command_rx: mpsc::Receiver<GameCommandCase>,
    io: SocketIo,
) {
    tokio::spawn(async move {
        while let Some(GameCommandCase {
            command,
            callback_tx,
        }) = command_rx.recv().await
        {
            let mut state = game_state.lock().await;
            let response = state.proc_command(&command);
            let mutated = response.mutated;
            if callback_tx.send(response).is_err() {
                eprintln!("Failed to send response");
            };
            if mutated && io.emit("state", state.clone()).is_err() {
                eprintln!("Failed to emit state");
                break;
            }
        }
    });
}

/// 自動的に食べ物を生成するなどのゲームサイクルを処理するループスレッド
fn game_cycle(command_tx: mpsc::Sender<GameCommandCase>) {
    // food loop
    tokio::spawn(async move {
        loop {
            let command = Command::GameCycleCommand(GameCycleCommand::SpawnFood);
            let (response_tx, response_rx) = oneshot::channel::<CommandResponse>();
            let send_result = command_tx
                .send(GameCommandCase {
                    command,
                    callback_tx: response_tx,
                })
                .await;
            if let Err(e) = send_result {
                eprintln!("Failed to send command: {}", e);
                break;
            }
            let response = response_rx.await;
            match response {
                Ok(response) => {
                    if response.wait > 0 {
                        sleep(Duration::from_millis(response.wait)).await;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to receive response: {}", e);
                    break;
                }
            }
        }
    });
}
