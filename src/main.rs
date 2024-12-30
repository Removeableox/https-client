mod https;
use https::client::HttpsClient;

fn main() {
    let url = String::from("www.youtube.com");
    let https_client = HttpsClient::new(&url);
}
