use rocheck::RoCheck;
use tokio::runtime::Runtime;


fn main() {
	let client = RoCheck::new("Bot Token");


	rouille::start_server("127.0.0.1:8000", move |request| {
		let ip: &str = request.header("X-Forwarded-For").unwrap();
		let placeid: i64 = request.header("Roblox-Id").unwrap_or("12356").parse().unwrap();
		let jobid: &str = request.header("Job-Id").unwrap_or("fake job id");
		println!("{} {}", placeid, jobid);

		let valid = Runtime::new().unwrap().block_on( client.verify_ip(placeid, jobid, ip) );
		if let Ok(valid) = valid {
			if valid {
				rouille::Response::text("You are a roblox server.")
			} else {
				rouille::Response::text("You are not a roblox server.")
			}
		} else {
			rouille::Response::text(format!("{}", valid.err().unwrap().to_string()))
		}
	});
}
