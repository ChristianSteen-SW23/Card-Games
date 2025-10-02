use rust_socketio::{ClientBuilder, Payload};
use serde::Serialize;
use server_rust::socket::{LobbyPayload, ErrorResponse};
use server_rust::socket::lobby_socket::{LobbyEvents, LobbyResponse};
use server_rust::{run_test_server, state::ServerState};
use std::any::Any;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_test_server(port: u16) -> Arc<Mutex<ServerState>> {
    let state = Arc::new(Mutex::new(ServerState::new()));
    let state_clone = state.clone();

    // spawn runtime + server
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            run_test_server(&format!("127.0.0.1:{port}"), state_clone).await;
        });
    });

    // wait for server bind
    thread::sleep(Duration::from_millis(300));
    state
}

#[test]
fn test_message_event() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    // Start tokio runtime manually
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn({
        let state = state.clone();
        async move {
            run_test_server("127.0.0.1:4001", state).await;
        }
    });

    // Wait a bit for server
    std::thread::sleep(std::time::Duration::from_millis(300));

    // Use sync rust_socketio client
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    let socket = rust_socketio::ClientBuilder::new("http://127.0.0.1:4001")
        .on("message-back", move |payload, _| match payload {
            rust_socketio::Payload::Text(values) => {
                if let Some(v) = values.get(0).and_then(|v| v.as_str()) {
                    tx.send(v.to_string()).unwrap();
                }
            }
            _ => {}
        })
        .connect()
        .expect("could not connect");

    socket.emit("message", "").expect("emit failed");

    let msg = rx
        .recv_timeout(std::time::Duration::from_secs(2))
        .expect("no response");
    assert_eq!(msg, "Hello World!");
}

#[test]
fn test_lobbyControl_event_createlobby_shortname_400() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    // Start tokio runtime manually
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn({
        let state = state.clone();
        async move {
            run_test_server("127.0.0.1:4001", state).await;
        }
    });

    // Wait a bit for server
    std::thread::sleep(std::time::Duration::from_millis(300));

    // Use sync rust_socketio client
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    let socket = rust_socketio::ClientBuilder::new("http://127.0.0.1:4001")
    .on("errorMessage", move |payload, _| {
        println!("Got payload: {:?}", payload);
        match payload {
            rust_socketio::Payload::Text(values) => {
                if let Some(v) = values.get(0) {
                    let _ = tx.send(v.to_string());
                }
            }
            rust_socketio::Payload::Binary(bin) => {
                let _ = tx.send(format!("(binary) {:?}", bin.len()));
            }
            rust_socketio::Payload::String(s) => {
                let _ = tx.send(s);
            }
        }
    })
    .connect()
    .expect("could not connect");

    let data = LobbyPayload {
        username: None, /*Some("player1".to_string())*/
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    socket
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    let msg = rx
        .recv_timeout(std::time::Duration::from_secs(1))
        .expect("no response");
    // parse into your ErrorResponse struct
    let err: ErrorResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    assert_eq!(err.message, "No username given");
    assert_eq!(err.r#type, "???");
}

#[test]
fn test_lobbyControl_event_createlobby_200() {
    let state = Arc::new(Mutex::new(ServerState::new()));

    // Start tokio runtime manually
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn({
        let state = state.clone();
        async move {
            run_test_server("127.0.0.1:4001", state).await;
        }
    });

    // Wait a bit for server
    std::thread::sleep(std::time::Duration::from_millis(300));

    // Use sync rust_socketio client
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    let socket = rust_socketio::ClientBuilder::new("http://127.0.0.1:4001")
    .on("conToLobby", move |payload, _| {
        println!("Got payload: {:?}", payload);
        match payload {
            rust_socketio::Payload::Text(values) => {
                if let Some(v) = values.get(0) {
                    let _ = tx.send(v.to_string());
                }
            }
            rust_socketio::Payload::Binary(bin) => {
                let _ = tx.send(format!("(binary) {:?}", bin.len()));
            }
            rust_socketio::Payload::String(s) => {
                let _ = tx.send(s);
            }
        }
    })
    .connect()
    .expect("could not connect");

    let data = LobbyPayload {
        username: Some("player1".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    socket
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    let msg = rx
        .recv_timeout(std::time::Duration::from_secs(1))
        .expect("no response");
    // parse into your ErrorResponse struct
    let res: LobbyResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    assert!((0..10).contains(&res.id));
    assert_eq!(res.players.len(), 1);
    assert_eq!(res.players[0].name, "player1");

    let state = state.lock().unwrap();
    assert!(state.player_lobby.len() == 1);
    assert!(state.lobbies.contains_key(&res.id));
    assert_eq!(state.lobbies.get(&res.id).unwrap().lock().unwrap().players[0].name, "player1");
    assert_eq!(state.lobbies.get(&res.id).unwrap().lock().unwrap().id, res.id);
}

#[test]
fn sanity_check() {
    assert!(true);
}
