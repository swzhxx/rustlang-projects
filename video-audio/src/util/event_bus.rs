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
        let (tx, _rx) = broadcast::channel(16);
        Self { label, tx: tx }
    }

    pub fn register_receive(&self) -> broadcast::Receiver<E> {
        let rx;
        rx = self.tx.subscribe();
        rx
    }

    pub async fn publish(&self, e: E) {
        match self.tx.send(e) {
            Ok(_) => {}
            Err(err) => {
                log::error!("[PUBLISH ERROR] {}", err)
            }
        }
    }
}
