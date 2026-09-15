use rust_socketio::{client::Client, ClientBuilder, Payload};
use serde::{Serialize, de::DeserializeOwned};
use std::{cell::RefCell, collections::{HashMap, HashSet}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex, mpsc::{self, Receiver}}, time::Instant};
use tokio::runtime::Runtime;
use std::time::Duration;
use server_rust::{objects::{GameLogic, states::ServerState}, run_test_server, socket::{LobbyPayload, lobby_socket::LobbyEvents}};


static WAIT_TIME: u64 = 3;

thread_local! {
    static TEST_ADDR: RefCell<Option<String>> = RefCell::new(None);
}

fn next_test_addr() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind");
    let addr = listener.local_addr().unwrap();
    drop(listener); // free it immediately so your server can bind it next
    addr.to_string()
}

fn current_test_addr() -> String {
    TEST_ADDR.with(|a| {
        a.borrow()
            .clone()
            .expect("no test server address set — did you call setup_test_with_listener first?")
    })
}

fn wait_for_server_ready(addr: &str) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if TcpStream::connect(addr).is_ok() {
            return;
        }
        if Instant::now() >= deadline {
            panic!("test server never became ready at {addr}");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// Starts a test server and returns the shared state, runtime, socket, and receiver.
pub fn setup_test_with_listener(event_name: &str) -> (Arc<Mutex<ServerState>>, Runtime, Client, Receiver<String>) {
    let addr = next_test_addr();
    TEST_ADDR.with(|a| *a.borrow_mut() = Some(addr.clone()));
    // --- Start server ---
    let state: Arc<Mutex<ServerState>> = Arc::new(Mutex::new(ServerState::new()));
    let rt = Runtime::new().unwrap();

    {
        let addr_clone = addr.clone();
        let state_clone = state.clone();
        rt.spawn(async move {
            run_test_server(&addr_clone, state_clone).await;
        });
    }

    // Give the server a moment to start
    wait_for_server_ready(&addr);

    // --- Setup Socket.IO client ---
    let (tx, rx) = mpsc::channel::<String>();
    let event = event_name.to_string();

    let tx_clone = tx.clone();
    let socket = rust_socketio::ClientBuilder::new(format!("http://{}",addr.to_owned()))
        .on(&*event, move |payload, _| {
            // println!("Got payload for '{}': {:?}", event, payload);
            match payload {
                Payload::Text(values) => {
                    if let Some(v) = values.get(0) {
                        let _ = tx_clone.send(v.to_string());
                    }
                }
                Payload::Binary(bin) => {
                    let _ = tx_clone.send(format!("(binary) {:?}", bin.len()));
                }
                Payload::String(s) => {
                    let _ = tx_clone.send(s);
                }
            }
        })
        .connect()
        .expect("could not connect to test server");

    (state, rt, socket, rx)
}

pub fn setup_test_with_listeners(
    events: &[&str],
) -> (Arc<Mutex<ServerState>>, Runtime, Client, HashMap<String, Receiver<String>>) {
    let addr = next_test_addr();
    TEST_ADDR.with(|a| *a.borrow_mut() = Some(addr.clone()));
    let state: Arc<Mutex<ServerState>> = Arc::new(Mutex::new(ServerState::new()));
    let rt = Runtime::new().unwrap();

    {
        let addr_clone = addr.clone();
        let state_clone = state.clone();
        rt.spawn(async move {
            run_test_server(&addr_clone, state_clone).await;
        });
    }

    wait_for_server_ready(&addr);

    let mut receivers = HashMap::new();
    let mut builder = rust_socketio::ClientBuilder::new(format!("http://{}",addr.to_owned()));

    for &event in events {
        let (tx, rx) = mpsc::channel::<String>();
        receivers.insert(event.to_string(), rx);

        builder = builder.on(event, move |payload, _| match payload {
            Payload::Text(values) => {
                if let Some(v) = values.get(0) {
                    let _ = tx.send(v.to_string());
                }
            }
            Payload::Binary(bin) => {
                let _ = tx.send(format!("(binary) {:?}", bin.len()));
            }
            Payload::String(s) => {
                let _ = tx.send(s);
            }
        });
    }

    let socket = builder.connect().expect("could not connect to test server");

    (state, rt, socket, receivers)
}


/// Connects to a Socket.IO server and listens for a given event name.
/// Returns both the connected client and the receiver channel for incoming messages.
pub fn connect_with_listener(event_name: &str) -> (Client, Receiver<String>) {
    let addr = current_test_addr();
    let (tx, rx) = mpsc::channel::<String>();

    let tx_clone = tx.clone();
    let socket = ClientBuilder::new(format!("http://{addr}"))
        .on(event_name, move |payload, _| {
            match payload {
                Payload::Text(values) => {
                    if let Some(v) = values.get(0) {
                        let _ = tx_clone.send(v.to_string());
                    }
                }
                Payload::Binary(bin) => {
                    let _ = tx_clone.send(format!("(binary) {:?}", bin.len()));
                }
                Payload::String(s) => {
                    let _ = tx_clone.send(s);
                }
            }
        })
        .connect()
        .expect("Could not connect to Socket.IO server");

    (socket, rx)
}

pub fn connect_with_listeners(events: &[&str]) -> (Client, HashMap<String, Receiver<String>>) {
    let addr = current_test_addr();
    let mut receivers = HashMap::new();
    let mut builder = ClientBuilder::new(format!("http://{addr}"));

    for &event in events {
        let (tx, rx) = mpsc::channel::<String>();
        receivers.insert(event.to_string(), rx);

        builder = builder.on(event, move |payload, _| match payload {
            Payload::Text(values) => {
                if let Some(v) = values.get(0) {
                    let _ = tx.send(v.to_string());
                }
            }
            Payload::Binary(bin) => {
                let _ = tx.send(format!("(binary) {:?}", bin.len()));
            }
            Payload::String(s) => {
                let _ = tx.send(s);
            }
        });
    }

    let socket = builder.connect().expect("could not connect to test server");
    (socket, receivers)
}


/// Spins up a test server, emits `payload` on `emit_event`, waits for a response
/// on `listen_event`, and parses it into `R`.
pub fn socket_roundtrip<P: Serialize, R: DeserializeOwned>(
    listen_event: &str,
    emit_event: &str,
    payload: &P,
) -> R {
    let (_state, _rt, socket, rx) = setup_test_with_listener(listen_event);

    socket
        .emit(emit_event, serde_json::to_value(payload).unwrap())
        .expect("emit failed");

    let msg = rx
        .recv_timeout(std::time::Duration::from_secs(WAIT_TIME))
        .expect("no response");

    serde_json::from_str(&msg).expect("invalid JSON in response")
}

pub fn emit_and_recv<P: Serialize, R: DeserializeOwned>(
    socket: &Client,
    rx: &Receiver<String>,
    emit_event: &str,
    payload: &P,
) -> R {
    socket
        .emit(emit_event, serde_json::to_value(payload).unwrap())
        .expect("emit failed");

    let msg = rx
        .recv_timeout(std::time::Duration::from_secs(WAIT_TIME))
        .expect("no response");

    serde_json::from_str(&msg).expect("invalid JSON in response")
}

pub fn recv<R: DeserializeOwned>(rx: &Receiver<String>) -> R {
    let msg = rx.recv_timeout(Duration::from_secs(WAIT_TIME)).expect("no response");
    serde_json::from_str(&msg).expect("invalid JSON in response")
}

pub fn create_lobby(socket: &Client, rx: &Receiver<String>, username: &str) {
    let data = LobbyPayload {
        username: Some(username.to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    socket.emit("lobbyControl", serde_json::to_value(&data).unwrap()).expect("emit failed");
}

pub fn join_lobby(socket: &Client, username: &str, lobby_id: u32) {
    let data = LobbyPayload {
        username: Some(username.to_string()),
        event_type: LobbyEvents::JoinLobby,
        lobby_id: Some(lobby_id),
    };
    socket.emit("lobbyControl", serde_json::to_value(&data).unwrap()).expect("emit failed");
}

pub fn get_lobby_id(state: &Arc<Mutex<ServerState>>) -> u32 {
    std::thread::sleep(Duration::from_millis(1000));
    let state = state.lock().unwrap();
    state.player_lobby_map.values().next().cloned().unwrap()
}

pub fn assert_lobby_state(state: &Arc<Mutex<ServerState>>, lobby_id: u32, expected_player_names: &[&str]) {
    let state = state.lock().unwrap();
    assert_eq!(state.player_lobby_map.len(), expected_player_names.len());
    assert!(state.game_map.contains_key(&lobby_id));

    let lobby_arc = state.game_map.get(&lobby_id).unwrap();
    let lobby = lobby_arc.lock().unwrap();
    let GameLogic::Lobby(ref lobby) = *lobby else {
        panic!("Expected LobbyLogic variant");
    };

    assert!(lobby.players.iter().any(|p| p.id == lobby.host));

    let actual_names: HashSet<&str> = lobby.get_players().get_all().iter().map(|p| p.name.as_str()).collect();
    let expected_names: HashSet<&str> = expected_player_names.iter().copied().collect();

    assert_eq!(actual_names, expected_names);
    assert_eq!(lobby.get_game_id(), lobby_id);
}


/// Polls `state` until `predicate` returns true, or panics after a timeout.
/// Use this only when there's no event/broadcast to synchronize on (e.g. a
/// disconnect that has no one left in the room to notify).
pub fn wait_until<F>(state: &Arc<Mutex<ServerState>>, predicate: F)
where
    F: Fn(&ServerState) -> bool,
{
    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    loop {
        {
            let guard = state.lock().unwrap();
            if predicate(&guard) {
                return;
            }
        }
        if std::time::Instant::now() >= deadline {
            panic!("condition was never met within timeout");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}