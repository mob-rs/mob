extern crate mob_server;

use mob_server::*;

fn main() {
    web::app(db::default_pool()).launch();
}
