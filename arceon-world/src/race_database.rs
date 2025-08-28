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
    Harmonious, Competitive, Protective, Exploratory, Meditative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpiritualAlignment {
    Light, Balance, Harmony, Wisdom, Courage, Compassion, Justice, 
    Beauty, Truth, Growth, Protection, Renewal, Unity, Transcendence,
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
    Nature, Crystal, Metal, Sound, Gravity, Time, Space,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MagicalInclination {
    Divination, Enchantment, Evocation, Illusion, Transmutation,
    Conjuration, Necromancy(bool), // bool: benevolent necromancy only
    Healing, Warding, Blessing, Nature, Elemental(ElementalAffinity),
    Dimensional, Temporal, Psionic, Divine, Primal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialStructure {
    Tribal, Clan, Kingdom, Democracy, Council, Theocracy,
    Meritocracy, Collective, Hierarchy, Egalitarian,
    Nomadic, CityStates, Federation, Empire,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Forest, Plains, Mountains, Desert, Ocean, River, Lake,
    Underground, Floating, Volcanic, Crystal, Ice, Jungle,
    Swamp, Canyon, Cliff, Beach, Urban, Rural, Wilderness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchitecturalStyle {
    Organic, Crystal, Stone, Wood, Metal, Floating,
    Underground, Cliff, Tree, Coral, Ice, Magical,
    Geometric, Flowing, Towering, Harmonious,
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
    
    // Placeholder methods for other planets - would implement full race generation for each
    fn generate_verdania_races(&mut self) -> Result<(), String> { Ok(()) }
    fn generate_pyros_races(&mut self) -> Result<(), String> { Ok(()) }
    fn generate_aquatica_races(&mut self) -> Result<(), String> { Ok(()) }
    fn generate_lumina_races(&mut self) -> Result<(), String> { Ok(()) }
    fn generate_umbra_races(&mut self) -> Result<(), String> { Ok(()) }
    fn generate_glacialis_races(&mut self) -> Result<(), String> { Ok(()) }
    fn generate_tempest_races(&mut self) -> Result<(), String> { Ok(()) }
    fn generate_seraphim_races(&mut self) -> Result<(), String> { Ok(()) }
    fn generate_nexus_races(&mut self) -> Result<(), String> { Ok(()) }
    fn generate_calypso_races(&mut self) -> Result<(), String> { Ok(()) }
    fn generate_shared_races(&mut self) -> Result<(), String> { Ok(()) }
    
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