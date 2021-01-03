// `actix_rt::test` is the testing equivalent of `actix_rt::main`.
// It also spares you from having to specify the `#[test]` attribute.
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // We brought `reqwest` in as a _development_ dependency
    // to perform HTTP requests against our application.
    // Either add it manually under [dev-dependencies] in Cargo.toml
    // or run `cargo add reqwest --dev`
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health-check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
fn spawn_app() {
    let server = alemanes::run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}
