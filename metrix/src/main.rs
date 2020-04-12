#[macro_use] extern crate diesel_migrations;
embed_migrations!();

use metrix::{
	db::establish_connection,
	create_app,
};

fn main() {
    println!("### Enter the Metrix ###");
    let db_conn = establish_connection();

    println!("### running db migrations...");
    let result = embedded_migrations::run(&db_conn);
    println!("### migration done; result: {}", result.is_ok());

    create_app().launch();
}
