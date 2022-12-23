use sqlx::{PgConnection, Connection};
use octantis::configuration::get_configuration;

#[tokio::test]
async fn status_works() {
    // Arrange
    let address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{address}/status"))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn create_customer_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    // Act
    let json = serde_json::json!({
        "name": "test_customer"
    });
    let response = client
        .post(&format!("{app_address}/customers"))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name FROM customer",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved customer.");
    assert_eq!(saved.name, "test_customer");
}

#[tokio::test]
async fn create_customer_returns_a_422_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        (serde_json::json!({"bad_key": "bad_value"}), "missing the name"),
        (serde_json::json!(""), "missing an object"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/customers", &app_address))
            .json(&invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            422,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 422 Bad Request when the payload was {}.",
            error_message
        );
    }
}

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("0.0.0.0:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();
    let server = octantis::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://0.0.0.0:{port}")
}
