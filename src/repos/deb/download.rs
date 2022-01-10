use anyhow::Result;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use futures_util::StreamExt;
use std::{path::PathBuf, fs::File, io::Write};

use crate::repos::config::Config;
use super::{package::DebPackage, signatures};
use reqwest;

pub async fn download(config: &Config, pkg: DebPackage, pb: ProgressBar) -> Result<PathBuf> {
    // https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
    let control = pkg.control.clone();
    let url = format!("http://{}", control.filename);    
    let response = reqwest::get(&url).await?;
    let size = response.content_length().unwrap_or_default();

    // let content = response.bytes().await?;
    // let mut content: &[u8] = content.as_ref();

    println!("Get: {} {} {} {} [{}]", url, control.architecture,
    control.package, control.version, HumanBytes(size));

    // let pb = ProgressBar::new(control.size.parse::<u64>()?);
    pb.set_style(ProgressStyle::default_bar()
    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
    .progress_chars("#>-"));
    let name = control.filename.split('/').last().unwrap().to_string();
    let mut content = Vec::with_capacity(size as usize);
    let fname = format!("{}/{}", config.tmp, name);

    let (mut stream, mut downloaded) = (response.bytes_stream(), 0_u64);

    while let Some(item) = stream.next().await {
        let chunk = item?;
        Write::write(&mut content, &chunk)?;
        let progress = std::cmp::min(downloaded + chunk.len() as u64, size);
        downloaded = progress;
        pb.set_position(progress);
    }

    match signatures::verify_sig(&pkg, &content) {
        Ok(()) => (),
        Err(e) => eprintln!("Failed to check package signature :: {}", e)
    }

    let mut pkg = File::create(&fname)?;
    pkg.write_all(&content)?;

    Ok(PathBuf::from(&fname))
}