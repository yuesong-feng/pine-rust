use std::{collections::HashMap, sync::Arc, thread};

use mio::net::{TcpListener, TcpStream};

use crate::{EventLoop, ThreadPool};

#[derive(Debug)]
pub struct Acceptor {
    id: usize,
    listener: TcpListener,
}
impl Acceptor {
    pub fn new(listener: TcpListener) -> Acceptor {
        Acceptor { id: 0, listener }
    }
}

#[derive(Debug)]
pub struct Connection {
    id: usize,
    stream: TcpStream,
}

#[derive(Debug)]
pub struct TcpServer {
    main_reactor: EventLoop,
    sub_reactors: Vec<EventLoop>,
    acceptor: Acceptor,
    connections: HashMap<u32, Connection>,
    thread_pool: ThreadPool,
}

impl TcpServer {
    pub fn new(addr: &str) -> Result<TcpServer, Box<dyn std::error::Error>> {
        let main_reactor = EventLoop::new()?;

        let mut sub_reactors = vec![];
        for _ in 0..8 {
            sub_reactors.push(EventLoop::new()?);
        }

        let addr = addr.parse()?;
        let listener = TcpListener::bind(addr)?;

        let acceptor = Acceptor::new(listener);

        let connections = HashMap::new();

        let thread_pool = ThreadPool::new(8);

        Ok(TcpServer {
            main_reactor,
            sub_reactors,
            acceptor,
            connections,
            thread_pool,
        })
    }

    pub fn start(&'static self) -> Result<(), Box<dyn std::error::Error>> {
        for each in &self.sub_reactors {
            let sub_reactor = Arc::new(each);
            self.thread_pool.execute(move || {
                sub_reactor.run();
            });
        }
        let main_reactor = Arc::new(&self.main_reactor);
        self.thread_pool.execute(move || {
            main_reactor.run();
        });
        Ok(())
    }
}
