extern crate hyper;
use hyper::Client;
use std::io::Read;
use select::document::Document;


fn main() {
    let client = Client::new(); // new method on struct 
    let mut res = client.get("https://brson.github.io/demo/wishlist.html")
                    .send()
                    .expect("Request failed"); // <---- different

    let mut body = String::new();
    res.read_to_string(&mut body).expect("Read failed");
    println!("{:?}", body);

    let document = Document::from(body.as_str());
    println!("doc = {:#?}", document);
}
