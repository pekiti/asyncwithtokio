pub async fn sleep(ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}

pub async fn sleep_for_1_second() {
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
}

pub async fn sleep_for_2_seconds() {
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
}