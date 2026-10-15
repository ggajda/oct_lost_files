use tokio::time::{Duration, sleep};

pub async fn auto_counter() {
    for i in 1..10 {
        println!("Counter: {}", i);
        sleep(Duration::from_secs(1)).await;
    }
}
