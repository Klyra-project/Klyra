use std::{thread::sleep, time::Duration};

use klyra_service::Service;

struct SleepService {
    duration: u64,
}

#[klyra_service::main]
async fn simple() -> Result<SleepService, klyra_service::Error> {
    Ok(SleepService { duration: 10 })
}

#[klyra_service::async_trait]
impl Service for SleepService {
    async fn bind(
        mut self: Box<Self>,
        _: std::net::SocketAddr,
    ) -> Result<(), klyra_service::error::Error> {
        let duration = Duration::from_secs(self.duration);

        sleep(duration);
        Ok(())
    }
}
