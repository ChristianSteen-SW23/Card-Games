use rust_socketio::{client::Client, ClientBuilder, Payload};
use std::sync::{Arc, Mutex, mpsc::{self, Receiver}};
use tokio::runtime::Runtime;
use std::time::Duration;
use server_rust::{objects::{states::ServerState, Game7Logic, GameLogic}, responses::LobbyResponse, run_test_server, socket::{lobby_socket::LobbyEvents, ErrorResponse, LobbyPayload}};          // adjust if needed

/// Starts a test server and returns the shared state, runtime, socket, and receiver.
pub fn setup_test_with_listener(event_name: &str) -> (Arc<Mutex<ServerState>>, Runtime, Client, Receiver<String>) {
    // --- Start server ---
    let state: Arc<Mutex<ServerState>> = Arc::new(Mutex::new(ServerState::new()));
    let rt = Runtime::new().unwrap();

    {
        let state_clone = state.clone();
        rt.spawn(async move {
            run_test_server("127.0.0.1:4001", state_clone).await;
        });
    }

    // Give the server a moment to start
    std::thread::sleep(Duration::from_millis(300));

    // --- Setup Socket.IO client ---
    let (tx, rx) = mpsc::channel::<String>();
    let event = event_name.to_string();

    let tx_clone = tx.clone();
    let socket = rust_socketio::ClientBuilder::new("http://127.0.0.1:4001")
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

pub fn start_server_and_state() -> (Arc<Mutex<ServerState>>, tokio::runtime::Runtime) {
    let state = Arc::new(Mutex::new(ServerState::new()));
    let rt = tokio::runtime::Runtime::new().unwrap();

    {
        let state = state.clone();
        rt.spawn(async move {
            run_test_server("127.0.0.1:4001", state).await;
        });
    }

    std::thread::sleep(std::time::Duration::from_millis(300));
    (state, rt)
}

/// Connects to a Socket.IO server and listens for a given event name.
/// Returns both the connected client and the receiver channel for incoming messages.
pub fn connect_with_listener(event_name: &str) -> (Client, Receiver<String>) {
    let (tx, rx) = mpsc::channel::<String>();

    let tx_clone = tx.clone();
    let socket = ClientBuilder::new("http://127.0.0.1:4001")
        .on(event_name, move |payload, _| {
            println!("Got payload for event '': {:?}", payload);
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