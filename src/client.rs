

pub struct Client {
    peer_id: String,
    info_hash: String,
}

impl Client {
    pub fn new(peer_id: String, info_hash: String) -> Client {
        Client {peer_id, info_hash}
    }
    pub async fn connect(
        &self,
        client: reqwest::Client,
        first: bool,
        uploaded: u64,
        downloaded: u64,
    ) -> Result<(), ()> {
        let params = [
            ("info_hash", self.info_hash.as_ref()),
            ("peer_id", self.peer_id.as_ref()),
            ("uploaded", "0"),
            ("downloaded", "0"),
            ("left", ""),
            ("port", "6889"),
            ("compact", "1"),
        ];
    
        println!("{}", url_encode(&params));
    
        // let res = reqwest::get("http://example.com").await.unwrap();
        // println!("Status: {}", res.status());
        // println!("Headers:\n{:#?}", res.headers());
    
        // let body = res.text().await.unwrap();
        // println!("Body:\n{}", body);
    
        Ok(())
    }
    pub async fn start(&self) {
        let client = reqwest::Client::new();
    
        // let body = reqwest::get("https://torrent.ubuntu.com/announce?info_hash=%90%28%9F%D3M%FC%1C%F8%F3%16%A2h%AD%D85L%853DX&peer_id=-PC0001-706887310628&uploaded=0&downloaded=0&left=699400192&port=6889&compact=1")
        //     .await.unwrap()
        //     .text()
        //     .await.unwrap();
    
        // println!("RESPONCE: {body}");
    
        self.connect(client, true, 0, 0).await;
    }
}

fn url_encode(params: &[(&str, &str)]) -> String {
    let mut output = String::new();

    for param in params {
        output.push_str(&urlencoding::encode(param.0));
        output.push('=');
        output.push_str(&urlencoding::encode(param.1));
        output.push('&');
    }
    output.pop();

    output
}

