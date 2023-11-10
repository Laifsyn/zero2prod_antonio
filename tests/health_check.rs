//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
use quote::quote;
use zero2prod_antonio::{bind_port, LOCAL_HOST_IP};
#[tokio::test]
async fn health_check_works() {
    let host_address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{host_address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length(), "Response wasn't empty!");
}

fn spawn_app() -> String {
    use std::format as f;
    let listener = bind_port(f!("{LOCAL_HOST_IP}:0"));
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod_antonio::run(listener).expect("Failed to bind Address");
    tokio::spawn(server);
    f!("http://{LOCAL_HOST_IP}:{port}")
}

#[tokio::test]
async fn subscribe_return_ok_200() {
    let host_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.comm&other=dfghjkl";
    println!("{}/subscriptions", &host_address);
    let response = client
        .post(format!("{}/subscriptions", &host_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap_or_else(|error| panic!("Failed to execute request \"{error}\""));

    assert_eq!(
        200,
        response.status().as_u16(),
        "Un-Successful post with payload {body}"
    )
}
#[tokio::test]
async fn subscribe_return_bad_request_400() {
    // Arrange
    let host_address = spawn_app();
    let client = reqwest::Client::new();
    // Act
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("name=le%20guin&other_param=hello_world", "extra fields"),
        (
            "nmes=le%20s&other_par=hello_world",
            "only unexpected fields",
        ),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", host_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .unwrap_or_else(|_| panic!("\nFailed to execute request for {error_message}"));

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            quote! {invalid_body}
        )
    }
}
