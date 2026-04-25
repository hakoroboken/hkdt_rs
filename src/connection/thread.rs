use tokio::sync::mpsc::{self, Receiver, Sender};

pub type MessageSender<T> = Sender<T>;
pub type MessageReceiver<T> = Receiver<T>;

pub struct ThreadConnection<T> {
    sender: MessageSender<T>,
    receiver: MessageReceiver<T>,
}

impl<T> ThreadConnection<T> {
    pub fn new() -> ThreadConnection<T> {
        let (tx, rx) = mpsc::channel::<T>(100);
        ThreadConnection {
            sender: tx,
            receiver: rx,
        }
    }

    pub fn get_sender(&mut self) -> MessageSender<T> {
        return self.sender.clone();
    }

    pub async fn recv_data(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}
