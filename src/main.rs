extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;

use std::path::Path;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>()
                        .unwrap()
                        .find("query")
                        .unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "query");

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/static/eh", Static::new(Path::new("test/")));

    Iron::new(mount).http("localhost:3000").unwrap();
}
