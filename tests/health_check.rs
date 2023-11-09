//! tests/health_check.rs

use std::future::{self, Future};

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
use zero2prod_antonio::{run, LOCAL_HOST};
#[tokio::test]
async fn health_check_works() {
    // Arrange
    panic!("WAITING REFACTOR BECAUSE IT ISN'T RUNNING!!");
    spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    println!("asdasd");
    let response = client
        .get(format!("http://{LOCAL_HOST}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    eprintln!("Response:\n\n\n\n{response:?}");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
async fn spawn_app() -> impl Future {
    eprintln!("Response:\n\n\n\n");
    let ret = run();
    eprintln!("Otro    :\n\n\n\n");
    ret
}
