use reqwest::blocking::Client;
use serde_json::json;
use serde::Deserialize;
use std::io;
use regex::Regex;

#[derive(Deserialize, Debug)]
struct ResponseItem {
    id: String,
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get username from user input
    let mut input = String::new();
    println!("Username:");
    io::stdin().read_line(&mut input)?;
    let input = input.trim(); // remove trailing newline

    // make a json request body
    let body = json!([input]);

    // create a HTTP client and send a POST request
    let client = Client::new();

    // Send POST request to LittleSkin
    let res = client.post("https://littleskin.cn/api/yggdrasil/api/profiles/minecraft")
        .json(&body)
        .send()?;

    // Get response from LittleSkin
    let response_text_littleskin = res.text()?;
    println!("Response From LittleSkin Yggdrasil API: {:?}\n", response_text_littleskin);

    // Parse JSON response from LittleSkin
    let items: Option<Vec<ResponseItem>> = serde_json::from_str(&response_text_littleskin).ok();

    // Use regex to format UUID
    let re = Regex::new(r"([a-fA-F0-9]{8})([a-fA-F0-9]{4})([a-fA-F0-9]{4})([a-fA-F0-9]{4})([a-fA-F0-9]{12})").unwrap();

    let mut formatted_uuid_littleskin = String::new();
    let mut username = String::new();

    if let Some(items) = items {
        if let Some(first_item) = items.first() {
            username = first_item.name.clone();
        }

        for item in &items {
            if let Some(caps) = re.captures(&item.id) {
                formatted_uuid_littleskin = format!(
                    "{}-{}-{}-{}-{}",
                    &caps[1], &caps[2], &caps[3], &caps[4], &caps[5]
                );
            } else {
                println!("Something wrong. Received: {}\n", item.id);
            }
        }
    }

    println!("Wait, trying to get UUID from Mojang API...\n");

    // Sent GET request to Mojang
    let mojang_url = format!("https://api.mojang.com/users/profiles/minecraft/{}", input);
    let res = client.get(&mojang_url).send()?;

    // Get response from Mojang
    let response_text_mojang = res.text()?;
    println!("Response From Mojang API: {:?}\n", response_text_mojang);

    // Parse JSON response from Mojang
    let item: Option<ResponseItem> = serde_json::from_str(&response_text_mojang).ok();

    let mut formatted_uuid_mojang = String::new();

    if let Some(item) = item {
        if let Some(caps) = re.captures(&item.id) {
            formatted_uuid_mojang = format!(
                "{}-{}-{}-{}-{}",
                &caps[1], &caps[2], &caps[3], &caps[4], &caps[5]
            );
        } else {
            println!("Something wrong. Received: {}", item.id);
        }
    }

    // Check if both LittleSkin and Mojang data are not found
    if username.is_empty() && formatted_uuid_littleskin.is_empty() && formatted_uuid_mojang.is_empty() {
        println!("Not Found");
    } else {
        // Print the result
        if !username.is_empty() {
            println!("Username: {}", username);
        }
        if !formatted_uuid_littleskin.is_empty() {
            println!("LittleSkin Ygg UUID: {}", formatted_uuid_littleskin);
        }
        if !formatted_uuid_mojang.is_empty() {
            println!("Mojang Ygg UUID: {}", formatted_uuid_mojang);
        }
    }

    Ok(())
}
