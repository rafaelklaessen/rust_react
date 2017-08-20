extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use router::Router;
use staticfile::Static;
use mount::Mount;

#[allow(unused_variables)]
fn index(req: &mut Request) -> IronResult<Response> {
    let mut file = File::open("app/index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Ok(Response::with((status::Ok, Header(ContentType::html()), contents)))
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>()
                        .unwrap()
                        .find("query")
                        .unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn main() {
    let mut router = Router::new();
    router.get("/", index, "index");
    router.get("/:query", handler, "query");

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/static/eh", Static::new(Path::new("test/")));

    Iron::new(mount).http("localhost:3000").unwrap();
}
