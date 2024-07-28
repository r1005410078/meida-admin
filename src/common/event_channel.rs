use std::{fmt::Debug, sync::Arc};
use tokio::sync::mpsc;

pub type EventSender<T> = Arc<mpsc::Sender<T>>;

#[async_trait::async_trait]
pub trait EventHandler<T>
where
    Self: Send + 'static,
{
    async fn handle(&self, event: T) -> anyhow::Result<()>;
}

pub struct EventChannel<T> {
    pub sender: mpsc::Sender<T>,
}

impl<T: Send + Debug + 'static> EventChannel<T> {
    pub fn new<E: EventHandler<T>>(ev: E) -> Self {
        let (sender, mut rx) = mpsc::channel::<T>(100);

        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                if let Err(err) = ev.handle(event).await {
                    log::error!("事件处理失败: {:?}", err);
                }
            }
        });

        Self { sender }
    }
}
