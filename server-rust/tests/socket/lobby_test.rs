use rust_socketio::{ClientBuilder, Payload};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use server_rust::{run_test_server, state::ServerState};

fn wait_for_server(addr: &str, timeout_ms: u64) {
    use std::net::TcpStream;
    let start = std::time::Instant::now();
    while start.elapsed().as_millis() < timeout_ms as u128 {
        if TcpStream::connect(addr).is_ok() {
            return; // server is ready
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    panic!("Server did not start within {timeout_ms}ms");
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
    .on("message-back", move |payload, _| {
        match payload {
            rust_socketio::Payload::Text(values) => {
                if let Some(v) = values.get(0).and_then(|v| v.as_str()) {
                    tx.send(v.to_string()).unwrap();
                }
            }
            _ => {}
        }
    })
    .connect()
    .expect("could not connect");


    socket.emit("message", "").expect("emit failed");

    let msg = rx.recv_timeout(std::time::Duration::from_secs(2)).expect("no response");
    assert_eq!(msg, "Hello World!");
}


#[test]
fn sanity_check() {
    assert!(true);
}
