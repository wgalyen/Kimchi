use crate::extract::{self, extract_links};
use anyhow::Result;
use extract::Uri;
use glob::glob;
use reqwest::Url;
use std::{collections::HashSet, fs};

pub(crate) async fn collect_links(inputs: Vec<String>) -> Result<HashSet<Uri>> {
    let mut links = HashSet::new();

    for input in inputs {
        match Url::parse(&input) {
            Ok(url) => {
                let res = reqwest::get(url).await?;
                let content = res.text().await?;
                links.extend(extract_links(&content));
            }
            Err(_) => {
                // Assume we received a single file or a glob on our hands
                for entry in glob(&input)? {
                    match entry {
                        Ok(path) => {
                            let content = fs::read_to_string(path)?;
                            links.extend(extract_links(&content));
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }
            }
        };
    }
    Ok(links)
}
