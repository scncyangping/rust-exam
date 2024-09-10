use tokio::sync::mpsc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionHandleCommand {
    Close,
}

pub struct SSHSessionHandle {
    sender: mpsc::UnboundedSender<SessionHandleCommand>,
}

impl SSHSessionHandle {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<SessionHandleCommand>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (SSHSessionHandle { sender }, receiver)
    }
}

impl SessionHandle for SSHSessionHandle {
    fn close(&mut self) {
        let _ = self.sender.send(SessionHandleCommand::Close);
    }
}

pub trait SessionHandle {
    fn close(&mut self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bl() {
        let tx = Some(1);
        if let Some(tx) = tx {
            println!("{tx}")
        }
        println!("{:?}", tx)
    }
}
