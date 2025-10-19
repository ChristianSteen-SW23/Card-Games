use std::sync::{Arc, Mutex};

use server_rust::{objects::{states::ServerState, Game7Logic, GameLogic}, responses::LobbyResponse, run_test_server, socket::{lobby_socket::LobbyEvents, ErrorResponse, LobbyPayload}};

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
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    assert_eq!(msg, "Hello World!");
}

#[test]
fn test_lobby_control_event_createlobby_no_name_400() {
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
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    // parse into your ErrorResponse struct
    let err: ErrorResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    assert_eq!(err.message, "You need to send a username");
    assert_eq!(err.r#type, "Lobby Error");
}

#[test]
fn test_lobby_control_event_createlobby_shortname_400() {
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
        username: Some("a".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    socket
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    let msg = rx
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    // parse into your ErrorResponse struct
    let err: ErrorResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    assert_eq!(err.message, "Your username did not uphold the high standards required");
    assert_eq!(err.r#type, "Lobby Error");
}

#[test]
fn test_lobby_control_event_createlobby_200() {
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
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    // parse into your ErrorResponse struct
    let res: LobbyResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    assert!((0..10).contains(&res.id));
    assert_eq!(res.players.len(), 1);
    assert_eq!(res.players[0].name, "player1");

    let state = state.lock().unwrap();
    assert_eq!(state.player_lobby_map.len(), 1);
    assert!(state.game_map.contains_key(&res.id));
    // let Ok(GameLogic::LobbyLogic(idk)): Option<_> = state.game_map.get(&res.id).unwrap().lock();

    let lobby_arc = state.game_map.get(&res.id).unwrap();
    let lobby = lobby_arc.lock().unwrap();
    let GameLogic::LobbyLogic(ref lobby) = *lobby;

    assert_eq!(lobby.get_players().get_all()[0].name, "player1");
    assert_eq!(lobby.get_game_id(), res.id);
}

#[test]
fn sanity_check() {
    assert!(true.eq(&true));
}
