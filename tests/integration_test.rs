use http::core::run;
use reqwest;
use tokio;

fn spawn_app() {
    let _ = tokio::spawn(run("127.0.0.1:8080"));
}

#[tokio::test]
async fn test_request_async() {
    spawn_app();

    let _response = reqwest::get("http://127.0.0.1:8080")
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get response");

    assert!(true == true);
}
