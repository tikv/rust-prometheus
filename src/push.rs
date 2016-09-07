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
use hyper::header::ContentType;

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

/// `push_from_gather` triggers a metric collection by the provided Gatherer (which is
/// usually implemented by a prometheus.Registry) and pushes all gathered metrics
/// to the Pushgateway specified by url, using the provided job name and the
/// (optional) further grouping labels (the grouping map may be nil). See the
/// Pushgateway documentation for detailed implications of the job and other
/// grouping labels. Neither the job name nor any grouping label value may
/// contain a "/". The metrics pushed must not contain a job label of their own
/// nor any of the grouping labels.
///
/// You can use just host:port or ip:port as url, in which case 'http://' is
/// added automatically. You can also include the schema in the URL. However, do
/// not include the '/metrics/jobs/...' part.
///
/// Note that all previously pushed metrics with the same job and other grouping
/// labels will be replaced with the metrics pushed by this call. (It uses HTTP
/// method 'PUT' to push to the Pushgateway.)
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

    let request = HTTP_CLIENT.request(Method::from_str(method).unwrap(), &push_url)
        .header(ContentType(encoder.format_type().parse().unwrap()))
        .body(buf.as_slice());

    let response = try!(request.send().map_err(|e| Error::Msg(format!("{}", e))));
    if response.status == StatusCode::Accepted {
        Ok(())
    } else {
        Err(Error::Msg(format!("unexpected status code {} while pushing to {}",
                               response.status,
                               push_url)))
    }
}
