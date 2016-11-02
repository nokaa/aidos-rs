#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hayaku;
extern crate toml;

use hayaku::{forms, Http, Request, ResponseWriter, Path, Status};

use std::fs;
use std::io::Read;
use std::collections::HashMap;
use std::rc::Rc;

type Ctx = HashMap<String, String>;

fn main() {
    env_logger::init().unwrap();
    info!("Starting up");

    let mut file = fs::File::open("config.toml").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let value: toml::Value = toml::Value::Table(toml::Parser::new(&buf).parse().unwrap());
    let config: HashMap<String, String> = toml::decode(value).unwrap();
    info!("{:?}", config);

    let addr = "0.0.0.0:5000".parse().unwrap();

    let mut http = Http::new(config);
    http.handle_func(Path::from("/search"), Rc::new(search_handler));
    http.listen_and_serve(addr);
}

fn search_handler(req: &Request, res: &mut ResponseWriter, ctx: &Ctx) {
    let buf = match req.body {
        Some(ref b) => &b.data[..],
        None => panic!("no body found"),
    };
    let form = forms::parse_form(&buf).unwrap();
    let search = form.get(&"q".to_string()).unwrap().clone();
    if search[0] == b'!' {
        let (bang, search) = split(&search);
        let url = match ctx.get(&bang) {
            Some(u) => u,
            None => ctx.get("default").unwrap(),
        };
        let url = url.clone() + &search;
        res.redirect(Status::Found, url.as_bytes(), b"You are being redirected")
            .unwrap();
    } else {
        // let search = String::from_utf8(search).unwrap();
        let url = ctx.get("default").unwrap();
        // let url = ctx.get("default").unwrap().clone() + &search;
        res.redirect(Status::TemporaryRedirect,
                      url.as_bytes(),
                      b"You are being redirected")
            .unwrap();
    }
}

// (bang, search_term)
fn split(search: &[u8]) -> (String, String) {
    let mut i = 0usize;
    for &b in search {
        match b {
            b' ' => {
                // let bang = &search[1..i];
                let bang = String::from_utf8(Vec::from(&search[1..i])).unwrap();
                let search = String::from_utf8(Vec::from(&search[i + 1..])).unwrap();
                return (bang, search);
            }
            _ => {}
        }
        i += 1;
    }
    panic!("Unable to split search");
}
