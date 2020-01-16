mod models;
use std::collections::HashMap;
use models::*;

pub struct RoCheck {
	cookie: String,
	client: reqwest::Client
}

impl RoCheck {
	fn new(cookie: &str) -> RoCheck {
		let client = reqwest::Client::builder()
			.build().expect("HttpClient failed to initialize");
		
		RoCheck {
			cookie: String::from(cookie),
			client: client
		}
	}

	async fn send_http(&self, url: &str) -> Error<HashMap<String, String>> {
		let req: reqwest::Request = self.client.request(reqwest::Method::GET, url)
			.header("Cookie", format!(".ROBLOSECURITY={}", self.cookie))
				.build()?;
		
		let resp: HashMap<String, String> = self.client.execute(req).await?
			.json().await?;
		
		Ok(resp)
	}

	async fn get_data(&self, place_id: i32, job_id: &str) -> Error<HashMap<String, String>> {
		let init_data = self.send_http(&format!("https://assetgame.roblox.com/Game/PlaceLauncher.ashx?request=RequestGameJob&placeId={}&gameId={}", place_id, job_id)).await?;

		let join_url = init_data.get("joinScriptUrl").expect("joinScriptUrl does not exist");

		let game_data = self.send_http(&join_url).await?;

		Ok(game_data)
	}

	async fn get_ip(&self, place_id: i32, job_id: &str) -> Error<String> {
		let game_data = self.get_data(place_id, job_id).await?;

		let machine_addr = game_data.get("MachineAddress").expect("MachineAddress does not exist");

		Ok(machine_addr.to_string())
	}

	async fn verify_ip(&self, place_id: i32, job_id: &str, ip: &str) -> Error<bool> {
		let mach_ip = self.get_ip(place_id, job_id).await?;
		Ok(mach_ip == ip)
	}
}

pub async fn test() -> Error<()> {
	let rcd: RoCheck = RoCheck::new("cook");
	let _game = rcd.get_ip(27574, "SOME JOB ID LOL XDD").await?;
	let _verified = rcd.verify_ip(27574, "SOME JOB ID LOL XDD", "127.0.0.1").await?;

	Ok(())
}