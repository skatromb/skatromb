use itertools::Itertools;
use reqwest::{Url, blocking::Client};
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("bad http response: {0}")]
    BadResponse(String),
}

trait PageLinks {
    fn page_links(&self, base_url: Url) -> Vec<Url>;
}

impl PageLinks for Html {
    fn page_links(&self, base_url: Url) -> Vec<Url> {
        let selector = Selector::parse("a").unwrap();

        let href_values = self
            .select(&selector)
            .filter_map(|element| element.value().attr("href"));

        href_values
            .filter_map(move |href| match base_url.join(href) {
                Ok(link_url) => Some(link_url),
                Err(err) => {
                    println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
                    None
                }
            })
            .collect()
    }
}

fn visit_page(client: &Client, url: &Url) -> Result<Vec<Url>, Error> {
    println!("{:#}", url);
    let response = client.get(url.clone()).send()?;
    if !response.status().is_success() {
        return Err(Error::BadResponse(response.status().to_string()));
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let links = document.page_links(base_url);

    Ok(links)
}

fn process_url(client: Client, url: &Url, checked_urls: Arc<Mutex<HashSet<String>>>) {
    // Нужно зайти на страницу и получить список урлов
    // Убрать из них те, что уже были посещены
    // Создать таску (тред) на обработку оставшихся урлов
    {
        let mut checked_urls = checked_urls.lock().unwrap();
        checked_urls.insert(url.to_string());
    }

    match visit_page(&client, url) {
        Ok(urls) => {
            let urls: Vec<Url> = urls
                .into_iter()
                .unique()
                .filter(|url| !checked_urls.lock().unwrap().contains(url.as_str()))
                .collect();

            let handles: Vec<_> = urls
                .into_iter()
                .map(|url| {
                    let client_clone = client.clone();
                    let checked_clone = Arc::clone(&checked_urls);

                    thread::spawn(move || process_url(client_clone, &url, checked_clone))
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        }
        Err(err) => println!("Could not extract links: {err:#}"),
    }
}

fn main() {
    let client = Client::new();
    let start_url = Url::parse("https://www.google.org").unwrap();
    let checked_urls = Arc::new(Mutex::new(HashSet::new()));

    process_url(client, &start_url, checked_urls)
}
