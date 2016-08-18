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

use std::io::{self, Write};

use hyper::{StatusCode, Decoder, Next, Encoder as HyperEncoder};
use hyper::header::{ContentLength, ContentType};
use hyper::net::HttpStream;
use hyper::server::{Server, Handler, Request, Response};
use hyper::mime::Mime;

use registry::Registry;
use errors::{Result, Error};
use encoder::{Encoder, TextEncoder, TEXT_FORMAT};

// run_with_registry runs a http server with a Registry, it blocks current thread.
pub fn run_with_registry(addr: &str, reg: &Registry) -> Result<()> {
    run(addr, reg, &TextEncoder {})
}

// run runs a http server with a Registry and a Encoder, it blocks current thread.
pub fn run<'a>(addr: &str, reg: &Registry, encoder: &'a Encoder) -> Result<()> {
    let reg = reg.clone();

    let addr = try!(addr.parse().or_else(|e| Err(Error::Msg(format!("{:?}", e)))));
    let server = try!(Server::http(&addr).or_else(|e| Err(Error::Msg(format!("{:?}", e)))));
    if let Ok((_, server)) = server.handle(|_| HttpHandler::new(reg.clone(), encoder)) {
        server.run();
    }

    Err(Error::Msg("http server error".to_owned()))
}

struct Buffer(Vec<u8>);

impl Write for Buffer {
    fn write(&mut self, v: &[u8]) -> io::Result<usize> {
        self.0.extend_from_slice(v);
        Ok(v.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct HttpHandler<'a> {
    registry: Registry,
    encoder: &'a (Encoder + 'a),
    buffer: Buffer,
    write_pos: usize,
}

impl<'a> HttpHandler<'a> {
    pub fn new(registry: Registry, encoder: &'a Encoder) -> HttpHandler<'a> {
        HttpHandler {
            registry: registry,
            encoder: encoder,
            buffer: Buffer(Vec::new()),
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
        if let Ok(written) = scarp(&self.registry, &mut self.buffer, self.encoder) {
            res.headers_mut().set(ContentLength(written as u64));
        } else {
            return Next::remove();
        }

        self.write_pos = 0;
        res.set_status(StatusCode::Ok);
        res.headers_mut().set(ContentType(TEXT_FORMAT.parse::<Mime>().unwrap()));
        Next::write()
    }

    fn on_response_writable(&mut self, encoder: &mut HyperEncoder<HttpStream>) -> Next {
        match encoder.try_write(&self.buffer.0[self.write_pos..]) {
            Ok(Some(n)) => {
                if n == self.buffer.0.len() {
                    Next::end()
                } else {
                    // a partial write
                    self.write_pos = n;
                    Next::write()
                }
            }
            Ok(None) => Next::write(),
            Err(_) => Next::remove(),
        }
    }
}

fn scarp(registry: &Registry, writer: &mut Write, encoder: &Encoder) -> Result<usize> {
    let core = registry.get_core();
    let mut written = 0;

    for collector in core.colloctors_by_id.values() {
        let metric_family = collector.collect();
        written += try!(encoder.encode(&metric_family, writer));
    }

    Ok(written)
}
