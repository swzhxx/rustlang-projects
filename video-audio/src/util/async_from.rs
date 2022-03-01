use anyhow::Result;
use async_trait::async_trait;
#[async_trait]
pub trait AsyncFrom<T> {
    async fn async_from(_: T) -> Self;
}

#[async_trait]
pub trait TryAsyncFrom<T> {
    async fn try_async_from(_: T) -> Result<Self>
    where
        Self: Sized;
}
