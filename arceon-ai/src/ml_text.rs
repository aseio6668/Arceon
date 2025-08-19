use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::npc::AiNpc;
use arceon_core::entities::SkillType;

/// Machine learning system for text processing and generation
pub struct MlTextProcessor {
    pub training_data_path: String,
    pub vocabulary: Vocabulary,
    pub models: HashMap<String, TextModel>,
}

/// Vocabulary management for the Common Tongue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vocabulary {
    pub word_to_id: HashMap<String, usize>,
    pub id_to_word: HashMap<usize, String>,
    pub word_frequencies: HashMap<String, u32>,
    pub next_id: usize,
}

/// Text generation model for NPCs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextModel {
    pub model_type: ModelType,
    pub parameters: ModelParameters,
    pub training_status: TrainingStatus,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    LanguageModel,     // For general text generation
    BookWriter,        // For creating in-game books
    ConversationAgent, // For NPC dialogue
    KnowledgeExtractor, // For learning from text
    ArtDescriptor,     // For describing generated art
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameters {
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub num_layers: usize,
    pub learning_rate: f32,
    pub batch_size: usize,
    pub sequence_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingStatus {
    pub is_trained: bool,
    pub epochs_completed: u32,
    pub loss: f32,
    pub last_training_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub perplexity: f32,
    pub coherence_score: f32,
    pub creativity_score: f32,
    pub factual_accuracy: f32,
}

/// A book that NPCs can read and learn from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBook {
    pub id: uuid::Uuid,
    pub title: String,
    pub author: Option<uuid::Uuid>, // NPC ID if written by NPC
    pub content: String,
    pub genre: BookGenre,
    pub difficulty_level: u32,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub topics: Vec<String>,
    pub skills_taught: Vec<SkillType>,
    pub read_by: Vec<uuid::Uuid>, // NPCs who have read this book
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BookGenre {
    Technical,      // Skill guides and manuals
    Historical,     // Lore and past events
    Philosophical,  // Abstract concepts
    Literary,       // Stories and poetry
    Scientific,     // Research and discoveries
    Cultural,       // Racial traditions
    Magical,        // Spells and enchantments
    Practical,      // Everyday knowledge
}

impl MlTextProcessor {
    /// Create a new ML text processor
    pub fn new(training_data_path: String) -> Self {
        Self {
            training_data_path,
            vocabulary: Vocabulary::new(),
            models: HashMap::new(),
        }
    }

    /// Load training data from the gametrainingdata folder
    pub async fn load_training_data(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let path = Path::new(&self.training_data_path);
        let mut training_texts = Vec::new();
        
        if path.exists() {
            let mut entries = tokio::fs::read_dir(path).await?;
            while let Some(entry) = entries.next_entry().await? {
                if let Some(extension) = entry.path().extension() {
                    if extension == "txt" {
                        let content = tokio::fs::read_to_string(entry.path()).await?;
                        training_texts.push(content);
                    }
                }
            }
        }
        
        // Build vocabulary from training data
        for text in &training_texts {
            self.vocabulary.add_text(text);
        }
        
        Ok(training_texts)
    }

    /// Train a model for a specific NPC based on their archetype and needs
    pub async fn train_model_for_npc(&mut self, npc: &AiNpc) -> Result<(), Box<dyn std::error::Error>> {
        if !npc.ml_capabilities.has_text_generation && !npc.ml_capabilities.has_text_comprehension {
            return Ok(()); // NPC doesn't need ML models
        }

        let model_id = format!("npc_{}", npc.id);
        let model_type = self.determine_model_type_for_npc(npc);
        
        let parameters = ModelParameters {
            vocab_size: self.vocabulary.word_to_id.len(),
            hidden_size: 128,
            num_layers: 2,
            learning_rate: npc.ml_capabilities.learning_rate,
            batch_size: 32,
            sequence_length: 50,
        };

        let mut model = TextModel {
            model_type,
            parameters,
            training_status: TrainingStatus {
                is_trained: false,
                epochs_completed: 0,
                loss: 0.0,
                last_training_time: chrono::Utc::now(),
            },
            performance_metrics: PerformanceMetrics {
                perplexity: 0.0,
                coherence_score: 0.0,
                creativity_score: 0.0,
                factual_accuracy: 0.0,
            },
        };

        // Simulate training process (in a real implementation, this would use actual ML frameworks)
        self.simulate_training(&mut model, npc.ml_capabilities.training_iterations).await?;
        
        self.models.insert(model_id, model);
        Ok(())
    }

    fn determine_model_type_for_npc(&self, npc: &AiNpc) -> ModelType {
        match &npc.archetype {
            crate::npc::NpcArchetype::Scholar | crate::npc::NpcArchetype::Teacher => ModelType::BookWriter,
            crate::npc::NpcArchetype::Trader => ModelType::ConversationAgent,
            crate::npc::NpcArchetype::Geomancer => ModelType::ArtDescriptor,
            _ => ModelType::LanguageModel,
        }
    }

    /// Simulate model training (replace with actual ML training in production)
    async fn simulate_training(&self, model: &mut TextModel, iterations: u32) -> Result<(), Box<dyn std::error::Error>> {
        // This is a placeholder for actual ML training
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for epoch in 0..iterations {
            // Simulate training loss decreasing over time
            let loss = 10.0 * (-(epoch as f32) / iterations as f32 * 3.0).exp() + rng.gen_range(0.1..0.5);
            
            if epoch % 100 == 0 {
                tracing::info!("Training epoch {}/{}, loss: {:.4}", epoch, iterations, loss);
            }
            
            model.training_status.loss = loss;
            model.training_status.epochs_completed = epoch;
            
            // Small delay to simulate computation
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
        
        model.training_status.is_trained = true;
        model.training_status.last_training_time = chrono::Utc::now();
        
        // Set performance metrics
        model.performance_metrics = PerformanceMetrics {
            perplexity: rng.gen_range(20.0..50.0),
            coherence_score: rng.gen_range(0.6..0.9),
            creativity_score: rng.gen_range(0.5..0.8),
            factual_accuracy: rng.gen_range(0.7..0.95),
        };
        
        Ok(())
    }

    /// Generate text using a trained model
    pub fn generate_text(&self, npc_id: uuid::Uuid, prompt: &str, max_length: usize) -> Result<String, Box<dyn std::error::Error>> {
        let model_id = format!("npc_{}", npc_id);
        
        if let Some(model) = self.models.get(&model_id) {
            if !model.training_status.is_trained {
                return Err("Model not trained yet".into());
            }
            
            // Simulate text generation (replace with actual model inference)
            let generated = self.simulate_text_generation(prompt, max_length, &model.model_type);
            Ok(generated)
        } else {
            Err("Model not found for NPC".into())
        }
    }

    /// Simulate text generation (replace with actual ML inference)
    fn simulate_text_generation(&self, prompt: &str, max_length: usize, model_type: &ModelType) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let templates = match model_type {
            ModelType::BookWriter => vec![
                "In the ancient times of Arceon, scholars discovered that {} leads to greater understanding of the world.",
                "The art of {} requires patience and dedication, as our ancestors learned through generations of practice.",
                "When studying {}, one must remember that knowledge is meant to be shared with others.",
            ],
            ModelType::ConversationAgent => vec![
                "Greetings, friend! I've been thinking about {} and wondered if you might have insights to share.",
                "The market prices for {} have been quite interesting lately. Perhaps we could discuss a trade?",
                "I've heard tales of {} from travelers. Do you know anything about this matter?",
            ],
            ModelType::ArtDescriptor => vec![
                "The landscape here could be enhanced with {}. I sense the earth yearning for this change.",
                "My connection to the land tells me that {} would bring harmony to this place.",
                "The ancient stones whisper of {} - a transformation that would honor the natural order.",
            ],
            _ => vec![
                "I have been contemplating {} and its implications for our community.",
                "The wisdom of {} has been passed down through generations in my family.",
                "Perhaps {} holds the key to solving the challenges we face.",
            ],
        };
        
        let template = templates[rng.gen_range(0..templates.len())];
        let generated = template.replace("{}", prompt);
        
        // Truncate to max_length if needed
        if generated.len() > max_length {
            format!("{}...", &generated[..max_length.saturating_sub(3)])
        } else {
            generated
        }
    }

    /// Extract knowledge and concepts from text
    pub fn extract_knowledge(&self, text: &str) -> Vec<String> {
        // Simple keyword extraction (replace with actual NLP)
        let mut concepts = Vec::new();
        
        let keywords = [
            "skill", "craft", "magic", "trade", "build", "learn", "teach",
            "mine", "farm", "fight", "defend", "create", "art", "knowledge",
            "wisdom", "alliance", "cooperation", "leadership", "innovation"
        ];
        
        for keyword in &keywords {
            if text.to_lowercase().contains(keyword) {
                concepts.push(keyword.to_string());
            }
        }
        
        concepts
    }
}

impl Vocabulary {
    fn new() -> Self {
        Self {
            word_to_id: HashMap::new(),
            id_to_word: HashMap::new(),
            word_frequencies: HashMap::new(),
            next_id: 0,
        }
    }

    fn add_text(&mut self, text: &str) {
        use unicode_segmentation::UnicodeSegmentation;
        
        let words: Vec<&str> = text.unicode_words().collect();
        
        for word in words {
            let word_lower = word.to_lowercase();
            
            // Update frequency
            *self.word_frequencies.entry(word_lower.clone()).or_insert(0) += 1;
            
            // Add to vocabulary if new
            if !self.word_to_id.contains_key(&word_lower) {
                self.word_to_id.insert(word_lower.clone(), self.next_id);
                self.id_to_word.insert(self.next_id, word_lower);
                self.next_id += 1;
            }
        }
    }
}

impl GameBook {
    /// Create a new book authored by an NPC
    pub fn create_by_npc(author_id: uuid::Uuid, title: String, content: String, genre: BookGenre) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            author: Some(author_id),
            content,
            genre,
            difficulty_level: 1,
            creation_date: chrono::Utc::now(),
            topics: Vec::new(),
            skills_taught: Vec::new(),
            read_by: Vec::new(),
        }
    }

    /// Mark that an NPC has read this book
    pub fn mark_read_by(&mut self, npc_id: uuid::Uuid) {
        if !self.read_by.contains(&npc_id) {
            self.read_by.push(npc_id);
        }
    }

    /// Extract skills that this book can teach
    pub fn analyze_content_for_skills(&mut self) {
        // Simple content analysis to determine what skills the book teaches
        let content_lower = self.content.to_lowercase();
        
        let skill_keywords = [
            ("mining", SkillType::Mining),
            ("blacksmith", SkillType::Smithing),
            ("craft", SkillType::Carpentry),
            ("sword", SkillType::OneHandedSword),
            ("magic", SkillType::FireMagic),
            ("leadership", SkillType::Leadership),
            ("trade", SkillType::Trading),
        ];
        
        for (keyword, skill) in &skill_keywords {
            if content_lower.contains(keyword) && !self.skills_taught.contains(skill) {
                self.skills_taught.push(*skill);
            }
        }
        
        // Set difficulty based on content complexity
        self.difficulty_level = if self.content.len() > 2000 { 3 }
                              else if self.content.len() > 1000 { 2 }
                              else { 1 };
    }
}
