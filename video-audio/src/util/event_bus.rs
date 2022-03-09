use chrono::Timelike;
use tokio::sync::broadcast;
pub struct EventBus<E> {
    label: String,
    tx: broadcast::Sender<E>,
}

impl<E> EventBus<E>
where
    E: Clone,
{
    pub fn new_with_label(label: String) -> Self {
        let (tx, rx) = broadcast::channel(1);
        Self { label, tx: tx }
    }

    pub fn register_receive(&self) -> broadcast::Receiver<E> {
        let mut rx;
        rx = self.tx.subscribe();
        rx
    }

    pub async fn publish(&self, e: E) {
        self.tx.send(e);
    }
}
