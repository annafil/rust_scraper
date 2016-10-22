extern crate hyper;
use hyper::Client;

fn main() {
    let client = Client::new(); // new method on struct 
    let res = client.get("http://integer32.com")
                    .send()
                    .expect("Request failed"); // <---- different

    assert_eq!(res.status, hyper::Ok); // what you should be getting back, different OK from results::Ok
}
