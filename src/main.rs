use crate::interfaces::cli::run;

#[tokio::main]
async fn main() {
    println!("Starting My Trading Bot...");
    run().await;
}
