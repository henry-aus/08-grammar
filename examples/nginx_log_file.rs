use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use polars::prelude::*;

use anyhow::{anyhow, Result};
use grammar::{parse_nginx_log, HttpMethod, HttpProto, NginxLog};

fn main() -> Result<()> {
    let file = File::open("./fixtures/ngnix_log").unwrap();
    let reader = BufReader::new(file);
    let mut logs: Vec<NginxLog> = Vec::new();
    for line in reader.lines() {
        if let Ok(input) = line {
            let log = parse_nginx_log(input.as_str())
                .map_err(|e| anyhow!("Failed to parse log: {:?}", e))?;
            logs.push(log);
        }
    }

    let mut df = DataFrame::new(vec![
        Series::new(
            "addr",
            logs.iter()
                .map(|p| p.addr.clone().to_string())
                .collect_vec(),
        ),
        Series::new(
            "datetime",
            logs.iter()
                .map(|p| p.datetime.clone().to_string())
                .collect_vec(),
        ),
        Series::new(
            "method",
            logs.iter()
                .map(|p| match p.method {
                    HttpMethod::Get => "GET",
                    HttpMethod::Post => "POST",
                    HttpMethod::Put => "PUT",
                    HttpMethod::Delete => "DELETE",
                    HttpMethod::Head => "HEAD",
                    HttpMethod::Options => "OPTIONS",
                    HttpMethod::Connect => "CONNECT",
                    HttpMethod::Trace => "TRACE",
                    HttpMethod::Patch => "PATCH",
                })
                .collect_vec(),
        ),
        Series::new("url", logs.iter().map(|p| p.url.clone()).collect_vec()),
        Series::new(
            "protocol",
            logs.iter()
                .map(|p| match p.protocol {
                    HttpProto::HTTP1_0 => "HTTP1_0",
                    HttpProto::HTTP1_1 => "HTTP1_1",
                    HttpProto::HTTP2_0 => "HTTP2_0",
                    HttpProto::HTTP3_0 => "HTTP3_0",
                })
                .collect_vec(),
        ),
        Series::new(
            "status",
            logs.iter()
                .map(|p| p.status.clone().to_string())
                .collect_vec(),
        ),
        Series::new(
            "body_bytes",
            logs.iter().map(|p| p.body_bytes.clone()).collect_vec(),
        ),
        Series::new(
            "referer",
            logs.iter().map(|p| p.referer.clone()).collect_vec(),
        ),
        Series::new(
            "user_agent",
            logs.iter().map(|p| p.user_agent.clone()).collect_vec(),
        ),
    ])?;

    let mut file = std::fs::File::create("./fixtures/ngnix_log.parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();

    Ok(())
}
