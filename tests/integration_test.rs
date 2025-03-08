use http::core::run;
use reqwest;
use tokio;

#[tokio::test]
async fn test_request() {
    run("127.0.0.1:8080").await;
    let _response = reqwest::get("localhost:8080")
        .await
        .expect("failed to send request");
    assert!(true == true);
}
