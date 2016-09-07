// Copyright 2014 The Prometheus Authors
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

use std::str::FromStr;
use std::collections::HashMap;

use hyper::client::Client;
use hyper::client::pool::Config;
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper::header::{Headers, ContentType};

use proto;
use errors::{Result, Error};
use encoder::{Encoder, TextEncoder};

lazy_static!{
    static ref HTTP_CLIENT: Client = Client::with_pool_config(
            Config{
                max_idle: 1,
            }
        );
}

const CONTENT_TYPE_HEADER: &'static str = "Content-Type";
const CONTENT_LENGTH_HEADER: &'static str = "Content-Length";

pub fn push_from_gather(job: &str,
                        grouping: HashMap<String, String>,
                        url: &str,
                        mfs: Vec<proto::MetricFamily>)
                        -> Result<()> {
    push(job, grouping, url, mfs, "PUT")
}

fn push(job: &str,
        grouping: HashMap<String, String>,
        url: &str,
        mfs: Vec<proto::MetricFamily>,
        method: &str)
        -> Result<()> {

    let mut push_url = if url.contains("://") {
        url.to_owned()
    } else {
        "http://".to_owned() + url
    };

    if push_url.ends_with('/') {
        push_url.pop();
    }

    let mut url_components = Vec::new();
    if job.contains('/') {
        return Err(Error::Msg(format!("job contains '/': {}", job)));
    }

    // TODO: escape job
    url_components.push(job.to_owned());

    for (ln, lv) in grouping {
        // TODO: check label name
        if lv.contains('/') {
            return Err(Error::Msg(format!("value of grouping label {} contains '/': {}", ln, lv)));
        }
        url_components.push(ln.to_owned());
        url_components.push(lv.to_owned());
    }

    push_url = format!("{}/metrics/job/{}", push_url, url_components.join("/"));

    let encoder = TextEncoder::new();
    let mut buf = Vec::new();
    try!(encoder.encode(&mfs, &mut buf));

    let mut request = HTTP_CLIENT.request(Method::from_str(method).unwrap(), &push_url);
    request = request.body(buf.as_slice());

    let mut headers = Headers::new();
    headers.set(ContentType(encoder.format_type().parse().unwrap()));
    request = request.headers(headers);

    let response = try!(request.send().map_err(|e| Error::Msg(format!("{}", e))));

    if response.status == StatusCode::Accepted {
        Ok(())
    } else {
        Err(Error::Msg(format!("unexpected status code {} while pushing to {}",
                               response.status,
                               push_url)))
    }
}
