use anyhow::Result;
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct GameClient {
    client: Client,
    server_url: String,
}

#[derive(Serialize)]
struct CreatePlayerRequest {
    player_id: String,
    player_name: String,
    race: String,
}

#[derive(Deserialize)]
struct CreatePlayerResponse {
    success: bool,
    being_id: Option<String>,
    message: String,
}

#[derive(Serialize)]
struct ProcessCommandRequest {
    player_id: String,
    command: String,
}

#[derive(Deserialize)]
struct ProcessCommandResponse {
    success: bool,
    response: String,
}

impl GameClient {
    pub fn new(server_url: String) -> Self {
        Self {
            client: Client::new(),
            server_url,
        }
    }
    
    pub async fn check_server_health(&self) -> Result<bool> {
        let url = format!("{}/api/health", self.server_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
    
    pub async fn create_player(&self, player_id: String, player_name: String, race: String) -> Result<String> {
        let url = format!("{}/api/players", self.server_url);
        
        let request = CreatePlayerRequest {
            player_id,
            player_name,
            race,
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
            
        if response.status().is_success() {
            let create_response: CreatePlayerResponse = response.json().await?;
            if create_response.success {
                Ok(create_response.message)
            } else {
                Err(anyhow::anyhow!("Failed to create player: {}", create_response.message))
            }
        } else {
            Err(anyhow::anyhow!("Server error: {}", response.status()))
        }
    }
    
    pub async fn process_command(&self, player_id: String, command: String) -> Result<String> {
        let url = format!("{}/api/commands", self.server_url);
        
        let request = ProcessCommandRequest {
            player_id,
            command,
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
            
        if response.status().is_success() {
            let command_response: ProcessCommandResponse = response.json().await?;
            if command_response.success {
                Ok(command_response.response)
            } else {
                Err(anyhow::anyhow!("Command failed"))
            }
        } else {
            Err(anyhow::anyhow!("Server error: {}", response.status()))
        }
    }
}