use std::collections::HashMap;
use crate::genesis::{AncientKnowledge, KnowledgeType, RunestoneType, ActivationRequirement};

/// Sacred runestones containing ancient knowledge to guide NPC civilization development
pub struct RunestoneLibrary {
    pub knowledge_database: HashMap<RunestoneType, Vec<AncientKnowledge>>,
}

impl RunestoneLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            knowledge_database: HashMap::new(),
        };
        library.initialize_ancient_knowledge();
        library
    }

    /// Initialize all the ancient knowledge that runestones can contain
    fn initialize_ancient_knowledge(&mut self) {
        self.initialize_crafting_knowledge();
        self.initialize_magic_knowledge();
        self.initialize_alchemy_knowledge();
        self.initialize_architecture_knowledge();
        self.initialize_agriculture_knowledge();
        self.initialize_currency_knowledge();
        self.initialize_writing_knowledge();
        self.initialize_technology_knowledge();
        self.initialize_social_knowledge();
        self.initialize_spiritual_knowledge();
    }

    /// Basic crafting knowledge for primitive NPCs to start civilization
    fn initialize_crafting_knowledge(&mut self) {
        let crafting_knowledge = vec![
            AncientKnowledge {
                title: "The Art of Tool Making".to_string(),
                knowledge_type: KnowledgeType::BasicCrafting,
                difficulty_level: 1,
                prerequisites: vec![],
                instructions: "Seek stones of hardness, bind them to wooden shafts with sinew or vine. The sharp edge shall cut, the blunt end shall pound. First make the hand-axe, then the spear, then tools for other crafts.".to_string(),
                ingredients_required: vec!["Stone".to_string(), "Wood".to_string(), "Animal Sinew".to_string()],
                tools_required: vec!["Hands".to_string(), "Rocks for knapping".to_string()],
                skill_requirements: HashMap::from([("Manual Dexterity".to_string(), 10)]),
            },
            AncientKnowledge {
                title: "Weaving of Cloth and Garments".to_string(),
                knowledge_type: KnowledgeType::BasicCrafting,
                difficulty_level: 2,
                prerequisites: vec!["The Art of Tool Making".to_string()],
                instructions: "Gather fibers from plants or fur from beasts. Twist these into threads by rolling between palm and thigh. Create the loom from wooden frame, weave threads in crossing patterns. First make simple coverings for warmth, then finer garments for status.".to_string(),
                ingredients_required: vec!["Plant Fiber".to_string(), "Animal Fur".to_string(), "Wood".to_string()],
                tools_required: vec!["Basic Tools".to_string(), "Loom".to_string()],
                skill_requirements: HashMap::from([("Weaving".to_string(), 15), ("Manual Dexterity".to_string(), 20)]),
            },
            AncientKnowledge {
                title: "The Forging of Metal".to_string(),
                knowledge_type: KnowledgeType::AdvancedCrafting,
                difficulty_level: 4,
                prerequisites: vec!["The Art of Tool Making".to_string(), "Mastery of Fire".to_string()],
                instructions: "Seek the colored stones that gleam in riverbeds and mountainsides. Build fires that burn hotter than cooking flames. Hammer the heated metal upon anvil stone. First copper, then bronze when tin is found, finally iron for the strongest tools and weapons.".to_string(),
                ingredients_required: vec!["Copper Ore".to_string(), "Tin Ore".to_string(), "Iron Ore".to_string(), "Coal".to_string()],
                tools_required: vec!["Forge".to_string(), "Hammer".to_string(), "Anvil".to_string(), "Bellows".to_string()],
                skill_requirements: HashMap::from([("Smithing".to_string(), 30), ("Fire Management".to_string(), 25)]),
            },
        ];

        self.knowledge_database.insert(RunestoneType::CraftingStone, crafting_knowledge);
    }

    /// Magical knowledge for those who seek arcane power
    fn initialize_magic_knowledge(&mut self) {
        let magic_knowledge = vec![
            AncientKnowledge {
                title: "First Steps of Arcane Study".to_string(),
                knowledge_type: KnowledgeType::Magic,
                difficulty_level: 3,
                prerequisites: vec!["Basic Language Skills".to_string()],
                instructions: "Magic flows through all things like invisible rivers. To touch this power, one must first quiet the mind and feel the energy within. Meditate at dawn and dusk, place hands upon living trees and flowing water. The wise learn to sense before they learn to shape.".to_string(),
                ingredients_required: vec!["Meditation Space".to_string(), "Natural Environment".to_string()],
                tools_required: vec!["Focused Mind".to_string(), "Peaceful Location".to_string()],
                skill_requirements: HashMap::from([("Meditation".to_string(), 20), ("Intuition".to_string(), 25)]),
            },
            AncientKnowledge {
                title: "Elemental Manipulation".to_string(),
                knowledge_type: KnowledgeType::Magic,
                difficulty_level: 5,
                prerequisites: vec!["First Steps of Arcane Study".to_string()],
                instructions: "Each element responds to different aspects of will and emotion. Fire burns with passion and anger, water flows with calm and adaptation, earth stands with patience and strength, air moves with freedom and change. Practice with small flames, droplets, pebbles, and breezes before attempting greater workings.".to_string(),
                ingredients_required: vec!["Elemental Focuses".to_string(), "Safe Practice Area".to_string()],
                tools_required: vec!["Elemental Talismans".to_string(), "Protected Workspace".to_string()],
                skill_requirements: HashMap::from([("Fire Magic".to_string(), 30), ("Water Magic".to_string(), 30), ("Earth Magic".to_string(), 30), ("Air Magic".to_string(), 30)]),
            },
            AncientKnowledge {
                title: "The Binding of Enchantments".to_string(),
                knowledge_type: KnowledgeType::Magic,
                difficulty_level: 7,
                prerequisites: vec!["Elemental Manipulation".to_string(), "Advanced Crafting".to_string()],
                instructions: "To make magic permanent in objects, one must forge connections between the material and spiritual realms. Inscribe symbols of power upon well-crafted items during celestial alignments. The enchantment draws power from the creator's own life force, so work slowly and with great care.".to_string(),
                ingredients_required: vec!["Quality Crafted Items".to_string(), "Rare Inks".to_string(), "Gemstone Focuses".to_string()],
                tools_required: vec!["Enchanting Tools".to_string(), "Ritual Circle".to_string(), "Star Charts".to_string()],
                skill_requirements: HashMap::from([("Enchanting".to_string(), 50), ("Crafting".to_string(), 40), ("Astronomy".to_string(), 30)]),
            },
        ];

        self.knowledge_database.insert(RunestoneType::MagicStone, magic_knowledge);
    }

    /// Alchemical knowledge for potion making and transformation
    fn initialize_alchemy_knowledge(&mut self) {
        let alchemy_knowledge = vec![
            AncientKnowledge {
                title: "Principles of Herbal Healing".to_string(),
                knowledge_type: KnowledgeType::Alchemy,
                difficulty_level: 2,
                prerequisites: vec!["Basic Plant Knowledge".to_string()],
                instructions: "Every plant holds within it properties of healing or harm. Learn to identify by leaf shape, flower color, and growing conditions. Willow bark soothes pain, chamomile calms the spirit, mint aids digestion. Prepare by drying, grinding, or steeping in hot water. Never mix unknown herbs.".to_string(),
                ingredients_required: vec!["Common Herbs".to_string(), "Clean Water".to_string(), "Drying Racks".to_string()],
                tools_required: vec!["Mortar and Pestle".to_string(), "Drying Equipment".to_string(), "Storage Containers".to_string()],
                skill_requirements: HashMap::from([("Herbalism".to_string(), 25), ("Plant Identification".to_string(), 20)]),
            },
            AncientKnowledge {
                title: "Transmutation of Base Materials".to_string(),
                knowledge_type: KnowledgeType::Alchemy,
                difficulty_level: 6,
                prerequisites: vec!["Principles of Herbal Healing".to_string(), "Elemental Manipulation".to_string()],
                instructions: "All matter is but energy in different forms. Through precise combination of herbs, minerals, and magical force, one substance may become another. Begin with simple changes: water to wine, copper to silver. The greatest secret lies in the universal solvent that dissolves all barriers between forms.".to_string(),
                ingredients_required: vec!["Rare Minerals".to_string(), "Magical Catalysts".to_string(), "Distillation Apparatus".to_string()],
                tools_required: vec!["Alchemical Laboratory".to_string(), "Precision Scales".to_string(), "Sealed Containers".to_string()],
                skill_requirements: HashMap::from([("Alchemy".to_string(), 45), ("Chemistry".to_string(), 35), ("Magic Theory".to_string(), 40)]),
            },
        ];

        self.knowledge_database.insert(RunestoneType::AlchemyStone, alchemy_knowledge);
    }

    /// Architectural knowledge for building lasting structures
    fn initialize_architecture_knowledge(&mut self) {
        let architecture_knowledge = vec![
            AncientKnowledge {
                title: "Foundations of Permanent Shelter".to_string(),
                knowledge_type: KnowledgeType::Architecture,
                difficulty_level: 2,
                prerequisites: vec!["The Art of Tool Making".to_string()],
                instructions: "First seek high ground that drains well, away from flooding but near water source. Dig foundation deep as a person's height, fill with stones for drainage. Build walls of stacked stone or packed earth, slope roof to shed rain. Door faces morning sun, window opposite for air flow.".to_string(),
                ingredients_required: vec!["Stone".to_string(), "Clay".to_string(), "Wood".to_string(), "Thatch".to_string()],
                tools_required: vec!["Digging Tools".to_string(), "Stone Working Tools".to_string(), "Measuring Ropes".to_string()],
                skill_requirements: HashMap::from([("Construction".to_string(), 20), ("Engineering".to_string(), 15)]),
            },
            AncientKnowledge {
                title: "The Arch and the Dome".to_string(),
                knowledge_type: KnowledgeType::Architecture,
                difficulty_level: 5,
                prerequisites: vec!["Foundations of Permanent Shelter".to_string(), "Mathematics of Building".to_string()],
                instructions: "The arch distributes weight through curved stone, allowing spans greater than any beam. Build upon wooden framework, place keystone last. The dome extends this principle skyward, requiring perfect balance and patient construction. These forms enable great halls and soaring temples.".to_string(),
                ingredients_required: vec!["Precisely Cut Stone".to_string(), "Strong Mortar".to_string(), "Wooden Framework".to_string()],
                tools_required: vec!["Stone Cutting Tools".to_string(), "Measuring Instruments".to_string(), "Lifting Equipment".to_string()],
                skill_requirements: HashMap::from([("Advanced Construction".to_string(), 40), ("Mathematics".to_string(), 30), ("Engineering".to_string(), 35)]),
            },
        ];

        self.knowledge_database.insert(RunestoneType::ArchitectureStone, architecture_knowledge);
    }

    /// Agricultural knowledge for sustainable food production
    fn initialize_agriculture_knowledge(&mut self) {
        let agriculture_knowledge = vec![
            AncientKnowledge {
                title: "Cultivation of the Soil".to_string(),
                knowledge_type: KnowledgeType::Agriculture,
                difficulty_level: 1,
                prerequisites: vec![],
                instructions: "The earth gives life when properly tended. Clear land of weeds and stones, break soil with digging stick or plow. Plant seeds when the moon waxes full, tend with water and removing of weeds. Harvest when fruits are full and grains golden. Save the best seeds for next season's planting.".to_string(),
                ingredients_required: vec!["Seeds".to_string(), "Water".to_string(), "Fertile Soil".to_string()],
                tools_required: vec!["Digging Stick".to_string(), "Watering Vessels".to_string(), "Harvesting Tools".to_string()],
                skill_requirements: HashMap::from([("Agriculture".to_string(), 15), ("Plant Knowledge".to_string(), 10)]),
            },
            AncientKnowledge {
                title: "Domestication of Beasts".to_string(),
                knowledge_type: KnowledgeType::Agriculture,
                difficulty_level: 3,
                prerequisites: vec!["Cultivation of the Soil".to_string()],
                instructions: "Wild creatures may become partners in the work of living. Begin with the young, feed them regularly, speak gently, move slowly. Sheep give wool and milk, cattle provide strength and meat, chickens lay eggs. Build secure pens and provide shelter from storms and predators.".to_string(),
                ingredients_required: vec!["Young Animals".to_string(), "Feed".to_string(), "Fencing Materials".to_string()],
                tools_required: vec!["Animal Enclosures".to_string(), "Feeding Equipment".to_string(), "Grooming Tools".to_string()],
                skill_requirements: HashMap::from([("Animal Handling".to_string(), 25), ("Construction".to_string(), 15)]),
            },
        ];

        self.knowledge_database.insert(RunestoneType::AgricultureStone, agriculture_knowledge);
    }

    /// Currency and trade knowledge for economic development
    fn initialize_currency_knowledge(&mut self) {
        let currency_knowledge = vec![
            AncientKnowledge {
                title: "The Value of Exchange".to_string(),
                knowledge_type: KnowledgeType::Currency,
                difficulty_level: 3,
                prerequisites: vec!["Basic Crafting".to_string(), "Social Organization".to_string()],
                instructions: "When communities grow large, direct exchange of goods becomes cumbersome. Establish common measures: a day's grain, a good knife, a strong pot. Create tokens of lasting materials to represent these values. All must agree on worth, and tokens must be difficult to counterfeit.".to_string(),
                ingredients_required: vec!["Durable Materials".to_string(), "Marking Tools".to_string(), "Standard Measures".to_string()],
                tools_required: vec!["Scales".to_string(), "Minting Equipment".to_string(), "Record Keeping Materials".to_string()],
                skill_requirements: HashMap::from([("Mathematics".to_string(), 20), ("Economics".to_string(), 25), ("Leadership".to_string(), 30)]),
            },
            AncientKnowledge {
                title: "Banking and Credit".to_string(),
                knowledge_type: KnowledgeType::Currency,
                difficulty_level: 6,
                prerequisites: vec!["The Value of Exchange".to_string(), "Written Records".to_string()],
                instructions: "Wealth may be stored and lent to enable greater projects. Establish houses of trust where coin may be deposited safely. Lend to worthy ventures, charging interest for the service and risk. Keep careful records of all transactions. The flow of money enables the growth of civilization.".to_string(),
                ingredients_required: vec!["Secure Vaults".to_string(), "Record Books".to_string(), "Trusted Guardians".to_string()],
                tools_required: vec!["Accounting Systems".to_string(), "Security Measures".to_string(), "Communication Networks".to_string()],
                skill_requirements: HashMap::from([("Mathematics".to_string(), 35), ("Economics".to_string(), 40), ("Trust Management".to_string(), 45)]),
            },
        ];

        self.knowledge_database.insert(RunestoneType::CurrencyStone, currency_knowledge);
    }

    /// Writing and communication knowledge
    fn initialize_writing_knowledge(&mut self) {
        let writing_knowledge = vec![
            AncientKnowledge {
                title: "Making of Writing Materials".to_string(),
                knowledge_type: KnowledgeType::Writing,
                difficulty_level: 2,
                prerequisites: vec!["Basic Crafting".to_string()],
                instructions: "To preserve words beyond memory, create materials that hold marks. Pound bark into smooth sheets, or scrape animal skins thin and white. Make ink from berries, soot, or crushed minerals mixed with tree sap. Craft pens from sharpened reeds or bird feathers.".to_string(),
                ingredients_required: vec!["Bark or Animal Skins".to_string(), "Natural Pigments".to_string(), "Binding Agents".to_string()],
                tools_required: vec!["Pounding Tools".to_string(), "Scraping Knives".to_string(), "Mixing Vessels".to_string()],
                skill_requirements: HashMap::from([("Crafting".to_string(), 20), ("Chemistry".to_string(), 15)]),
            },
            AncientKnowledge {
                title: "The Art of Symbol and Script".to_string(),
                knowledge_type: KnowledgeType::Writing,
                difficulty_level: 4,
                prerequisites: vec!["Making of Writing Materials".to_string()],
                instructions: "Begin with pictures that represent ideas: sun for day, water for life, crossed lines for meeting. Gradually simplify these marks for speed of writing. Assign sounds to symbols so any word may be recorded. Teach others this system so knowledge may spread and endure.".to_string(),
                ingredients_required: vec!["Writing Materials".to_string(), "Teaching Spaces".to_string()],
                tools_required: vec!["Writing Implements".to_string(), "Practice Surfaces".to_string()],
                skill_requirements: HashMap::from([("Language".to_string(), 30), ("Teaching".to_string(), 25), ("Art".to_string(), 20)]),
            },
        ];

        self.knowledge_database.insert(RunestoneType::WritingStone, writing_knowledge);
    }

    /// Advanced technology knowledge
    fn initialize_technology_knowledge(&mut self) {
        let technology_knowledge = vec![
            AncientKnowledge {
                title: "Mechanisms of Leverage and Motion".to_string(),
                knowledge_type: KnowledgeType::Technology,
                difficulty_level: 4,
                prerequisites: vec!["Advanced Crafting".to_string(), "Mathematics".to_string()],
                instructions: "Small forces may be multiplied through clever construction. The lever amplifies strength, the wheel reduces effort, the gear changes direction of force. Combine these principles to create machines that perform great work with less human labor. Water and wind may also drive mechanisms.".to_string(),
                ingredients_required: vec!["Worked Metal".to_string(), "Precise Wood Work".to_string(), "Fastening Materials".to_string()],
                tools_required: vec!["Precision Tools".to_string(), "Assembly Space".to_string(), "Testing Equipment".to_string()],
                skill_requirements: HashMap::from([("Engineering".to_string(), 35), ("Mathematics".to_string(), 30), ("Crafting".to_string(), 40)]),
            },
            AncientKnowledge {
                title: "The Printing of Many Copies".to_string(),
                knowledge_type: KnowledgeType::Technology,
                difficulty_level: 7,
                prerequisites: vec!["The Art of Symbol and Script".to_string(), "Mechanisms of Leverage and Motion".to_string()],
                instructions: "To spread knowledge quickly, create devices that make many identical copies of writing. Carve letters in reverse upon blocks of wood or metal. Apply ink and press firmly upon paper. Arrange moveable letters for different texts. This art multiplies the power of learning across all peoples.".to_string(),
                ingredients_required: vec!["Type Metal".to_string(), "Quality Ink".to_string(), "Paper".to_string(), "Precise Machinery".to_string()],
                tools_required: vec!["Type Casting Equipment".to_string(), "Printing Press".to_string(), "Typesetting Tools".to_string()],
                skill_requirements: HashMap::from([("Advanced Engineering".to_string(), 50), ("Metallurgy".to_string(), 45), ("Writing".to_string(), 40)]),
            },
        ];

        self.knowledge_database.insert(RunestoneType::TechnologyStone, technology_knowledge);
    }

    /// Social organization knowledge
    fn initialize_social_knowledge(&mut self) {
        let social_knowledge = vec![
            AncientKnowledge {
                title: "Laws and Justice".to_string(),
                knowledge_type: KnowledgeType::Social,
                difficulty_level: 3,
                prerequisites: vec!["Basic Writing".to_string()],
                instructions: "When groups grow large, agreements must be made about acceptable behavior. Establish clear rules for common disputes: ownership of goods, harm between persons, obligations to community. Choose wise judges who will listen fairly to all sides. Record important decisions for future guidance.".to_string(),
                ingredients_required: vec!["Writing Materials".to_string(), "Meeting Spaces".to_string()],
                tools_required: vec!["Record Keeping".to_string(), "Communication Systems".to_string()],
                skill_requirements: HashMap::from([("Leadership".to_string(), 30), ("Writing".to_string(), 25), ("Wisdom".to_string(), 35)]),
            },
            AncientKnowledge {
                title: "Specialization of Labor".to_string(),
                knowledge_type: KnowledgeType::Social,
                difficulty_level: 4,
                prerequisites: vec!["Laws and Justice".to_string()],
                instructions: "Prosperity grows when each person focuses on what they do best. Some become skilled farmers, others master crafts, still others lead or teach. Organize exchange of goods and services so all needs are met. This division of work enables advancement impossible for any individual alone.".to_string(),
                ingredients_required: vec!["Diverse Skills".to_string(), "Trade Networks".to_string()],
                tools_required: vec!["Guild Systems".to_string(), "Market Spaces".to_string()],
                skill_requirements: HashMap::from([("Organization".to_string(), 35), ("Economics".to_string(), 30), ("Leadership".to_string(), 40)]),
            },
        ];

        self.knowledge_database.insert(RunestoneType::SocialStone, social_knowledge);
    }

    /// Spiritual and religious knowledge
    fn initialize_spiritual_knowledge(&mut self) {
        let spiritual_knowledge = vec![
            AncientKnowledge {
                title: "Reverence for the Sacred".to_string(),
                knowledge_type: KnowledgeType::Spiritual,
                difficulty_level: 1,
                prerequisites: vec![],
                instructions: "All life springs from forces greater than ourselves. Honor the sun that gives warmth, the earth that provides food, the waters that sustain life. Create rituals of gratitude and respect. Seek wisdom from dreams and visions. The sacred connects all things in web of meaning.".to_string(),
                ingredients_required: vec!["Natural Materials".to_string(), "Sacred Spaces".to_string()],
                tools_required: vec!["Ritual Objects".to_string(), "Ceremonial Spaces".to_string()],
                skill_requirements: HashMap::from([("Spirituality".to_string(), 20), ("Ritual Knowledge".to_string(), 15)]),
            },
            AncientKnowledge {
                title: "Communication with Spirits".to_string(),
                knowledge_type: KnowledgeType::Spiritual,
                difficulty_level: 5,
                prerequisites: vec!["Reverence for the Sacred".to_string(), "Basic Magic".to_string()],
                instructions: "The world teems with spirits of place and purpose. Approach with respect and proper offerings. Speak in ancient words and gesture. Some spirits guard knowledge, others guide the lost, still others protect from harm. Build relationships slowly through consistent honor and service.".to_string(),
                ingredients_required: vec!["Sacred Offerings".to_string(), "Ritual Components".to_string()],
                tools_required: vec!["Ceremonial Tools".to_string(), "Spirit Vessels".to_string()],
                skill_requirements: HashMap::from([("Spirit Communication".to_string(), 40), ("Magic".to_string(), 35), ("Wisdom".to_string(), 45)]),
            },
        ];

        self.knowledge_database.insert(RunestoneType::SpiritualStone, spiritual_knowledge);
    }

    /// Get random knowledge for a runestone type
    pub fn get_knowledge_for_runestone(&self, runestone_type: &RunestoneType) -> Vec<AncientKnowledge> {
        if let Some(knowledge_list) = self.knowledge_database.get(runestone_type) {
            // For now, return all knowledge. In practice, might select random subset
            knowledge_list.clone()
        } else {
            Vec::new()
        }
    }

    /// Check if NPC has prerequisites for specific knowledge
    pub fn check_prerequisites(&self, npc_knowledge: &[String], required_prerequisites: &[String]) -> bool {
        required_prerequisites.iter().all(|prereq| npc_knowledge.contains(prereq))
    }

    /// Generate activation requirements based on knowledge difficulty
    pub fn generate_activation_requirements(&self, knowledge: &AncientKnowledge) -> Vec<ActivationRequirement> {
        let mut requirements = vec![
            ActivationRequirement::MinimumIntelligence(knowledge.difficulty_level * 10),
        ];

        // Add skill requirements
        for (skill, level) in &knowledge.skill_requirements {
            requirements.push(ActivationRequirement::RequiredSkill(skill.clone(), *level));
        }

        // Add prerequisite knowledge requirements
        for prereq in &knowledge.prerequisites {
            requirements.push(ActivationRequirement::PreviousKnowledge(prereq.clone()));
        }

        // Special requirements for advanced knowledge
        if knowledge.difficulty_level >= 5 {
            requirements.push(ActivationRequirement::GroupSize(2)); // Requires collaboration
        }

        if knowledge.difficulty_level >= 7 {
            requirements.push(ActivationRequirement::TimeOfDay("Dawn or Dusk".to_string()));
        }

        requirements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runestone_library_creation() {
        let library = RunestoneLibrary::new();
        assert!(!library.knowledge_database.is_empty());
        
        // Check that all runestone types have knowledge
        for runestone_type in [
            RunestoneType::CraftingStone,
            RunestoneType::MagicStone,
            RunestoneType::AlchemyStone,
            RunestoneType::ArchitectureStone,
            RunestoneType::AgricultureStone,
            RunestoneType::CurrencyStone,
            RunestoneType::WritingStone,
            RunestoneType::TechnologyStone,
            RunestoneType::SocialStone,
            RunestoneType::SpiritualStone,
        ] {
            assert!(library.knowledge_database.contains_key(&runestone_type));
        }
    }

    #[test]
    fn test_knowledge_prerequisites() {
        let library = RunestoneLibrary::new();
        let npc_knowledge = vec!["The Art of Tool Making".to_string()];
        let prerequisites = vec!["The Art of Tool Making".to_string()];
        
        assert!(library.check_prerequisites(&npc_knowledge, &prerequisites));
    }
}
