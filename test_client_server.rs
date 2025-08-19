#!/usr/bin/env cargo run --bin

use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Arceon Client-Server Communication");
    println!("==============================================");
    
    let client = reqwest::Client::new();
    let server_url = "http://localhost:8080";
    
    // Test 1: Health check
    println!("\n1️⃣ Testing health endpoint...");
    match client.get(&format!("{}/api/health", server_url)).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let health: serde_json::Value = response.json().await?;
                println!("✅ Server health: {}", health);
            } else {
                println!("❌ Health check failed: {}", response.status());
            }
        }
        Err(e) => {
            println!("❌ Cannot connect to server: {}", e);
            println!("💡 Make sure to run: ./target/release/arceon-server.exe --port 8080");
            return Ok(());
        }
    }
    
    // Test 2: Create player
    println!("\n2️⃣ Testing player creation...");
    let create_player_request = json!({
        "player_id": "test_client_001",
        "player_name": "APITestHero",
        "race": "Human"
    });
    
    let response = client
        .post(&format!("{}/api/players", server_url))
        .json(&create_player_request)
        .send()
        .await?;
        
    if response.status().is_success() {
        let create_response: serde_json::Value = response.json().await?;
        println!("✅ Player created: {}", create_response);
    } else {
        println!("❌ Player creation failed: {}", response.status());
    }
    
    // Test 3: Process commands
    println!("\n3️⃣ Testing game commands...");
    let commands = ["look", "stats", "who", "help"];
    
    for command in &commands {
        let command_request = json!({
            "player_id": "test_client_001",
            "command": command
        });
        
        let response = client
            .post(&format!("{}/api/commands", server_url))
            .json(&command_request)
            .send()
            .await?;
            
        if response.status().is_success() {
            let command_response: serde_json::Value = response.json().await?;
            println!("📡 Command '{}' response:", command);
            if let Some(response_text) = command_response.get("response") {
                println!("   {}", response_text.as_str().unwrap_or("No response"));
            }
        } else {
            println!("❌ Command '{}' failed: {}", command, response.status());
        }
    }
    
    println!("\n🎉 Client-Server communication test completed!");
    println!("🎮 The GUI now connects to this same server API!");
    
    Ok(())
}