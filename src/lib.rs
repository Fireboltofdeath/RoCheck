/*!
	# RoCheck
	An implementation of [grilme99](https://github.com/grilme99)'s [RoCheck](https://github.com/grilme99/RoCheck) module in Rust.
	Credits to him for the method used.

	## Installation
	```toml
	[dependencies]
	rocheck = "0.1"
	```

	## Usage
	Using RoCheck is simple, simply feed it in a couple values, and boom!
	```
	use rocheck::RoCheck;

	fn main() {
		let client = RoCheck::new("Your Bot Token");

		let test_ip = "127.0.0.1";
		let job_id = "SomeLongStringOfCharactersShouldGoHere";
		let place_id = 123456;

		let is_roblox = client.verify_ip(place_id, job_id, test_ip);

		if is_roblox {
			println!("Hoorah! You're a real roblox server!");
		}
	}
	```
*/

#[macro_use] extern crate lazy_static;

mod models;
use std::collections::HashMap;
use models::*;
use serde_json::Value;
use regex::Regex;


pub struct RoCheck {
	cookie: String,
	client: reqwest::Client
}

impl RoCheck {
	/**
		Create and initialize a new RoCheck class.
		```rust
		# use std::error::Error
		#
		# fn main() -> Result<(), Box<dyn Error>> {
		let client = RoCheck::new("MySecurityToken");
		# }
		```
	*/
	pub fn new(cookie: &str) -> RoCheck {
		let client = reqwest::Client::builder()
			.build().expect("HttpClient failed to initialize");
		
		RoCheck {
			cookie: String::from(cookie),
			client: client
		}
	}


	/**
		Retrieve additional data other than the IP.
		```rust
		let client = RoCheck::new("MySecurityToken");
		let data = client.get_data(123456, "JobIdFromRequest").await?;
		data.get("some-field")
		```
	*/
	pub async fn get_data(&self, place_id: i64, job_id: &str) -> Error<HashMap<String, serde_json::Value>> {
		let init_data = self.send_http(&format!("https://assetgame.roblox.com/Game/PlaceLauncher.ashx?request=RequestGameJob&placeId={}&gameId={}", place_id, job_id), Some(place_id)).await?;

		let join_url = init_data.get("joinScriptUrl").expect("joinScriptUrl does not exist");
		
		if let Value::String(join_url) = join_url {
			let game_data = self.send_http(&join_url, None).await?;

			Ok(game_data)
		} else {
			Err(Box::new(RoError::Failure("Join URL is not there")))
		}
	}

	/**
		Retrieve the IP belonging to this PlaceId and JobId
		```rust
		let client = RoCheck::new("MySecurityToken");
		let ip = client.get_ip(123456, "JobIdFromRequest").await?;
		```
	*/
	pub async fn get_ip(&self, place_id: i64, job_id: &str) -> Error<String> {
		let game_data = self.get_data(place_id, job_id).await?;

		let machine_addr = game_data.get("MachineAddress").expect("MachineAddress does not exist");

		Ok(machine_addr.as_str().unwrap().to_string())
	}

	/**
		Retrieve the IP belonging to this PlaceId and JobId and compare it to the inputted IP.
		```rust
		let my_ip = "127.0.0.1";
		let client = RoCheck::new("MySecurityToken");
		
		let ip_verified = client.verify_ip(123456, "JobIdFromRequest", my_ip).await?;
		```
	*/
	pub async fn verify_ip(&self, place_id: i64, job_id: &str, ip: &str) -> Error<bool> {
		let mach_ip = self.get_ip(place_id, job_id).await?;
		println!("{} ! {}", mach_ip, ip);
		Ok(mach_ip == ip)
	}


	async fn send_http(&self, url: &str, place_info: Option<i64>) -> Error<HashMap<String, serde_json::Value>> {
		let mut req = self.client.request(reqwest::Method::GET, url)
			.header("Cookie", format!(".ROBLOSECURITY={}", self.cookie));


		if let Some(placeid) = place_info {
			req = req.header("Referer", format!("https://www.roblox.com/games/{}", placeid));
			req = req.header("Origin", "https://www.roblox.com");
			req = req.header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.169 Safari/537.36")
		}

		let req = req.build()?;

		let pltxt: String = self.client.execute(req).await?
			.text().await?;

		lazy_static! {
			static ref sig_removal: Regex = Regex::new("--.*\r\n").expect("Regex is invalid");
		}
		
		let resp: HashMap<String, Value> = serde_json::from_str( sig_removal.replace(&pltxt, "").as_ref() )?;
		
		Ok(resp)
	}
}