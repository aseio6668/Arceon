use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArceonError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Blockchain error: {0}")]
    Blockchain(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("World generation error: {0}")]
    WorldGeneration(String),
    
    #[error("AI error: {0}")]
    Ai(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}
