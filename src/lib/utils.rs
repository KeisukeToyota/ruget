use reqwest::{blocking::Client, header::ACCEPT_RANGES};

pub fn get_file_size(b: f32) -> String {
    format!("{:.2} MB", b / 1000000.0)
}

pub fn is_accept_ranges(url: &str) -> bool {
    let client = Client::new();
    let res = client.head(url).send().expect("head failed...");
    match res.headers().get(ACCEPT_RANGES) {
        Some(res) => !matches!(res.to_str().unwrap(), "none"),
        None => false,
    }
}

pub trait Download {
    fn download(&self);
}
