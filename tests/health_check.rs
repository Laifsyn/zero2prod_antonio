//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
use zero2prod_antonio::LOCAL_HOST;
#[tokio::test]
async fn health_check_works() {
    spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{LOCAL_HOST}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    eprintln!("Testing Server alive");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length(), "Response wasn't empty!");
}

// Launch our application in the background ~somehow~
fn spawn_app() {
    let server = zero2prod_antonio::run().expect("Failed to bind Address");

    tokio::spawn(server);
}
