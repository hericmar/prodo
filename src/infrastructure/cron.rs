use crate::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait CronJob {
    async fn run(&self) -> Result<()>;
}
