use reqwest;
use zero2prod;
use std::net;

// NOTE: This is the only part of this test code that has to do with our app. It could also launch,
//       for example, a Rails app by shelling out.
fn spawn_app() -> String {
    let local_host = "127.0.0.1";
    // Port 0 is special: requesting to bind to port 0 triggers the OS to provide an available port
    // instead.
    // Source: https://www.lifewire.com/port-0-in-tcp-and-udp-818145
    let listener = net::TcpListener::bind(&format!("{}:0", local_host))
        .expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener)
        .expect("Failed to bind to address.");

    // Launch the server as a background task with tokio::spawn, which returns a handle to the
    // spawned future.
    // We're not going to do anything with it so we don't bind it to anything.
    let _ = tokio::spawn(server);
    format!("http://{}:{}", local_host, port)
}

// NOTE: This test code has no knowledge of our app. The same test code could test any app available
//       at the specified address and port.
// actix_rt::test is the test equivalent of actix_rt::main in that it is a procedural macro that moves the body of the function into a future, run by the actix runtime.
// It also adds the `#[test]` attribute for you.
// Verify this by running `$ cargo expand --test health_check`
#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-formurlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customized error message on test failure:
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

