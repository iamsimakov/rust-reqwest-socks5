extern crate reqwest;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too less args, run it: `run secret_id body the msg`");
        return;
    }
    let chat = &args[1];
    let msg = &args[2..].join(" ");

    let proxy = reqwest::Proxy::all("socks5://user:pswd@server:90").unwrap();

    let client = reqwest::Client::builder()
        .proxy(proxy)
        .build().unwrap();

    let _resp = client.get("http://api.secret.org/url:path1/send")
        .query(&[("param1", chat), ("param2", msg)])
        .send();

}
