use std::time::Duration;

use klyra_service::Service;
use tokio::time::sleep;

struct SleepService {
    duration: u64,
}

#[klyra_service::main]
async fn simple() -> Result<SleepService, klyra_service::Error> {
    Ok(SleepService { duration: 4 })
}

#[klyra_service::async_trait]
impl Service for SleepService {
    async fn bind(mut self, _: std::net::SocketAddr) -> Result<(), klyra_service::error::Error> {
        let duration = Duration::from_secs(self.duration);

        sleep(duration).await;
        Ok(())
    }
}
