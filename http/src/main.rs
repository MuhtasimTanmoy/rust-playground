use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Write;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

fn fetch_users() -> Result<Vec<User>, reqwest::Error> {
    let client = Client::new();
    let users: Vec<User> = client
        .get("https://jsonplaceholder.typicode.com/users")
        .send()?
        .json()?;
    Ok(users)
}

fn filter_users(users: Vec<User>) -> Vec<User> {
    users
        .into_iter()
        .filter(|user| user.email.contains("@"))
        .collect()
}

fn save_users(users: &[User]) -> Result<(), std::io::Error> {
    let json = serde_json::to_string(users)?;
    let mut file = File::create("users.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let mut file = File::open("users.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let users: Vec<User> = serde_json::from_str(&contents)?;
    Ok(users)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch users from API
    let users = fetch_users()?;
    println!("Fetched {} users", users.len());

    // Filter users
    let filtered_users = filter_users(users);
    println!("Filtered to {} users with .com emails", filtered_users.len());

    // Save to file
    save_users(&filtered_users)?;
    println!("Saved filtered users to users.json");

    // Load back from file
    let loaded_users = load_users()?;
    println!("Loaded {} users from file", loaded_users.len());

    // Print the first user as a sanity check
    if let Some(user) = loaded_users.first() {
        println!("First user: {:?}", user);
    }

    Ok(())
}