use http::core::run;
use tokio;

#[tokio::main]
async fn main() {
    run("127.0.0.1:8080").await;
}
