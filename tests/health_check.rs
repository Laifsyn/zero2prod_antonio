//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
use zero2prod_antonio::{bind_port, LOCAL_HOST_IP};
#[tokio::test]
async fn health_check_works() {
    let host_address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{host_address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    eprintln!("Testing Server alive");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length(), "Response wasn't empty!");
}

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    use std::format as f;
    let listener = bind_port(f!("{LOCAL_HOST_IP}:0"));
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod_antonio::run(listener).expect("Failed to bind Address");
    tokio::spawn(server);
    f!("{LOCAL_HOST_IP}:{port}")
}
