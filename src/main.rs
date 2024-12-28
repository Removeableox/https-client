mod https;
use https::HttpsClient;

fn main() {
    let url = String::from("www.youtube.com");
    let https_client = HttpsClient::new(&url);
}
