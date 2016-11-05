#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hayaku;
extern crate toml;

use hayaku::{Http, Request, ResponseWriter, Path, Status};

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
    let search = req.form_value("q".to_string()).unwrap();
    if search.starts_with('!') {
        let terms: Vec<&str> = search.splitn(2, ' ').collect();
        let (bang, search): (&str, &str) = (terms[0], terms[1]);
        let url = match ctx.get(bang) {
            Some(u) => u,
            None => ctx.get("default").unwrap(),
        };
        let url = url.clone() + search;
        res.redirect(Status::Found, url.as_bytes(), b"You are being redirected")
            .unwrap();
    } else {
        let url = ctx.get("default").unwrap();
        res.redirect(Status::TemporaryRedirect,
                      url.as_bytes(),
                      b"You are being redirected")
            .unwrap();
    }
}
