[![crates.io](https://meritbadge.herokuapp.com/rocheck?)](https://crates.io/crates/rocheck)
[![Released API docs](https://docs.rs/rocheck/badge.svg)](https://docs.rs/rocheck)
[![Build Status](https://travis-ci.com/Fireboltofdeath/RoCheck.svg?branch=master)](https://travis-ci.com/Fireboltofdeath/RoCheck)

# RoCheck
An implementation of [grilme99](https://github.com/grilme99)'s [RoCheck](https://github.com/grilme99/RoCheck) module in Rust.
Credits to him for the method used.

## Installation
```toml
[dependencies]
rocheck = "0.2"
```

## Usage
Using RoCheck is simple, simply feed it in a couple values, and boom!
```rust
use rocheck::RoCheck;

async fn check() -> Result<(), Box<dyn std::error::Error>> {
	let client = RoCheck::new("Your Bot Token");

	let test_ip = "127.0.0.1";
	let job_id = "SomeLongStringOfCharactersShouldGoHere";
	let place_id = 123456;

	let is_roblox = client.verify_ip(place_id, job_id, test_ip).await?;

	if is_roblox {
		println!("Hoorah! You're a real roblox server!");
	}

	Ok(())
}
```
