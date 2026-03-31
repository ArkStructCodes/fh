use std::marker::PhantomData;
use std::sync::Arc;

use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::{mpsc, oneshot};

use crate::error::{Error, Result};
use crate::structures::Packet;

// states of `Listener` as concrete types
pub struct Idle;
pub struct Connected;

pub struct Listener<S = Idle> {
    socket: Arc<UdpSocket>,
    _state: PhantomData<S>,
}

impl Listener<Idle> {
    pub async fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(Self {
            socket: Arc::new(socket),
            _state: PhantomData,
        })
    }

    pub async fn listen(&self) -> Result<Listener<Connected>> {
        // allow for size checking with a reasonably high buffer size
        let mut buf = [0; size_of::<Packet>() * 2];
        let (len, addr) = self.socket.recv_from(&mut buf).await?;

        if len != size_of::<Packet>() {
            return Err(Error::Unsupported);
        }

        self.socket.connect(addr).await?;
        Ok(Listener {
            socket: self.socket.clone(),
            _state: PhantomData,
        })
    }
}

#[derive(Debug)]
pub struct StopSignal;

impl Listener<Connected> {
    pub async fn recv(
        &self,
        tx: mpsc::Sender<Packet>,
        stop_rx: oneshot::Receiver<StopSignal>,
    ) -> Result<()> {
        let mut buf = [0; size_of::<Packet>()];
        let socket_ref = self.socket.clone();
        let (task_tx, task_rx) = oneshot::channel();

        tokio::spawn(async move {
            let err = loop {
                let len = match socket_ref.recv(&mut buf).await {
                    Ok(v) => v,
                    Err(e) => break e.into(),
                };

                if len != buf.len() {
                    break Error::Unsupported;
                }

                let Ok(data) = Packet::try_from(&buf) else {
                    break Error::Unsupported;
                };

                if tx.send(data).await.is_err() {
                    break Error::Internal;
                }
            };
            task_tx.send(err).expect("error handling failure");
        });

        tokio::select! {
            // success, received a signal to stop receiving data 
            _ = stop_rx => Ok(()),
            // failure, propagate errors caught in the receive loop
            v = task_rx => match v {
                Ok(err) => Err(err),
                Err(_) => Err(Error::Internal),
            },
        }
    }
}
