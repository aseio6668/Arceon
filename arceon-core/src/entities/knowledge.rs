use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
// use rand::Rng; // Unused import

/// Reading and Writing system for knowledge transfer between NPCs
/// This system allows NPCs to record knowledge, create libraries, and teach others through text

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTransferSystem {
    pub literacy_registry: HashMap<Uuid, LiteracySkills>,
    pub written_works: HashMap<Uuid, WrittenWork>,
    pub libraries: HashMap<Uuid, Library>,
    pub educational_institutions: HashMap<Uuid, EducationalInstitution>,
    pub knowledge_sharing_events: Vec<KnowledgeEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteracySkills {
    pub npc_id: Uuid,
    pub reading_level: f64,         // 0.0 to 100.0
    pub writing_level: f64,         // 0.0 to 100.0
    pub comprehension_level: f64,   // Understanding of complex texts
    pub teaching_ability: f64,      // Ability to teach literacy to others
    pub languages_known: Vec<Language>,
    pub reading_speed: f64,         // Words per minute
    pub writing_speed: f64,         // Words per minute
    pub preferred_topics: Vec<String>,
    pub learning_history: Vec<LiteracyProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub proficiency: f64,          // 0.0 to 1.0
    pub script_type: ScriptType,
    pub dialect_variations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptType {
    Alphabetic,
    Syllabic,
    Logographic,
    Runic,
    Archaic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteracyProgress {
    pub skill_improved: String,
    pub improvement_amount: f64,
    pub timestamp: DateTime<Utc>,
    pub learning_method: LearningMethod,
    pub teacher_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningMethod {
    SelfStudy,
    DirectTeaching,
    PeerLearning,
    ObservationalLearning,
    PracticalApplication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrittenWork {
    pub work_id: Uuid,
    pub title: String,
    pub author_id: Uuid,
    pub content: String,
    pub work_type: WorkType,
    pub subject_matter: Vec<String>,
    pub complexity_level: f64,      // Reading level required
    pub language: String,
    pub creation_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub copies_made: u32,
    pub readers: Vec<ReaderRecord>,
    pub knowledge_contained: Vec<KnowledgeEntry>,
    pub cultural_impact: f64,
    pub preservation_status: PreservationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkType {
    Manual,              // How-to guides, technical instructions
    History,             // Historical records and chronicles
    Philosophy,          // Philosophical treatises and wisdom
    Science,             // Scientific observations and theories
    Literature,          // Stories, poems, creative works
    Law,                 // Legal codes and regulations
    Religion,            // Religious texts and spiritual guidance
    Biography,           // Personal histories and memoirs
    Reference,           // Dictionaries, encyclopedias
    Correspondence,      // Letters and personal communications
    Recipe,              // Cooking and crafting recipes
    Map,                 // Geographic and exploratory information
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderRecord {
    pub reader_id: Uuid,
    pub reading_start: DateTime<Utc>,
    pub reading_completion: Option<DateTime<Utc>>,
    pub comprehension_level: f64,
    pub notes_taken: Option<String>,
    pub knowledge_gained: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    pub knowledge_id: Uuid,
    pub content: String,
    pub domain: String,
    pub importance: f64,           // How important this knowledge is
    pub accuracy: f64,             // How accurate the information is
    pub verifiability: f64,        // How easily it can be verified
    pub practical_value: f64,      // How useful it is in practice
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreservationStatus {
    Excellent,
    Good,
    Fair,
    Poor,
    Deteriorating,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub library_id: Uuid,
    pub name: String,
    pub location: String,
    pub founder_id: Uuid,
    pub librarians: Vec<Uuid>,
    pub collection: Vec<Uuid>,     // Written work IDs
    pub visiting_scholars: Vec<Uuid>,
    pub reading_rooms: u32,
    pub copying_facilities: bool,
    pub specializations: Vec<String>,
    pub access_policy: AccessPolicy,
    pub knowledge_categories: HashMap<String, Vec<Uuid>>,
    pub patron_records: Vec<PatronRecord>,
    pub establishment_date: DateTime<Utc>,
    pub reputation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessPolicy {
    Open,                          // Anyone can access
    Restricted(Vec<String>),       // Only certain groups
    MembershipBased,               // Requires membership
    ExpertOnly,                    // Only scholars and experts
    Private,                       // Owner/staff only
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatronRecord {
    pub patron_id: Uuid,
    pub first_visit: DateTime<Utc>,
    pub total_visits: u32,
    pub works_read: Vec<Uuid>,
    pub works_contributed: Vec<Uuid>,
    pub scholarly_interests: Vec<String>,
    pub reading_preferences: ReadingPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingPreferences {
    pub preferred_complexity: f64,
    pub preferred_subjects: Vec<String>,
    pub reading_speed: f64,
    pub note_taking_style: NoteTakingStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoteTakingStyle {
    Detailed,
    Summary,
    KeyPoints,
    Questions,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationalInstitution {
    pub institution_id: Uuid,
    pub name: String,
    pub institution_type: InstitutionType,
    pub location: String,
    pub founder_id: Uuid,
    pub instructors: Vec<Uuid>,
    pub students: Vec<Uuid>,
    pub curriculum: Vec<Course>,
    pub library_id: Option<Uuid>,
    pub graduation_requirements: Vec<String>,
    pub reputation: f64,
    pub establishment_date: DateTime<Utc>,
    pub alumni: Vec<AlumnusRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstitutionType {
    BasicSchool,        // Teaches literacy and basic skills
    TradeSchool,        // Focuses on practical skills
    College,            // Advanced academic learning
    University,         // Research and higher learning
    Academy,            // Specialized training
    Seminary,           // Religious education
    Guild,              // Professional organization
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub course_id: Uuid,
    pub name: String,
    pub instructor_id: Uuid,
    pub subject_area: String,
    pub difficulty_level: f64,
    pub required_reading: Vec<Uuid>,
    pub practical_components: Vec<String>,
    pub duration_weeks: u32,
    pub prerequisites: Vec<String>,
    pub learning_outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlumnusRecord {
    pub graduate_id: Uuid,
    pub graduation_date: DateTime<Utc>,
    pub courses_completed: Vec<Uuid>,
    pub final_assessment: f64,
    pub specialization: String,
    pub career_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEvent {
    pub event_id: Uuid,
    pub event_type: KnowledgeEventType,
    pub participants: Vec<Uuid>,
    pub location: String,
    pub timestamp: DateTime<Utc>,
    pub knowledge_exchanged: Vec<String>,
    pub cultural_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeEventType {
    BookWriting(Uuid),             // Written work ID
    LibraryEstablishment(Uuid),    // Library ID
    SchoolFounding(Uuid),          // Institution ID
    LiteracyTeaching,              // Teaching reading/writing
    ScholarlyCongress,             // Academic conference
    BookCopying(Uuid),             // Copying existing work
    KnowledgeDiscovery(String),    // New discovery documented
    CulturalPreservation,          // Recording oral traditions
}

impl KnowledgeTransferSystem {
    pub fn new() -> Self {
        Self {
            literacy_registry: HashMap::new(),
            written_works: HashMap::new(),
            libraries: HashMap::new(),
            educational_institutions: HashMap::new(),
            knowledge_sharing_events: Vec::new(),
        }
    }

    /// Initialize literacy skills for an NPC
    pub fn initialize_npc_literacy(&mut self, npc_id: Uuid, base_intelligence: f64) {
        let literacy = LiteracySkills {
            npc_id,
            reading_level: base_intelligence * 10.0,
            writing_level: base_intelligence * 8.0,
            comprehension_level: base_intelligence * 12.0,
            teaching_ability: base_intelligence * 5.0,
            languages_known: vec![Language {
                name: "Common Tongue".to_string(),
                proficiency: 0.8,
                script_type: ScriptType::Alphabetic,
                dialect_variations: vec!["Archaic".to_string()],
            }],
            reading_speed: 50.0 + (base_intelligence * 20.0),
            writing_speed: 25.0 + (base_intelligence * 15.0),
            preferred_topics: Vec::new(),
            learning_history: Vec::new(),
        };

        self.literacy_registry.insert(npc_id, literacy);
    }

    /// NPC creates a written work
    pub fn create_written_work(&mut self, author_id: Uuid, title: String, content: String, work_type: WorkType, subject_matter: Vec<String>) -> Result<Uuid, String> {
        let literacy = self.literacy_registry.get(&author_id)
            .ok_or("Author not found in literacy registry")?;

        if literacy.writing_level < 30.0 {
            return Err("Author's writing level too low to create written work".to_string());
        }

        let work_id = Uuid::new_v4();
        let complexity_level = (literacy.writing_level / 100.0) * 80.0 + 20.0; // 20-100 complexity

        let work = WrittenWork {
            work_id,
            title,
            author_id,
            content: content.clone(),
            work_type,
            subject_matter: subject_matter.clone(),
            complexity_level,
            language: "Common Tongue".to_string(),
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            copies_made: 1,
            readers: Vec::new(),
            knowledge_contained: self.extract_knowledge_from_content(&content, &subject_matter),
            cultural_impact: 0.1,
            preservation_status: PreservationStatus::Excellent,
        };

        self.written_works.insert(work_id, work);

        // Record the knowledge event
        self.knowledge_sharing_events.push(KnowledgeEvent {
            event_id: Uuid::new_v4(),
            event_type: KnowledgeEventType::BookWriting(work_id),
            participants: vec![author_id],
            location: "Author's workspace".to_string(),
            timestamp: Utc::now(),
            knowledge_exchanged: subject_matter,
            cultural_impact: 0.1,
        });

        Ok(work_id)
    }

    /// NPC reads a written work
    pub fn read_written_work(&mut self, reader_id: Uuid, work_id: Uuid) -> Result<Vec<String>, String> {
        let literacy = self.literacy_registry.get(&reader_id)
            .ok_or("Reader not found in literacy registry")?;

        let work = self.written_works.get_mut(&work_id)
            .ok_or("Written work not found")?;

        if literacy.reading_level < work.complexity_level {
            return Err("Reader's literacy level too low for this work".to_string());
        }

        // Calculate comprehension based on reader's skills
        let comprehension = (literacy.comprehension_level / 100.0).min(1.0);
        let _reading_time = work.content.len() as f64 / literacy.reading_speed;

        // Create reader record
        let reader_record = ReaderRecord {
            reader_id,
            reading_start: Utc::now(),
            reading_completion: Some(Utc::now()),
            comprehension_level: comprehension,
            notes_taken: None,
            knowledge_gained: work.knowledge_contained.iter()
                .filter(|_entry| rand::random::<f64>() < comprehension)
                .map(|entry| entry.content.clone())
                .collect(),
        };

        let knowledge_gained = reader_record.knowledge_gained.clone();
        work.readers.push(reader_record);

        // Improve reader's literacy through reading
        if let Some(reader_literacy) = self.literacy_registry.get_mut(&reader_id) {
            reader_literacy.reading_level += 0.5;
            reader_literacy.comprehension_level += 0.2;
            reader_literacy.learning_history.push(LiteracyProgress {
                skill_improved: "Reading".to_string(),
                improvement_amount: 0.5,
                timestamp: Utc::now(),
                learning_method: LearningMethod::SelfStudy,
                teacher_id: None,
            });
        }

        Ok(knowledge_gained)
    }

    /// NPC teaches literacy to another NPC
    pub fn teach_literacy(&mut self, teacher_id: Uuid, student_id: Uuid, skill_focus: LiteracySkill) -> Result<f64, String> {
        let teacher_ability = self.literacy_registry.get(&teacher_id)
            .map(|lit| lit.teaching_ability)
            .ok_or("Teacher not found")?;

        if teacher_ability < 20.0 {
            return Err("Teacher's ability too low to teach literacy".to_string());
        }

        let improvement = teacher_ability * 0.1;

        if let Some(student_literacy) = self.literacy_registry.get_mut(&student_id) {
            match skill_focus {
                LiteracySkill::Reading => student_literacy.reading_level += improvement,
                LiteracySkill::Writing => student_literacy.writing_level += improvement,
                LiteracySkill::Comprehension => student_literacy.comprehension_level += improvement,
            }

            student_literacy.learning_history.push(LiteracyProgress {
                skill_improved: format!("{:?}", skill_focus),
                improvement_amount: improvement,
                timestamp: Utc::now(),
                learning_method: LearningMethod::DirectTeaching,
                teacher_id: Some(teacher_id),
            });
        }

        // Also improve teacher's teaching ability
        if let Some(teacher_literacy) = self.literacy_registry.get_mut(&teacher_id) {
            teacher_literacy.teaching_ability += 0.5;
        }

        Ok(improvement)
    }

    /// Establish a library
    pub fn establish_library(&mut self, founder_id: Uuid, name: String, location: String, initial_collection: Vec<Uuid>) -> Uuid {
        let library_id = Uuid::new_v4();

        let library = Library {
            library_id,
            name: name.clone(),
            location: location.clone(),
            founder_id,
            librarians: vec![founder_id],
            collection: initial_collection.clone(),
            visiting_scholars: Vec::new(),
            reading_rooms: 3,
            copying_facilities: true,
            specializations: Vec::new(),
            access_policy: AccessPolicy::Open,
            knowledge_categories: HashMap::new(),
            patron_records: Vec::new(),
            establishment_date: Utc::now(),
            reputation: 10.0,
        };

        self.libraries.insert(library_id, library);

        // Record the event
        self.knowledge_sharing_events.push(KnowledgeEvent {
            event_id: Uuid::new_v4(),
            event_type: KnowledgeEventType::LibraryEstablishment(library_id),
            participants: vec![founder_id],
            location,
            timestamp: Utc::now(),
            knowledge_exchanged: vec!["Library establishment".to_string()],
            cultural_impact: 0.5,
        });

        library_id
    }

    /// NPC visits a library to read
    pub fn visit_library(&mut self, visitor_id: Uuid, library_id: Uuid, desired_subject: Option<String>) -> Result<Vec<Uuid>, String> {
        let library = self.libraries.get_mut(&library_id)
            .ok_or("Library not found")?;

        // Check access policy
        match &library.access_policy {
            AccessPolicy::Open => {},
            AccessPolicy::Private => {
                if !library.librarians.contains(&visitor_id) {
                    return Err("Access denied: Private library".to_string());
                }
            },
            _ => {} // For now, allow other access types
        }

        // Find suitable works based on visitor's literacy and interests
        let visitor_literacy = self.literacy_registry.get(&visitor_id)
            .ok_or("Visitor literacy not found")?;

        let suitable_works: Vec<Uuid> = library.collection.iter()
            .filter(|work_id| {
                if let Some(work) = self.written_works.get(work_id) {
                    work.complexity_level <= visitor_literacy.reading_level &&
                    (desired_subject.is_none() || 
                     work.subject_matter.contains(&desired_subject.as_ref().unwrap()))
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        // Record patron visit
        if let Some(existing_record) = library.patron_records.iter_mut()
            .find(|record| record.patron_id == visitor_id) {
            existing_record.total_visits += 1;
        } else {
            library.patron_records.push(PatronRecord {
                patron_id: visitor_id,
                first_visit: Utc::now(),
                total_visits: 1,
                works_read: Vec::new(),
                works_contributed: Vec::new(),
                scholarly_interests: Vec::new(),
                reading_preferences: ReadingPreferences {
                    preferred_complexity: visitor_literacy.reading_level,
                    preferred_subjects: visitor_literacy.preferred_topics.clone(),
                    reading_speed: visitor_literacy.reading_speed,
                    note_taking_style: NoteTakingStyle::Summary,
                },
            });
        }

        Ok(suitable_works)
    }

    /// Found an educational institution
    pub fn found_educational_institution(&mut self, founder_id: Uuid, name: String, location: String, institution_type: InstitutionType) -> Uuid {
        let institution_id = Uuid::new_v4();

        let institution = EducationalInstitution {
            institution_id,
            name: name.clone(),
            institution_type,
            location: location.clone(),
            founder_id,
            instructors: vec![founder_id],
            students: Vec::new(),
            curriculum: Vec::new(),
            library_id: None,
            graduation_requirements: vec!["Basic Literacy".to_string()],
            reputation: 5.0,
            establishment_date: Utc::now(),
            alumni: Vec::new(),
        };

        self.educational_institutions.insert(institution_id, institution);

        // Record the event
        self.knowledge_sharing_events.push(KnowledgeEvent {
            event_id: Uuid::new_v4(),
            event_type: KnowledgeEventType::SchoolFounding(institution_id),
            participants: vec![founder_id],
            location,
            timestamp: Utc::now(),
            knowledge_exchanged: vec!["Educational institution establishment".to_string()],
            cultural_impact: 0.8,
        });

        institution_id
    }

    /// Extract knowledge from written content
    fn extract_knowledge_from_content(&self, content: &str, subject_matter: &[String]) -> Vec<KnowledgeEntry> {
        let mut knowledge_entries = Vec::new();

        // Simple knowledge extraction based on content and subjects
        for subject in subject_matter {
            knowledge_entries.push(KnowledgeEntry {
                knowledge_id: Uuid::new_v4(),
                content: format!("Knowledge about {} from written work", subject),
                domain: subject.clone(),
                importance: 0.6,
                accuracy: 0.8,
                verifiability: 0.7,
                practical_value: 0.5,
            });
        }

        // Extract additional knowledge based on content length and complexity
        if content.len() > 1000 {
            knowledge_entries.push(KnowledgeEntry {
                knowledge_id: Uuid::new_v4(),
                content: "Detailed information from comprehensive work".to_string(),
                domain: "General Knowledge".to_string(),
                importance: 0.7,
                accuracy: 0.75,
                verifiability: 0.8,
                practical_value: 0.6,
            });
        }

        knowledge_entries
    }

    /// Get all written works by subject
    pub fn get_works_by_subject(&self, subject: &str) -> Vec<&WrittenWork> {
        self.written_works.values()
            .filter(|work| work.subject_matter.contains(&subject.to_string()))
            .collect()
    }

    /// Get literacy level of an NPC
    pub fn get_npc_literacy(&self, npc_id: Uuid) -> Option<&LiteracySkills> {
        self.literacy_registry.get(&npc_id)
    }

    /// Get library information
    pub fn get_library(&self, library_id: Uuid) -> Option<&Library> {
        self.libraries.get(&library_id)
    }

    /// Get institution information
    pub fn get_institution(&self, institution_id: Uuid) -> Option<&EducationalInstitution> {
        self.educational_institutions.get(&institution_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiteracySkill {
    Reading,
    Writing,
    Comprehension,
}