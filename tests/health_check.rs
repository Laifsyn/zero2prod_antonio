//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
use quote::quote;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod_antonio::{
    bind_port,
    configuration::{get_configuration, Settings},
    generate_db_pool,
};
extern crate dotenv;

pub struct TestApp {
    // The http URL to query the server
    pub host_address: String,
    pub db_pool: PgPool,
}
use dotenv::dotenv;
#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", test_app.host_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length(), "Response wasn't empty!");
}

async fn configure_database() -> PgPool {
    let mut config = get_configuration().expect("Couldn't load config files.");
    config.database.database_name = "test_db".to_string();
    let connection: Result<PgConnection, sqlx::Error> =
        PgConnection::connect(&config.database.connection_string()).await;
    if let Err(connection_error) = connection {
        match connection_error {
            sqlx::Error::Database(database_error) => match database_error.code().as_deref() {
                Some("3D000") => {
                    create_db(&config).await;
                }
                _ => panic!("Database Error: {database_error:#?}\n"),
            },
            _ => panic!(
                "\nCouldn't connect to \"{}\" database\nError:{connection_error:#?}<End\n\n",
                config.database.database_name
            ),
        }
    }

    generate_db_pool(config).await // create a connection with the newly created database
}
async fn create_db(config: &Settings) {
    let mut connection = PgConnection::connect(&config.database.connection_string_without_db())
        .await
        .unwrap_or_else(|e| {
            panic!(
                "Failed to set connection to \"{}\"\n{e:?}",
                config.database.connection_string_without_db()
            )
        });
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database.database_name).as_str())
        .await
        .unwrap_or_else(|e| panic!("Failed to create database {e:?}"));
}
async fn spawn_app() -> TestApp {
    let connection_pool = configure_database().await;

    use std::format as f;
    use zero2prod_antonio::LOCAL_HOST_IP;
    let listener = bind_port(f!("{LOCAL_HOST_IP}:0")); // The IP where the HTTP Server will be listening from
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod_antonio::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind Address");
    tokio::spawn(server);
    TestApp {
        host_address: f!("http://{LOCAL_HOST_IP}:{port}"),
        db_pool: connection_pool,
    }
}

#[tokio::test]
async fn subscribe_return_ok_200_for_valid_data() {
    let _ = dotenv();

    let test_app = spawn_app().await;

    let client = reqwest::Client::new();
    let (name, email) = (
        "le guin",
        format!("ursula_le_guinny{}@gmail.com", Uuid::new_v4()),
    );
    let body = format!("name={name}&email={email}&other=dfghjkl");
    let target_address = format!("{}/subscriptions", test_app.host_address);
    println!("Uploading to: {target_address}");
    let response = client
        .post(target_address)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body.clone())
        .send()
        .await
        .unwrap_or_else(|error| panic!("Failed to execute request \"{error}\""));

    assert_eq!(
        200,
        response.status().as_u16(),
        "Un-Successful post with payload {body}"
    );

    let saved = sqlx::query!(
        "SELECT email, name FROM subscriptions WHERE email = $1",
        email
    )
    .fetch_one(&test_app.db_pool)
    .await
    .unwrap_or_else(|e: sqlx::Error| panic!("Failed to fetch saved subscriptions. \n{e:?}"));
    assert_eq!(saved.email, email);
    assert_eq!(saved.name, name);
}

#[tokio::test]
async fn subscribe_return_bad_request_400_for_invalid_data() {
    let test_app: TestApp = spawn_app().await;
    let client = reqwest::Client::new();
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
            .post(format!("{}/subscriptions", test_app.host_address))
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
