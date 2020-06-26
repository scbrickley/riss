extern crate easy_http_request;
extern crate serde;
extern crate serde_json;

use easy_http_request::DefaultHttpRequest as dhr;

use serde::{ Deserialize, Serialize };

use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct IssPosition {
	longitude: String,
	latitude: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IssData {
	timestamp: u64,
	iss_position: IssPosition,
}

impl IssData {
	fn to_line_protocol(&self) -> String {
		let lp = format!(
			"iss_position latitude={},longitude={} {}\n",
			self.iss_position.latitude, self.iss_position.longitude, self.timestamp,
		);

		lp
	}
}

fn main() {
	let iss: IssData = serde_json::from_str(&iss_data()).unwrap();

	println!("{}", iss.to_line_protocol());
}

fn iss_data() -> String {
	let resp = dhr::get_from_url_str("http://api.open-notify.org/iss-now.json")
		.unwrap()
		.send()
		.unwrap();

	String::from_utf8(resp.body).unwrap()
}

fn token() -> String {
	fs::read_to_string("/Users/seanbrickley/.tok/ossinflux")
		.unwrap()
		.trim()
		.to_string()
}
