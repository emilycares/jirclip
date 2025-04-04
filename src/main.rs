#![windows_subsystem = "windows"]

#[tokio::main]
async fn main() {
    jirclip::run().await;
}
