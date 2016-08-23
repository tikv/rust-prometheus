// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate prom;
extern crate hyper;

use std::thread;
use std::time::Duration;

use hyper::{StatusCode, Decoder, Next, Encoder as HyperEncoder};
use hyper::header::{ContentLength, ContentType};
use hyper::net::HttpStream;
use hyper::server::{Server, Handler, Request, Response};
use hyper::mime::Mime;

use prom::errors::{Error, Result};
use prom::encoder::{Encoder, TextEncoder};
use prom::{Counter, Opts, Registry};

fn main() {
    let opts = Opts::new("test", "test help").const_label("a", "1").const_label("b", "2");
    let counter = Counter::with_opts(opts).unwrap();

    let r = Registry::new();
    r.register(Box::new(counter.clone())).unwrap();

    counter.inc();
    assert_eq!(counter.get() as u64, 1);
    counter.inc_by(42.0).unwrap();
    assert_eq!(counter.get() as u64, 43);

    let c2 = counter.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(300));
            c2.inc();
        }
    });

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1500));
            counter.inc();
        }
    });

    let encoder = TextEncoder::new();
    // Http server
    run("127.0.0.1:9898", &r, &encoder).unwrap();
}

// run runs a http server with a Registry and a Encoder, it blocks current thread.
pub fn run<'a>(addr: &str, reg: &Registry, encoder: &'a Encoder) -> Result<()> {
    let reg = reg.clone();

    let addr = try!(addr.parse().or_else(|e| Err(Error::Msg(format!("{:?}", e)))));
    let server = try!(Server::http(&addr).or_else(|e| Err(Error::Msg(format!("{:?}", e)))));
    if let Ok((listener, server)) = server.handle(|_| HttpHandler::new(reg.clone(), encoder)) {
        println!("listening {}", listener);

        server.run();
    }

    Err(Error::Msg("http server error".to_owned()))
}

pub struct HttpHandler<'a> {
    registry: Registry,
    encoder: &'a (Encoder + 'a),
    buffer: Vec<u8>,
    write_pos: usize,
}

impl<'a> HttpHandler<'a> {
    pub fn new(registry: Registry, encoder: &'a Encoder) -> HttpHandler<'a> {
        HttpHandler {
            registry: registry,
            encoder: encoder,
            buffer: Vec::new(),
            write_pos: 0,
        }
    }
}

impl<'a> Handler<HttpStream> for HttpHandler<'a> {
    fn on_request(&mut self, _: Request<HttpStream>) -> Next {
        // TODO: route requests
        Next::write()
    }

    fn on_request_readable(&mut self, _: &mut Decoder<HttpStream>) -> Next {
        Next::write()
    }

    fn on_response(&mut self, res: &mut Response) -> Next {
        let metric_familys = self.registry.gather();
        if let Ok(_) = self.encoder.encode(&metric_familys, &mut self.buffer) {
            res.headers_mut().set(ContentLength(self.buffer.len() as u64));
        } else {
            return Next::remove();
        }

        self.write_pos = 0;
        res.set_status(StatusCode::Ok);
        res.headers_mut().set(ContentType((&self.encoder.format_type()).parse::<Mime>().unwrap()));
        Next::write()
    }

    fn on_response_writable(&mut self, encoder: &mut HyperEncoder<HttpStream>) -> Next {
        match encoder.try_write(&self.buffer[self.write_pos..]) {
            Ok(Some(n)) => {
                if (self.write_pos + n) == self.buffer.len() {
                    Next::end()
                } else {
                    // a partial write
                    self.write_pos += n;
                    Next::write()
                }
            }
            Ok(None) => Next::write(),
            Err(_) => Next::remove(),
        }
    }
}
