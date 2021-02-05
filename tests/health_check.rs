use reqwest;
use zero2prod;

// NOTE: This is the only part of this test code that has to do with our app. It could also launch,
//       for example, a Rails app by shelling out.
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind to address.");

    // Launch the server as a background task with tokio::spawn, which returns a handle to the
    // spawned future.
    // We're not going to do anything with it so we don't bind it to anything.
    let _ = tokio::spawn(server);
}

// NOTE: This test code has no knowledge of our app. The same test code could test any app available
//       at the specified address and port.
// actix_rt::test is the test equivalent of actix_rt::main in that it is a procedural macro that moves the body of the function into a future, run by the actix runtime.
// It also adds the `#[test]` attribute for you.
// Verify this by running `$ cargo expand --test health_check`
#[actix_rt::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
