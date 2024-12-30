use std::net::TcpStream;
use crate::https::crypto::key_pair;

pub struct HttpsClient {
    stream: TcpStream
}

impl HttpsClient {
    pub fn new(url: &String) -> HttpsClient {
        let stream = TcpStream::connect(url).unwrap();
        let client = HttpsClient {stream};
        
        client.send_client_hello();
        client.get_server_hello();
        client.make_handshake_keys();
        client.parse_server_handshake();
        client.client_change_cipher_spec();
        client.client_handshake_finished();

        client
    }
    fn send_client_hello(&self) {
        let client_hello: Vec<u8> = Vec::new(); 
        let keys = key_pair();
    }
    fn get_server_hello(&self) {}
    fn make_handshake_keys(&self) {}
    fn parse_server_handshake(&self) {}
    fn client_change_cipher_spec(&self) {}
    fn client_handshake_finished(&self) {}
    
    // misc
    
}
