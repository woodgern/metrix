use rocket::local::Client;

use metrix::create_app;

pub fn create_app_client() -> Client {
	Client::new(create_app()).expect("valid rocket instance")
}
