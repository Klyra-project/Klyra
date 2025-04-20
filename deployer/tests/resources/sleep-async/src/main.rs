use std::time::Duration;

use klyra_runtime::Service;
use tokio::time::sleep;

struct SleepService {
    duration: u64,
}

#[klyra_runtime::main]
async fn simple() -> Result<SleepService, klyra_runtime::Error> {
    Ok(SleepService { duration: 4 })
}

#[klyra_runtime::async_trait]
impl Service for SleepService {
    async fn bind(mut self, _: std::net::SocketAddr) -> Result<(), klyra_runtime::Error> {
        let duration = Duration::from_secs(self.duration);

        sleep(duration).await;
        Ok(())
    }
}
