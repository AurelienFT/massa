// Copyright (c) 2022 MASSA LABS <info@massa.net>
use crossbeam::channel::{bounded, Receiver, Sender};
use massa_models::config::CHANNEL_SIZE;
use socket2 as _;
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::establisher::{BSConnector, BSListener};

pub fn new() -> (MockEstablisher, MockEstablisherInterface) {
    let (connection_listener_tx, connection_listener_rx) =
        bounded::<(SocketAddr, Sender<TcpStream>)>(CHANNEL_SIZE);

    let (connection_connector_tx, connection_connector_rx) =
        bounded::<(TcpStream, SocketAddr, Arc<AtomicBool>)>(CHANNEL_SIZE);

    (
        MockEstablisher {
            connection_listener_rx: Some(connection_listener_rx),
            connection_connector_tx,
        },
        MockEstablisherInterface {
            connection_listener_tx: Some(connection_listener_tx),
            connection_connector_rx,
        },
    )
}

#[derive(Debug)]
pub struct MockListener {
    connection_listener_rx: crossbeam::channel::Receiver<(SocketAddr, Sender<TcpStream>)>, // (controller, mock)
}

impl BSListener for MockListener {
    fn accept(&mut self) -> std::io::Result<(TcpStream, SocketAddr)> {
        let (_addr, sender) = self.connection_listener_rx.recv().map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "MockListener accept channel from Establisher closed".to_string(),
            )
        })?;
        let duplex_controller = TcpListener::bind("localhost:0").unwrap();
        let duplex_mock = TcpStream::connect(duplex_controller.local_addr().unwrap()).unwrap();
        let duplex_controller = duplex_controller.accept().unwrap();

        // Tokio `from_std` have non-blocking Tcp objects as a requirement
        duplex_mock.set_nonblocking(true).unwrap();

        sender.send(duplex_mock).map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "MockListener accept return \"oneshot\" channel to Establisher closed".to_string(),
            )
        })?;

        Ok(duplex_controller)
    }
}

#[derive(Debug)]
pub struct MockConnector {
    connection_connector_tx: Sender<(TcpStream, SocketAddr, Arc<AtomicBool>)>,
}

impl BSConnector for MockConnector {
    fn connect(&mut self, addr: SocketAddr) -> std::io::Result<TcpStream> {
        let duplex_mock = TcpListener::bind(addr).unwrap();
        let duplex_controller = TcpStream::connect(addr).unwrap();
        let duplex_mock = duplex_mock.accept().unwrap();

        // Used to see if the connection is accepted
        let waker = Arc::new(AtomicBool::from(false));
        let provided_waker = Arc::clone(&waker);

        let sender = self.connection_connector_tx.clone();
        let send = std::thread::spawn(move || {
            // send new connection to mock
            sender
                .send((duplex_mock.0, addr, provided_waker))
                .map_err(|_err| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "MockConnector connect channel to Establisher closed".to_string(),
                    )
                })
                .unwrap();
        });

        while !waker.load(Ordering::Relaxed) {
            std::thread::yield_now();
        }
        send.join().unwrap();
        Ok(duplex_controller)
    }
}

#[derive(Debug)]
pub struct MockEstablisher {
    connection_listener_rx: Option<crossbeam::channel::Receiver<(SocketAddr, Sender<TcpStream>)>>,
    connection_connector_tx: Sender<(TcpStream, SocketAddr, Arc<AtomicBool>)>,
}

impl MockEstablisher {
    pub(crate) fn get_listener(&mut self, _addr: &SocketAddr) -> io::Result<MockListener> {
        Ok(MockListener {
            connection_listener_rx: self
                .connection_listener_rx
                .take()
                .expect("MockEstablisher get_listener called more than once"),
        })
    }

    pub(crate) fn get_connector(&mut self) -> MockConnector {
        // create connector stream

        MockConnector {
            connection_connector_tx: self.connection_connector_tx.clone(),
        }
    }
}

pub struct MockEstablisherInterface {
    connection_listener_tx: Option<crossbeam::channel::Sender<(SocketAddr, Sender<TcpStream>)>>,
    connection_connector_rx: Receiver<(TcpStream, SocketAddr, Arc<AtomicBool>)>,
}

impl MockEstablisherInterface {
    pub async fn connect_to_controller(&self, addr: &SocketAddr) -> io::Result<TcpStream> {
        let sender = self.connection_listener_tx.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "mock connect_to_controller_listener channel not initialized".to_string(),
            )
        })?;
        let (response_tx, response_rx) = bounded::<TcpStream>(1);
        sender.send((*addr, response_tx)).map_err(|_err| {
            io::Error::new(
                io::ErrorKind::Other,
                "mock connect_to_controller_listener channel to listener closed".to_string(),
            )
        })?;
        let duplex_mock = response_rx.recv().map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "MockListener connect_to_controller_listener channel from listener closed"
                    .to_string(),
            )
        })?;
        Ok(duplex_mock)
    }

    pub fn wait_connection_attempt_from_controller(
        &mut self,
    ) -> io::Result<(TcpStream, SocketAddr, Arc<AtomicBool>)> {
        self.connection_connector_rx.recv().map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "MockListener get_connect_stream channel from connector closed".to_string(),
            )
        })
    }
}
