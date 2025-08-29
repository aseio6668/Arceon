use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use arceon_core::entities::being::Race as CoreRace;

/// Comprehensive race database for the Arceon universe
/// All races are divine, enlightened humanoid bipeds inspired by mythology and folklore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceDatabase {
    pub races: HashMap<String, RaceDefinition>,
    pub planet_distributions: HashMap<String, Vec<String>>, // Planet -> Race names
}

/// Detailed race definition with characteristics, abilities, and lore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceDefinition {
    pub id: Uuid,
    pub name: String,
    pub plural_name: String,
    pub category: RaceCategory,
    pub origin_planet: String,
    pub subspecies_of: Option<String>, // If this is a variant of another race
    
    // Physical characteristics
    pub appearance: RaceAppearance,
    pub lifespan: RaceLifespan,
    pub size_category: SizeCategory,
    
    // Cultural and spiritual traits
    pub cultural_traits: Vec<CulturalTrait>,
    pub spiritual_alignment: SpiritualAlignment,
    pub divine_blessing: DivineBlessings,
    
    // Abilities and affinities
    pub natural_abilities: Vec<NaturalAbility>,
    pub elemental_affinities: Vec<ElementalAffinity>,
    pub magical_inclinations: Vec<MagicalInclination>,
    
    // Social characteristics
    pub social_structure: SocialStructure,
    pub preferred_environments: Vec<Environment>,
    pub architectural_style: ArchitecturalStyle,
    
    // Language and communication
    pub native_languages: Vec<String>,
    pub communication_style: CommunicationStyle,
    
    // Lore and background
    pub origin_story: String,
    pub cultural_values: Vec<String>,
    pub legendary_figures: Vec<String>,
    pub relationships_with_other_races: HashMap<String, RelationshipType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RaceCategory {
    // Human variants and related
    Human, HumanHybrid, HumanDescendant,
    
    // Classical fantasy races and variants
    Elven, ElvenHybrid, Dwarven, DwarvenHybrid, Halfling, HalflingHybrid,
    Gnomish, GnomishHybrid, Orcish, OrcishHybrid,
    
    // Draconic heritage
    Dragonborn, DragonkinHybrid, Dragonblood,
    
    // Feline heritage (divine cat-people)
    Feline, FelineHybrid, LionkinHybrid, TigerkinHybrid, PantherkinHybrid,
    
    // Ocean and aquatic heritage
    Aquatic, MerfolkHybrid, TritonHybrid, OceanicHybrid,
    
    // Mythological creatures (humanoid forms)
    Phoenix, PhoenixHybrid, GriffinHybrid, UnicornHybrid,
    
    // Elemental touched
    FireTouched, WaterTouched, EarthTouched, AirTouched, IceTouched,
    
    // Celestial and divine
    Celestial, AngelicHybrid, Starborn, Moonborn, Sunborn,
    
    // Forest and nature
    Sylvan, TreekinHybrid, FlowerkinHybrid, BeastkinHybrid,
    
    // Crystal and gem
    CrystalkinHybrid, GemTouched, MineralHybrid,
    
    // Shadow and twilight (but divine)
    TwilightTouched, ShadowDancer, NightbloomHybrid,
    
    // Planar and dimensional
    PlanarHybrid, VoidTouched, DimensionalHybrid,
    
    // Additional race categories for comprehensive system (avoiding duplicates)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceAppearance {
    pub height_range: (f32, f32), // meters
    pub build: BodyBuild,
    pub skin_tones: Vec<String>,
    pub hair_colors: Vec<String>,
    pub eye_colors: Vec<String>,
    pub distinctive_features: Vec<String>,
    pub markings_or_patterns: Vec<String>,
    pub secondary_features: Vec<String>, // Wings, tails, horns, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyBuild {
    Petite, Slender, Athletic, Robust, Statuesque, Ethereal, Graceful, Powerful,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SizeCategory {
    Tiny, Small, Medium, Large, // Standard D&D-style sizes but all humanoid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceLifespan {
    pub maturity_age: u32,
    pub average_lifespan: u32,
    pub maximum_lifespan: u32,
    pub aging_rate: AgingRate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgingRate {
    Rapid, Normal, Slow, Eternal, Cyclical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CulturalTrait {
    Scholarly, Artistic, Warrior, Peaceful, Nomadic, Traditionalist, 
    Progressive, Mystical, Communal, Independent, Ritualistic, Pragmatic,
    Harmonious, Competitive, Protective, Exploratory, Meditative, Artisan,
    Playful, Survivor, Enduring, Wise, Stable, Helpful, Free, Adaptive,
    Detached, Ancient, Explorer, Joyful,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpiritualAlignment {
    Light, Balance, Harmony, Wisdom, Courage, Compassion, Justice, 
    Beauty, Truth, Growth, Protection, Renewal, Unity, Transcendence,
    Grace, Joy, Flow, Perfection, Endurance, Serenity, Winter, Freedom,
    Mirage, Illusion, Storm, Wind, Sky, Stability, Grounding, Void,
    Cosmos, Universe, Creation, Discovery, Connection, Clarity,
    Perseverance, Cycle, Survival,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivineBlessings {
    pub primary_blessing: String,
    pub secondary_blessings: Vec<String>,
    pub patron_deity: Option<String>,
    pub sacred_symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NaturalAbility {
    EnhancedSenses(String), // Which sense
    NaturalFlight, NaturalSwim, NaturalClimb,
    Telepathy, Empathy, PsychicResilience,
    ShapeMinorChanges, Camouflage, NaturalArmor,
    LifeDetection, MagicDetection, DangerSense,
    NaturalHealing, PoisonResistance, DiseaseResistance,
    TemperatureResistance, PressureAdaptation,
    NightVision, TrueSeeing, Darkvision,
    NaturalWeapons(String), // Claws, fangs, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementalAffinity {
    Fire, Water, Earth, Air, Ice, Lightning, Light, Shadow,
    Nature, Crystal, Metal, Sound, Gravity, Time, Space, Moon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MagicalInclination {
    Divination, Enchantment, Evocation, Illusion, Transmutation,
    Conjuration, Necromancy(bool), // bool: benevolent necromancy only
    Healing, Warding, Blessing, Nature, Elemental(ElementalAffinity),
    Dimensional, Temporal, Psionic, Divine, Primal, Light, Telepathic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialStructure {
    Tribal, Clan, Kingdom, Democracy, Council, Theocracy,
    Meritocracy, Collective, Hierarchy, Egalitarian,
    Nomadic, CityStates, Federation, Empire, Independent, Ancient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Forest, Plains, Mountains, Desert, Ocean, River, Lake,
    Underground, Floating, Volcanic, Crystal, Ice, Jungle,
    Swamp, Canyon, Cliff, Beach, Urban, Rural, Wilderness, Hills,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchitecturalStyle {
    Organic, Crystal, Stone, Wood, Metal, Floating,
    Underground, Cliff, Tree, Coral, Ice, Magical,
    Geometric, Flowing, Towering, Harmonious, Dimensional, Cosmic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Verbal, Telepathic, Harmonic, Empathic, Gestural,
    Luminous, Aromatic, Tactile, Combined(Vec<CommunicationStyle>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Allied, Friendly, Neutral, Cautious, Rival, Ancient, Protective,
    Complementary, Competitive, Respectful, Mysterious,
}

impl RaceDatabase {
    pub fn new() -> Self {
        Self {
            races: HashMap::new(),
            planet_distributions: HashMap::new(),
        }
    }
    
    /// Generate the complete database of 500 races distributed across all 13 planets
    pub fn generate_complete_database(&mut self) -> Result<(), String> {
        // Clear existing data
        self.races.clear();
        self.planet_distributions.clear();
        
        // Generate races for each planet
        self.generate_espan_races()?;          // ~80 races (most diverse)
        self.generate_verdania_races()?;       // ~50 races (forest world)
        self.generate_pyros_races()?;          // ~40 races (volcanic world)
        self.generate_aquatica_races()?;       // ~45 races (ocean world)
        self.generate_lumina_races()?;         // ~35 races (crystal world)
        self.generate_umbra_races()?;          // ~40 races (underground)
        self.generate_glacialis_races()?;      // ~30 races (ice world)
        self.generate_tempest_races()?;        // ~30 races (desert world)
        self.generate_seraphim_races()?;       // ~25 races (floating world)
        self.generate_nexus_races()?;          // ~35 races (terrestrial)
        self.generate_calypso_races()?;        // ~40 races (mysterious floating)
        self.generate_shared_races()?;         // ~50 races found on multiple planets
        
        Ok(())
    }
    
    /// Generate races for Espan (the most diverse starting world)
    fn generate_espan_races(&mut self) -> Result<(), String> {
        let planet_name = "Espan".to_string();
        let mut races = Vec::new();
        
        // Standard fantasy races (enhanced divine versions)
        races.extend(self.create_human_variants(&planet_name, 15));
        races.extend(self.create_elven_variants(&planet_name, 12));
        races.extend(self.create_dwarven_variants(&planet_name, 10));
        races.extend(self.create_halfling_variants(&planet_name, 8));
        races.extend(self.create_gnomish_variants(&planet_name, 8));
        races.extend(self.create_orcish_variants(&planet_name, 6));
        races.extend(self.create_dragonborn_variants(&planet_name, 8));
        races.extend(self.create_feline_variants(&planet_name, 10));
        races.extend(self.create_mixed_heritage_races(&planet_name, 3));
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    
    /// Create human variants - the most adaptable race
    fn create_human_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        let variants = vec![
            ("Alderheart Humans", "Noble humans from the trading capital", vec!["Golden", "Fair", "Olive"], vec!["Brown", "Black", "Auburn"]),
            ("Goldenvale Humans", "Traditional humans from agricultural regions", vec!["Tan", "Fair", "Ruddy"], vec!["Blonde", "Brown", "Red"]),
            ("Mountain Humans", "Hardy humans adapted to high altitudes", vec!["Bronzed", "Weathered"], vec!["Dark Brown", "Black"]),
            ("Forest Humans", "Humans living in harmony with woodland elves", vec!["Olive", "Tanned"], vec!["Green-tinged Brown", "Dark Blonde"]),
            ("Coastal Humans", "Maritime humans with sea-blessed heritage", vec!["Salt-kissed Tan", "Deep Bronze"], vec!["Sea Blue", "Dark Brown"]),
            ("Desert Humans", "Sun-blessed humans from arid regions", vec!["Deep Bronze", "Golden Brown"], vec!["Black", "Dark Brown"]),
            ("Crossroads Humans", "Cosmopolitan humans from trading posts", vec!["Mixed heritage - all tones"], vec!["All colors"]),
            ("Stargazer Humans", "Humans blessed with celestial insight", vec!["Pale with silver undertones"], vec!["Silver", "Platinum", "Star-white"]),
            ("Emberhart Humans", "Fire-touched humans near volcanic regions", vec!["Warm golden", "Copper-tinged"], vec!["Fiery red", "Copper", "Auburn"]),
            ("Frostborn Humans", "Cold-adapted humans from mountain peaks", vec!["Pale with blue undertones"], vec!["Ice blue", "White", "Silver"]),
            ("Windwalker Humans", "Air-blessed humans from high plateaus", vec!["Light with ethereal quality"], vec!["Platinum blonde", "Silver-white"]),
            ("Earthward Humans", "Stone-blessed humans from deep valleys", vec!["Earth-toned", "Rocky gray"], vec!["Dark brown", "Stone gray"]),
            ("Twilight Humans", "Shadow-blessed but divine humans", vec!["Dusky", "Twilight purple"], vec!["Deep purple", "Midnight blue"]),
            ("Dawnbringer Humans", "Light-blessed humans from eastern lands", vec!["Radiant golden"], vec!["Golden", "Light blonde", "Radiant white"]),
            ("Voidwalker Humans", "Space-touched humans with cosmic insight", vec!["Deep blue-black with star freckles"], vec!["Cosmic blue", "Void black"]),
        ];
        
        variants.into_iter().take(count).enumerate().map(|(i, (name, desc, skin, hair))| {
            RaceDefinition {
                id: Uuid::new_v4(),
                name: name.to_string(),
                plural_name: name.to_string(),
                category: RaceCategory::Human,
                origin_planet: planet.to_string(),
                subspecies_of: Some("Human".to_string()),
                appearance: RaceAppearance {
                    height_range: (1.6, 1.9),
                    build: BodyBuild::Athletic,
                    skin_tones: skin.into_iter().map(|s| s.to_string()).collect(),
                    hair_colors: hair.into_iter().map(|h| h.to_string()).collect(),
                    eye_colors: vec!["Brown".to_string(), "Blue".to_string(), "Green".to_string()],
                    distinctive_features: vec!["Adaptive physique".to_string()],
                    markings_or_patterns: vec!["Cultural tattoos".to_string()],
                    secondary_features: Vec::new(),
                },
                lifespan: RaceLifespan {
                    maturity_age: 18,
                    average_lifespan: 80,
                    maximum_lifespan: 120,
                    aging_rate: AgingRate::Normal,
                },
                size_category: SizeCategory::Medium,
                cultural_traits: vec![CulturalTrait::Pragmatic, CulturalTrait::Progressive],
                spiritual_alignment: SpiritualAlignment::Balance,
                divine_blessing: DivineBlessings {
                    primary_blessing: "Adaptability".to_string(),
                    secondary_blessings: vec!["Resilience".to_string(), "Innovation".to_string()],
                    patron_deity: Some("The Great Balance".to_string()),
                    sacred_symbols: vec!["Balanced scales".to_string()],
                },
                natural_abilities: vec![NaturalAbility::NaturalHealing],
                elemental_affinities: Vec::new(),
                magical_inclinations: vec![MagicalInclination::Enchantment],
                social_structure: SocialStructure::Kingdom,
                preferred_environments: vec![Environment::Plains, Environment::Urban],
                architectural_style: ArchitecturalStyle::Stone,
                native_languages: vec!["Common".to_string(), format!("{} Dialect", name)],
                communication_style: CommunicationStyle::Verbal,
                origin_story: desc.to_string(),
                cultural_values: vec!["Adaptability".to_string(), "Progress".to_string()],
                legendary_figures: vec!["The First King".to_string()],
                relationships_with_other_races: HashMap::new(),
            }
        }).collect()
    }
    
    /// Create elven variants - graceful and magical
    fn create_elven_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        let variants = vec![
            ("Silverleaf Elves", "Classical forest elves with silver-touched hair", vec!["Pale silver"], vec!["Silver", "Platinum"]),
            ("Moonlight Elves", "Nocturnal elves blessed by lunar magic", vec!["Luminous pale"], vec!["Silver-white", "Moon-blue"]),
            ("Sunrise Elves", "Dawn-blessed elves with golden features", vec!["Golden-touched"], vec!["Golden", "Sunrise orange"]),
            ("Star Elves", "Celestial elves connected to the cosmos", vec!["Pale with starlight shimmer"], vec!["Starlight silver", "Cosmic blue"]),
            ("Autumn Elves", "Seasonal elves with changing appearances", vec!["Warm amber"], vec!["Auburn", "Copper", "Gold"]),
            ("Spring Elves", "Growth-blessed elves with verdant features", vec!["Green-tinged"], vec!["Moss green", "Leaf brown"]),
            ("Crystal Elves", "Gem-touched elves with crystalline features", vec!["Translucent with crystal highlights"], vec!["Crystal clear", "Prism-colored"]),
            ("Wind Elves", "Air-blessed elves from mountain peaks", vec!["Ethereal pale"], vec!["Silver-white", "Sky blue"]),
            ("Water Elves", "Lake and river dwelling elves", vec!["Blue-tinged"], vec!["Aqua blue", "Sea green"]),
            ("Shadow Elves", "Twilight-dwelling but divine elves", vec!["Dusky with silver highlights"], vec!["Deep purple", "Silver-streaked black"]),
            ("Fire Elves", "Flame-blessed elves with warm features", vec!["Warm copper"], vec!["Flame red", "Copper"]),
            ("Ice Elves", "Cold-adapted elves from northern regions", vec!["Ice-pale"], vec!["Ice blue", "Frost white"]),
        ];
        
        variants.into_iter().take(count).enumerate().map(|(i, (name, desc, skin, hair))| {
            RaceDefinition {
                id: Uuid::new_v4(),
                name: name.to_string(),
                plural_name: name.to_string(),
                category: RaceCategory::Elven,
                origin_planet: planet.to_string(),
                subspecies_of: Some("Elf".to_string()),
                appearance: RaceAppearance {
                    height_range: (1.7, 1.95),
                    build: BodyBuild::Graceful,
                    skin_tones: skin.into_iter().map(|s| s.to_string()).collect(),
                    hair_colors: hair.into_iter().map(|h| h.to_string()).collect(),
                    eye_colors: vec!["Silver".to_string(), "Gold".to_string(), "Violet".to_string()],
                    distinctive_features: vec!["Pointed ears".to_string(), "Ethereal beauty".to_string()],
                    markings_or_patterns: vec!["Natural luminescence".to_string()],
                    secondary_features: Vec::new(),
                },
                lifespan: RaceLifespan {
                    maturity_age: 100,
                    average_lifespan: 750,
                    maximum_lifespan: 1200,
                    aging_rate: AgingRate::Slow,
                },
                size_category: SizeCategory::Medium,
                cultural_traits: vec![CulturalTrait::Artistic, CulturalTrait::Mystical],
                spiritual_alignment: SpiritualAlignment::Harmony,
                divine_blessing: DivineBlessings {
                    primary_blessing: "Eternal Grace".to_string(),
                    secondary_blessings: vec!["Magical Insight".to_string(), "Natural Harmony".to_string()],
                    patron_deity: Some("The Eternal Moon".to_string()),
                    sacred_symbols: vec!["Silver tree".to_string()],
                },
                natural_abilities: vec![NaturalAbility::MagicDetection, NaturalAbility::EnhancedSenses("Hearing".to_string())],
                elemental_affinities: vec![ElementalAffinity::Nature],
                magical_inclinations: vec![MagicalInclination::Enchantment, MagicalInclination::Illusion],
                social_structure: SocialStructure::Council,
                preferred_environments: vec![Environment::Forest, Environment::Urban],
                architectural_style: ArchitecturalStyle::Organic,
                native_languages: vec!["Elvish".to_string(), format!("{} Dialect", name)],
                communication_style: CommunicationStyle::Combined(vec![CommunicationStyle::Verbal, CommunicationStyle::Telepathic]),
                origin_story: desc.to_string(),
                cultural_values: vec!["Harmony".to_string(), "Beauty".to_string(), "Wisdom".to_string()],
                legendary_figures: vec!["The First Singer".to_string()],
                relationships_with_other_races: HashMap::new(),
            }
        }).collect()
    }

    /// Create dwarven variants - masterful crafters and mountain dwellers
    fn create_dwarven_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        let variants = vec![
            ("Ironforge Dwarves", "Traditional mountain dwarves master smiths", vec!["Bronzed", "Ruddy"], vec!["Brown", "Black", "Gray"]),
            ("Goldbeard Dwarves", "Wealthy dwarves blessed with golden features", vec!["Golden-tinged"], vec!["Gold", "Copper"]),
            ("Stoneward Dwarves", "Deep-dwelling dwarves attuned to earth magic", vec!["Gray-toned", "Stone-like"], vec!["Stone gray", "Silver"]),
            ("Flame Dwarves", "Forge-blessed dwarves with fire resistance", vec!["Copper", "Bronze"], vec!["Flame red", "Copper"]),
            ("Crystal Dwarves", "Gem-touched dwarves with crystalline beards", vec!["Translucent-tinged"], vec!["Crystal clear", "Diamond"]),
            ("Thunder Dwarves", "Storm-blessed dwarves from high peaks", vec!["Storm gray"], vec!["Thunder white", "Lightning blue"]),
            ("Mithril Dwarves", "Rare metal blessed dwarves", vec!["Silver-touched"], vec!["Mithril silver", "Platinum"]),
            ("Deepdelver Dwarves", "Underground dwelling dwarves", vec!["Pale from depths"], vec!["Dark brown", "Black"]),
            ("Gemcutter Dwarves", "Artisan dwarves specializing in precious stones", vec!["Multi-hued like gems"], vec!["Ruby red", "Sapphire blue"]),
            ("Runesmith Dwarves", "Magic-blessed crafters of runic items", vec!["Rune-marked"], vec!["Runic blue", "Mystical silver"]),
        ];
        
        variants.into_iter().take(count).map(|(name, desc, skin, hair)| {
            self.create_base_race(
                name, desc, RaceCategory::Dwarven, planet,
                Some("Dwarf".to_string()),
                (1.3, 1.5), BodyBuild::Robust,
                skin, hair,
                vec!["Brown".to_string(), "Black".to_string(), "Gold".to_string()],
                120, 300, 500, AgingRate::Slow,
                vec![CulturalTrait::Artisan, CulturalTrait::Traditionalist],
                SpiritualAlignment::Justice,
                "Craft Mastery".to_string(),
                vec!["Stone Resilience".to_string(), "Fire Resistance".to_string()],
                vec![NaturalAbility::NaturalArmor, NaturalAbility::DangerSense],
                vec![ElementalAffinity::Earth, ElementalAffinity::Fire],
                vec![MagicalInclination::Enchantment, MagicalInclination::Transmutation],
                SocialStructure::Clan,
                vec![Environment::Mountains, Environment::Underground],
                ArchitecturalStyle::Stone,
                vec!["Dwarvish".to_string()],
            )
        }).collect()
    }
    
    /// Create halfling variants - peaceful and community-focused
    fn create_halfling_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        let variants = vec![
            ("Greenhill Halflings", "Traditional rural halflings", vec!["Warm tan", "Rosy"], vec!["Brown", "Auburn"]),
            ("Riverbend Halflings", "Waterside dwelling halflings", vec!["Fair", "Freckled"], vec!["Light brown", "Blonde"]),
            ("Harvest Halflings", "Agricultural blessed halflings", vec!["Sun-kissed"], vec!["Golden", "Wheat blonde"]),
            ("Wanderlust Halflings", "Travel-blessed nomadic halflings", vec!["Weather-tanned"], vec!["All colors"]),
            ("Feast Halflings", "Culinary blessed halflings", vec!["Well-fed healthy glow"], vec!["Rich brown", "Golden"]),
            ("Song Halflings", "Music-blessed halflings with melodic voices", vec!["Harmonious features"], vec!["Melodic blonde", "Harmonic brown"]),
            ("Garden Halflings", "Nature-blessed halflings", vec!["Green-tinged"], vec!["Moss green", "Flower colors"]),
            ("Comfort Halflings", "Home-blessed halflings", vec!["Warm and welcoming"], vec!["Hearth brown", "Comfort blonde"]),
        ];
        
        variants.into_iter().take(count).map(|(name, desc, skin, hair)| {
            self.create_base_race(
                name, desc, RaceCategory::Halfling, planet,
                Some("Halfling".to_string()),
                (0.9, 1.2), BodyBuild::Petite,
                skin, hair,
                vec!["Brown".to_string(), "Green".to_string(), "Blue".to_string()],
                33, 150, 200, AgingRate::Normal,
                vec![CulturalTrait::Peaceful, CulturalTrait::Communal],
                SpiritualAlignment::Compassion,
                "Natural Luck".to_string(),
                vec!["Community Spirit".to_string(), "Natural Healing".to_string()],
                vec![NaturalAbility::NaturalHealing, NaturalAbility::PoisonResistance],
                vec![ElementalAffinity::Nature],
                vec![MagicalInclination::Healing, MagicalInclination::Nature],
                SocialStructure::Collective,
                vec![Environment::Plains, Environment::Rural],
                ArchitecturalStyle::Organic,
                vec!["Halfling".to_string()],
            )
        }).collect()
    }
    
    /// Create feline variants - graceful cat-people with divine heritage
    fn create_feline_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        let variants = vec![
            ("Lionkin", "Noble lion-featured humanoids", vec!["Golden", "Tawny"], vec!["Golden mane", "Auburn mane"]),
            ("Tigerkin", "Striped tiger-featured humanoids", vec!["Orange with black stripes"], vec!["Orange", "Black-striped"]),
            ("Pantherkin", "Sleek panther-featured humanoids", vec!["Midnight black", "Deep purple"], vec!["Jet black", "Silver-black"]),
            ("Leopardkin", "Spotted leopard-featured humanoids", vec!["Golden with rosettes"], vec!["Spotted gold", "Rosette pattern"]),
            ("Lynxkin", "Tufted ear lynx-featured humanoids", vec!["Silver-gray"], vec!["Silver with tufts", "Gray"]),
            ("Cheetahkin", "Swift cheetah-featured humanoids", vec!["Golden with spots"], vec!["Spotted gold", "Speed-blessed"]),
            ("Snow Leopardkin", "Mountain cat-featured humanoids", vec!["Pale with rosettes"], vec!["Ice white", "Snow-spotted"]),
            ("Ocelotkin", "Jungle cat-featured humanoids", vec!["Dappled gold"], vec!["Forest gold", "Jungle-patterned"]),
            ("Servalkin", "Tall-eared cat-featured humanoids", vec!["Tawny with spots"], vec!["Tawny gold", "Spotted"]),
            ("Caracalkin", "Desert cat-featured humanoids", vec!["Sandy gold"], vec!["Desert gold", "Tufted ears"]),
        ];
        
        variants.into_iter().take(count).map(|(name, desc, skin, hair)| {
            self.create_base_race(
                name, desc, RaceCategory::Feline, planet,
                None,
                (1.6, 1.9), BodyBuild::Athletic,
                skin, hair,
                vec!["Golden".to_string(), "Green".to_string(), "Amber".to_string()],
                25, 120, 180, AgingRate::Normal,
                vec![CulturalTrait::Independent, CulturalTrait::Harmonious],
                SpiritualAlignment::Grace,
                "Feline Grace".to_string(),
                vec!["Night Vision".to_string(), "Perfect Balance".to_string()],
                vec![NaturalAbility::NightVision, NaturalAbility::EnhancedSenses("Hearing".to_string()), NaturalAbility::NaturalClimb],
                vec![ElementalAffinity::Shadow, ElementalAffinity::Moon],
                vec![MagicalInclination::Illusion, MagicalInclination::Divination],
                SocialStructure::Tribal,
                vec![Environment::Forest, Environment::Plains],
                ArchitecturalStyle::Flowing,
                vec!["Feline".to_string()],
            )
        }).collect()
    }

    /// Create aquatic variants - water-blessed humanoids
    fn create_aquatic_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        let variants = vec![
            ("Coral Merfolk", "Reef-dwelling aquatic humanoids", vec!["Coral-tinged", "Sea-green"], vec!["Seaweed green", "Coral pink"]),
            ("Deep Sea Tritons", "Ocean-depth dwelling humanoids", vec!["Deep blue", "Bioluminescent"], vec!["Deep blue", "Phosphorescent"]),
            ("Pearl Divers", "Surface-dwelling aquatic humanoids", vec!["Pearl-lustrous"], vec!["Pearl white", "Mother of pearl"]),
            ("Tide Dancers", "Coastal aquatic humanoids", vec!["Wave-pattern skin"], vec!["Foam white", "Tide blue"]),
            ("Kelp Shepherds", "Forest-sea dwelling humanoids", vec!["Kelp green"], vec!["Forest green", "Kelp brown"]),
            ("Storm Swimmers", "Tempest-blessed aquatic humanoids", vec!["Storm gray"], vec!["Lightning white", "Thunder gray"]),
            ("Ice swimmers", "Arctic aquatic humanoids", vec!["Ice blue", "Frost white"], vec!["Ice white", "Glacier blue"]),
            ("River Runners", "Freshwater aquatic humanoids", vec!["Clear-toned"], vec!["River brown", "Spring clear"]),
        ];
        
        variants.into_iter().take(count).map(|(name, desc, skin, hair)| {
            self.create_base_race(
                name, desc, RaceCategory::Aquatic, planet,
                None,
                (1.7, 2.0), BodyBuild::Athletic,
                skin, hair,
                vec!["Blue".to_string(), "Green".to_string(), "Pearl".to_string()],
                30, 200, 300, AgingRate::Slow,
                vec![CulturalTrait::Harmonious, CulturalTrait::Peaceful],
                SpiritualAlignment::Flow,
                "Ocean's Blessing".to_string(),
                vec!["Water Breathing".to_string(), "Pressure Adaptation".to_string()],
                vec![NaturalAbility::NaturalSwim, NaturalAbility::PressureAdaptation, NaturalAbility::LifeDetection],
                vec![ElementalAffinity::Water, ElementalAffinity::Ice],
                vec![MagicalInclination::Healing, MagicalInclination::Elemental(ElementalAffinity::Water)],
                SocialStructure::Council,
                vec![Environment::Ocean, Environment::River],
                ArchitecturalStyle::Coral,
                vec!["Aquatic".to_string()],
            )
        }).collect()
    }

    /// Helper method to create base race with common structure
    fn create_base_race(
        &self,
        name: &str,
        description: &str,
        category: RaceCategory,
        planet: &str,
        subspecies_of: Option<String>,
        height_range: (f32, f32),
        build: BodyBuild,
        skin_tones: Vec<&str>,
        hair_colors: Vec<&str>,
        eye_colors: Vec<String>,
        maturity: u32,
        avg_lifespan: u32,
        max_lifespan: u32,
        aging_rate: AgingRate,
        cultural_traits: Vec<CulturalTrait>,
        spiritual_alignment: SpiritualAlignment,
        primary_blessing: String,
        secondary_blessings: Vec<String>,
        natural_abilities: Vec<NaturalAbility>,
        elemental_affinities: Vec<ElementalAffinity>,
        magical_inclinations: Vec<MagicalInclination>,
        social_structure: SocialStructure,
        preferred_environments: Vec<Environment>,
        architectural_style: ArchitecturalStyle,
        native_languages: Vec<String>,
    ) -> RaceDefinition {
        RaceDefinition {
            id: Uuid::new_v4(),
            name: name.to_string(),
            plural_name: name.to_string(),
            category,
            origin_planet: planet.to_string(),
            subspecies_of,
            appearance: RaceAppearance {
                height_range,
                build,
                skin_tones: skin_tones.into_iter().map(|s| s.to_string()).collect(),
                hair_colors: hair_colors.into_iter().map(|h| h.to_string()).collect(),
                eye_colors,
                distinctive_features: vec!["Divine heritage".to_string()],
                markings_or_patterns: vec!["Blessing marks".to_string()],
                secondary_features: Vec::new(),
            },
            lifespan: RaceLifespan {
                maturity_age: maturity,
                average_lifespan: avg_lifespan,
                maximum_lifespan: max_lifespan,
                aging_rate,
            },
            size_category: SizeCategory::Medium,
            cultural_traits,
            spiritual_alignment,
            divine_blessing: DivineBlessings {
                primary_blessing,
                secondary_blessings,
                patron_deity: Some("The Great Creator".to_string()),
                sacred_symbols: vec!["Divine light".to_string()],
            },
            natural_abilities,
            elemental_affinities,
            magical_inclinations,
            social_structure,
            preferred_environments,
            architectural_style,
            native_languages,
            communication_style: CommunicationStyle::Verbal,
            origin_story: description.to_string(),
            cultural_values: vec!["Honor".to_string(), "Wisdom".to_string()],
            legendary_figures: vec!["The First of their kind".to_string()],
            relationships_with_other_races: HashMap::new(),
        }
    }

    // Simplified implementations for now - each would be fully detailed like above
    fn create_gnomish_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Gnomish Variant {}", i + 1), "Clever small folk", RaceCategory::Gnomish, planet, Some("Gnome".to_string()),
                (0.9, 1.1), BodyBuild::Petite, vec!["Fair", "Rosy"], vec!["Various"], 
                vec!["Bright".to_string()], 40, 400, 500, AgingRate::Slow,
                vec![CulturalTrait::Scholarly], SpiritualAlignment::Wisdom, "Innovation".to_string(), vec![],
                vec![NaturalAbility::MagicDetection], vec![ElementalAffinity::Earth], vec![MagicalInclination::Transmutation],
                SocialStructure::Meritocracy, vec![Environment::Underground], ArchitecturalStyle::Geometric, vec!["Gnomish".to_string()]
            )
        }).collect()
    }
    
    fn create_orcish_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Orcish Variant {}", i + 1), "Noble warrior folk", RaceCategory::Orcish, planet, Some("Orc".to_string()),
                (1.8, 2.2), BodyBuild::Powerful, vec!["Green-tinted", "Earth-toned"], vec!["Dark"], 
                vec!["Red".to_string(), "Yellow".to_string()], 16, 80, 120, AgingRate::Normal,
                vec![CulturalTrait::Warrior], SpiritualAlignment::Courage, "Battle Honor".to_string(), vec![],
                vec![NaturalAbility::NaturalWeapons("Tusks".to_string())], vec![ElementalAffinity::Earth], vec![MagicalInclination::Evocation],
                SocialStructure::Tribal, vec![Environment::Plains], ArchitecturalStyle::Stone, vec!["Orcish".to_string()]
            )
        }).collect()
    }
    
    fn create_dragonborn_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Dragonborn Variant {}", i + 1), "Dragon-blooded noble folk", RaceCategory::Dragonborn, planet, Some("Dragonborn".to_string()),
                (1.8, 2.1), BodyBuild::Statuesque, vec!["Scaled"], vec!["None"], 
                vec!["Draconic".to_string()], 15, 80, 120, AgingRate::Normal,
                vec![CulturalTrait::Warrior], SpiritualAlignment::Justice, "Dragon Heritage".to_string(), vec![],
                vec![NaturalAbility::NaturalArmor], vec![ElementalAffinity::Fire], vec![MagicalInclination::Evocation],
                SocialStructure::Hierarchy, vec![Environment::Mountains], ArchitecturalStyle::Towering, vec!["Draconic".to_string()]
            )
        }).collect()
    }
    
    fn create_mixed_heritage_races(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Mixed Heritage {}", i + 1), "Blessed unions of different races", RaceCategory::HumanHybrid, planet, None,
                (1.5, 1.8), BodyBuild::Athletic, vec!["Varied"], vec!["Varied"], 
                vec!["Varied".to_string()], 25, 150, 250, AgingRate::Normal,
                vec![CulturalTrait::Progressive], SpiritualAlignment::Unity, "Hybrid Vigor".to_string(), vec![],
                vec![], vec![], vec![MagicalInclination::Enchantment],
                SocialStructure::Democracy, vec![Environment::Urban], ArchitecturalStyle::Harmonious, vec!["Common".to_string()]
            )
        }).collect()
    }

    // New planet-specific race creation methods needed for the implementations above
    fn create_sylvan_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Sylvan Variant {}", i + 1), "Tree-bonded nature folk", RaceCategory::Sylvan, planet, None,
                (1.8, 2.4), BodyBuild::Graceful, vec!["Bark-textured", "Leaf-patterned"], vec!["Moss green", "Autumn colors"], 
                vec!["Green".to_string(), "Brown".to_string()], 50, 500, 800, AgingRate::Slow,
                vec![CulturalTrait::Harmonious, CulturalTrait::Protective], SpiritualAlignment::Growth, "Nature Unity".to_string(), vec!["Plant Communication".to_string()],
                vec![NaturalAbility::LifeDetection, NaturalAbility::NaturalHealing], vec![ElementalAffinity::Nature], vec![MagicalInclination::Nature, MagicalInclination::Healing],
                SocialStructure::Collective, vec![Environment::Forest], ArchitecturalStyle::Organic, vec!["Sylvan".to_string()]
            )
        }).collect()
    }

    fn create_nature_elven_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Nature Elf Variant {}", i + 1), "Forest-guardian elves", RaceCategory::Elven, planet, Some("Elf".to_string()),
                (1.7, 1.9), BodyBuild::Graceful, vec!["Green-tinged", "Earth-toned"], vec!["Forest green", "Auburn"], 
                vec!["Green".to_string(), "Gold".to_string()], 100, 750, 1200, AgingRate::Slow,
                vec![CulturalTrait::Mystical, CulturalTrait::Protective], SpiritualAlignment::Harmony, "Forest Guardian".to_string(), vec!["Animal Speech".to_string()],
                vec![NaturalAbility::EnhancedSenses("All".to_string()), NaturalAbility::Camouflage], vec![ElementalAffinity::Nature], vec![MagicalInclination::Nature, MagicalInclination::Illusion],
                SocialStructure::Council, vec![Environment::Forest], ArchitecturalStyle::Tree, vec!["Elvish".to_string()]
            )
        }).collect()
    }

    fn create_plant_hybrid_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Plant Hybrid {}", i + 1), "Human-plant blessed unions", RaceCategory::HumanHybrid, planet, None,
                (1.6, 1.8), BodyBuild::Athletic, vec!["Chlorophyll-tinted"], vec!["Flower colors", "Leaf patterns"], 
                vec!["Green".to_string(), "Gold".to_string()], 25, 200, 300, AgingRate::Slow,
                vec![CulturalTrait::Harmonious], SpiritualAlignment::Growth, "Photosynthetic Blessing".to_string(), vec!["Sunlight Sustenance".to_string()],
                vec![NaturalAbility::NaturalHealing, NaturalAbility::PoisonResistance], vec![ElementalAffinity::Nature, ElementalAffinity::Light], vec![MagicalInclination::Nature, MagicalInclination::Healing],
                SocialStructure::Collective, vec![Environment::Forest], ArchitecturalStyle::Organic, vec!["Common".to_string()]
            )
        }).collect()
    }

    fn create_animal_kin_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Beast Kin {}", i + 1), "Animal-spirited humanoids", RaceCategory::BeastkinHybrid, planet, None,
                (1.5, 2.0), BodyBuild::Athletic, vec!["Fur patterns", "Natural coloring"], vec!["Animal colors"], 
                vec!["Wild colors".to_string()], 20, 100, 150, AgingRate::Normal,
                vec![CulturalTrait::Harmonious, CulturalTrait::Independent], SpiritualAlignment::Balance, "Wild Spirit".to_string(), vec!["Animal Communication".to_string()],
                vec![NaturalAbility::EnhancedSenses("Smell".to_string()), NaturalAbility::NaturalClimb], vec![ElementalAffinity::Nature], vec![MagicalInclination::Nature, MagicalInclination::Primal],
                SocialStructure::Tribal, vec![Environment::Forest, Environment::Wilderness], ArchitecturalStyle::Organic, vec!["Beast Speech".to_string()]
            )
        }).collect()
    }

    fn create_fairy_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Fairy Folk {}", i + 1), "Diminutive forest spirits", RaceCategory::Sylvan, planet, None,
                (0.3, 0.6), BodyBuild::Ethereal, vec!["Iridescent", "Flower-petal"], vec!["Rainbow colors", "Flower hues"], 
                vec!["Sparkling".to_string()], 10, 300, 500, AgingRate::Slow,
                vec![CulturalTrait::Playful, CulturalTrait::Mystical], SpiritualAlignment::Joy, "Fairy Magic".to_string(), vec!["Natural Flight".to_string(), "Size Change".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::MagicDetection], vec![ElementalAffinity::Nature, ElementalAffinity::Air], vec![MagicalInclination::Illusion, MagicalInclination::Nature],
                SocialStructure::Collective, vec![Environment::Forest], ArchitecturalStyle::Organic, vec!["Fairy".to_string()]
            )
        }).collect()
    }

    fn create_salamander_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Salamander Kin {}", i + 1), "Fire salamander humanoids", RaceCategory::FireTouched, planet, None,
                (1.5, 1.8), BodyBuild::Athletic, vec!["Fire-scaled", "Ember-touched"], vec!["Flame colors"], 
                vec!["Fire orange".to_string(), "Lava red".to_string()], 30, 150, 250, AgingRate::Normal,
                vec![CulturalTrait::Warrior, CulturalTrait::Independent], SpiritualAlignment::Courage, "Fire Immunity".to_string(), vec!["Heat Resistance".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::NaturalArmor], vec![ElementalAffinity::Fire], vec![MagicalInclination::Evocation, MagicalInclination::Elemental(ElementalAffinity::Fire)],
                SocialStructure::Tribal, vec![Environment::Volcanic], ArchitecturalStyle::Stone, vec!["Draconic".to_string()]
            )
        }).collect()
    }

    fn create_forge_dwarf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Forge Dwarf {}", i + 1), "Volcano-dwelling master smiths", RaceCategory::Dwarven, planet, Some("Dwarf".to_string()),
                (1.3, 1.5), BodyBuild::Robust, vec!["Heat-resistant", "Ember-touched"], vec!["Molten colors"], 
                vec!["Flame orange".to_string(), "Coal black".to_string()], 120, 400, 600, AgingRate::Slow,
                vec![CulturalTrait::Artisan, CulturalTrait::Traditionalist], SpiritualAlignment::Justice, "Master Smithing".to_string(), vec!["Fire Resistance".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::NaturalArmor], vec![ElementalAffinity::Fire, ElementalAffinity::Metal], vec![MagicalInclination::Transmutation, MagicalInclination::Enchantment],
                SocialStructure::Clan, vec![Environment::Volcanic, Environment::Mountains], ArchitecturalStyle::Stone, vec!["Dwarvish".to_string()]
            )
        }).collect()
    }
    
    // Placeholder methods for other planets - would implement full race generation for each
    /// Generate races for Verdania (the forest world)
    fn generate_verdania_races(&mut self) -> Result<(), String> {
        let planet_name = "Verdania".to_string();
        let mut races = Vec::new();
        
        // Forest-adapted races
        races.extend(self.create_sylvan_variants(&planet_name, 15)); // Tree-people
        races.extend(self.create_nature_elven_variants(&planet_name, 12)); // Forest elves
        races.extend(self.create_plant_hybrid_variants(&planet_name, 10)); // Plant-human hybrids
        races.extend(self.create_animal_kin_variants(&planet_name, 8)); // Beast-people
        races.extend(self.create_fairy_variants(&planet_name, 5)); // Small fae folk
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    /// Generate races for Pyros (volcanic world)
    fn generate_pyros_races(&mut self) -> Result<(), String> {
        let planet_name = "Pyros".to_string();
        let mut races = Vec::new();
        
        // Fire-adapted races
        races.extend(self.create_salamander_variants(&planet_name, 12)); // Fire salamander people
        races.extend(self.create_forge_dwarf_variants(&planet_name, 10)); // Volcano dwarves
        races.extend(self.create_phoenix_kin_variants(&planet_name, 8)); // Phoenix-blooded
        races.extend(self.create_lava_elemental_variants(&planet_name, 6)); // Lava-touched humans
        races.extend(self.create_ember_fae_variants(&planet_name, 4)); // Fire sprites
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    /// Generate races for Aquatica (ocean world)
    fn generate_aquatica_races(&mut self) -> Result<(), String> {
        let planet_name = "Aquatica".to_string();
        let mut races = Vec::new();
        
        // Aquatic races (expanding the existing system)
        races.extend(self.create_aquatic_variants(&planet_name, 15)); // Deep sea dwellers
        races.extend(self.create_coral_people_variants(&planet_name, 10)); // Coral reef people
        races.extend(self.create_sea_elf_variants(&planet_name, 8)); // Ocean elves
        races.extend(self.create_tide_human_variants(&planet_name, 7)); // Sea-adapted humans
        races.extend(self.create_whale_kin_variants(&planet_name, 5)); // Whale-song people
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    /// Generate races for Lumina (crystal world)
    fn generate_lumina_races(&mut self) -> Result<(), String> {
        let planet_name = "Lumina".to_string();
        let mut races = Vec::new();
        
        // Crystal-attuned races
        races.extend(self.create_crystal_kin_variants(&planet_name, 12)); // Living crystal people
        races.extend(self.create_gem_dwarf_variants(&planet_name, 8)); // Crystal-mining dwarfs
        races.extend(self.create_prism_elf_variants(&planet_name, 7)); // Light-refracting elves
        races.extend(self.create_resonance_human_variants(&planet_name, 5)); // Sound-crystal humans
        races.extend(self.create_light_fae_variants(&planet_name, 3)); // Radiant sprites
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    /// Generate races for Umbra (underground world)
    fn generate_umbra_races(&mut self) -> Result<(), String> {
        let planet_name = "Umbra".to_string();
        let mut races = Vec::new();
        
        // Underground-adapted races
        races.extend(self.create_deep_dwarf_variants(&planet_name, 12)); // Cave dwarfs
        races.extend(self.create_shadow_elf_variants(&planet_name, 10)); // Dark elves (divine)
        races.extend(self.create_cave_human_variants(&planet_name, 8)); // Underground humans
        races.extend(self.create_mushroom_kin_variants(&planet_name, 6)); // Fungal people
        races.extend(self.create_crystal_moth_variants(&planet_name, 4)); // Luminous cave dwellers
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    /// Generate races for Glacialis (ice world)
    fn generate_glacialis_races(&mut self) -> Result<(), String> {
        let planet_name = "Glacialis".to_string();
        let mut races = Vec::new();
        
        // Ice-adapted races
        races.extend(self.create_frost_giant_variants(&planet_name, 8)); // Ice giants
        races.extend(self.create_ice_elf_variants(&planet_name, 7)); // Frost elves
        races.extend(self.create_snow_dwarf_variants(&planet_name, 6)); // Arctic dwarfs
        races.extend(self.create_ice_human_variants(&planet_name, 5)); // Cold-adapted humans
        races.extend(self.create_winter_fae_variants(&planet_name, 4)); // Frost sprites
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    /// Generate races for Tempest (desert world)
    fn generate_tempest_races(&mut self) -> Result<(), String> {
        let planet_name = "Tempest".to_string();
        let mut races = Vec::new();
        
        // Desert-adapted races
        races.extend(self.create_sand_nomad_variants(&planet_name, 8)); // Desert nomads
        races.extend(self.create_dune_elf_variants(&planet_name, 7)); // Sand elves
        races.extend(self.create_cactus_kin_variants(&planet_name, 6)); // Plant-desert hybrids
        races.extend(self.create_mirage_human_variants(&planet_name, 5)); // Heat-adapted humans
        races.extend(self.create_sand_djinn_variants(&planet_name, 4)); // Desert spirits
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    /// Generate races for Seraphim (floating world)
    fn generate_seraphim_races(&mut self) -> Result<(), String> {
        let planet_name = "Seraphim".to_string();
        let mut races = Vec::new();
        
        // Sky-adapted races
        races.extend(self.create_sky_human_variants(&planet_name, 8)); // Cloud-dwelling humans
        races.extend(self.create_wind_elf_variants(&planet_name, 7)); // Air elves
        races.extend(self.create_cloud_giant_variants(&planet_name, 5)); // Sky giants
        races.extend(self.create_storm_kin_variants(&planet_name, 3)); // Lightning people
        races.extend(self.create_air_sprite_variants(&planet_name, 2)); // Wind spirits
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    /// Generate races for Nexus (terrestrial world)
    fn generate_nexus_races(&mut self) -> Result<(), String> {
        let planet_name = "Nexus".to_string();
        let mut races = Vec::new();
        
        // Terrestrial balanced races
        races.extend(self.create_plains_human_variants(&planet_name, 10)); // Grassland humans
        races.extend(self.create_hill_dwarf_variants(&planet_name, 8)); // Rolling hill dwarfs
        races.extend(self.create_meadow_elf_variants(&planet_name, 7)); // Peaceful elves
        races.extend(self.create_earth_giant_variants(&planet_name, 5)); // Stone giants
        races.extend(self.create_terra_kin_variants(&planet_name, 5)); // Earth-bonded folk
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    /// Generate races for Calypso (mysterious floating world)
    fn generate_calypso_races(&mut self) -> Result<(), String> {
        let planet_name = "Calypso".to_string();
        let mut races = Vec::new();
        
        // Mysterious floating island races
        races.extend(self.create_void_touched_variants(&planet_name, 12)); // Space-touched beings
        races.extend(self.create_star_elf_variants(&planet_name, 10)); // Cosmic elves
        races.extend(self.create_gravity_human_variants(&planet_name, 8)); // Weightless humans
        races.extend(self.create_cosmic_giant_variants(&planet_name, 6)); // Space giants
        races.extend(self.create_nebula_sprite_variants(&planet_name, 4)); // Cosmic spirits
        
        // Store races and planet distribution
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        self.planet_distributions.insert(planet_name, races.into_iter().map(|r| r.name).collect());
        Ok(())
    }
    /// Generate shared races (found on multiple planets)
    fn generate_shared_races(&mut self) -> Result<(), String> {
        // These races exist across multiple planets and facilitate inter-world contact
        let mut races = Vec::new();
        
        // Dimensional travelers and traders
        races.extend(self.create_planar_human_variants(15)); // Dimension-hopping humans
        races.extend(self.create_cosmic_elf_variants(12)); // Universe-traveling elves
        races.extend(self.create_void_walker_variants(10)); // Inter-dimensional beings
        races.extend(self.create_star_dwarf_variants(8)); // Space-faring dwarfs
        races.extend(self.create_portal_mage_variants(5)); // Teleportation specialists
        
        // Store races across multiple planets
        for race in races.clone() {
            self.races.insert(race.name.clone(), race);
        }
        
        // Distribute shared races to multiple planets
        let shared_race_names: Vec<String> = races.into_iter().map(|r| r.name).collect();
        let planets = vec!["Espan", "Verdania", "Pyros", "Aquatica", "Lumina", "Umbra", "Glacialis", "Tempest", "Seraphim", "Nexus", "Calypso"];
        
        for planet in planets {
            if let Some(planet_races) = self.planet_distributions.get_mut(planet) {
                planet_races.extend(shared_race_names.clone());
            }
        }
        
        Ok(())
    }
    
    /// Get races by planet
    pub fn get_races_for_planet(&self, planet_name: &str) -> Vec<&RaceDefinition> {
        if let Some(race_names) = self.planet_distributions.get(planet_name) {
            race_names.iter()
                .filter_map(|name| self.races.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get race by name
    pub fn get_race(&self, race_name: &str) -> Option<&RaceDefinition> {
        self.races.get(race_name)
    }
    
    /// Get all races in a category
    pub fn get_races_by_category(&self, category: &RaceCategory) -> Vec<&RaceDefinition> {
        self.races.values()
            .filter(|race| std::mem::discriminant(&race.category) == std::mem::discriminant(category))
            .collect()
    }
}

// Include additional race creation helpers
pub mod race_creation_helpers;
pub mod remaining_races;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_race_database_creation() {
        let mut db = RaceDatabase::new();
        assert!(db.generate_complete_database().is_ok());
        assert!(db.races.len() > 0);
    }
}