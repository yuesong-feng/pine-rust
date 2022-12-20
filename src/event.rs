use std::{collections::HashMap, error::Error, io, time::Duration};

use mio::{
    event::{self, Event},
    Events, Interest, Poll, Registry, Token,
};

#[derive(Debug)]
pub struct EventLoop {
    poll: Poll,
}
impl EventLoop {
    pub fn new() -> io::Result<EventLoop> {
        let poll = Poll::new()?;
        Ok(EventLoop { poll })
    }
    pub fn run(&self) {

    }
}

pub struct Channel<'a> {
    target: Token,
    evloop: &'a EventLoop,
    levents: Interest,
    revents: Interest,
    read_cb: Box<dyn Fn() + 'a>,
    write_cb: Box<dyn Fn() + 'a>,
}
