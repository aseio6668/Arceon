use rand::Rng;
use arceon_core::entities::being::Race;
use arceon_core::entities::world::{LocationType, CultureType};

pub struct NameGenerator {
    rng: rand::rngs::ThreadRng,
}

impl NameGenerator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
    
    /// Generate a city name based on race and culture
    pub fn generate_city_name(&mut self, race: &Race, culture: &CultureType) -> String {
        match race {
            Race::Human => {
                let prefixes = match culture {
                    CultureType::TradingHub => vec!["Gold", "Silver", "Merchant", "Trade", "Market"],
                    CultureType::Traditional => vec!["Old", "King", "Royal", "Grand", "Noble"],
                    CultureType::Military => vec!["Fort", "Guard", "Shield", "Sword", "Banner"],
                    _ => vec!["New", "Fair", "Green", "High", "White"],
                };
                let suffixes = vec!["haven", "vale", "ford", "burg", "town", "helm", "heart", "gate"];
                
                let prefix = prefixes[self.rng.gen_range(0..prefixes.len())];
                let suffix = suffixes[self.rng.gen_range(0..suffixes.len())];
                format!("{}{}", prefix, suffix)
            },
            Race::Elf => {
                let prefixes = vec!["Silver", "Moon", "Star", "Sun", "Wind", "Forest", "Ancient", "Mystic"];
                let suffixes = vec!["leaf", "grove", "spire", "haven", "glade", "wood", "vale", "reach"];
                
                let prefix = prefixes[self.rng.gen_range(0..prefixes.len())];
                let suffix = suffixes[self.rng.gen_range(0..suffixes.len())];
                format!("{}{}", prefix, suffix)
            },
            Race::Dwarf => {
                let prefixes = vec!["Iron", "Stone", "Deep", "Under", "Hammer", "Anvil", "Forge", "Mountain"];
                let suffixes = vec!["forge", "hold", "hall", "mine", "peak", "deep", "helm", "guard"];
                
                let prefix = prefixes[self.rng.gen_range(0..prefixes.len())];
                let suffix = suffixes[self.rng.gen_range(0..suffixes.len())];
                format!("{}{}", prefix, suffix)
            },
            Race::Gnome => {
                let prefixes = vec!["Bright", "Clever", "Tinker", "Gear", "Spring", "Crystal", "Deep", "Hidden"];
                let suffixes = vec!["holm", "wick", "glen", "burrow", "nook", "haven", "dell", "works"];
                
                let prefix = prefixes[self.rng.gen_range(0..prefixes.len())];
                let suffix = suffixes[self.rng.gen_range(0..suffixes.len())];
                format!("{}{}", prefix, suffix)
            },
            Race::Halfling => {
                let prefixes = vec!["Green", "Pleasant", "Cozy", "Peaceful", "Golden", "Sunny", "Warm", "Happy"];
                let suffixes = vec!["hill", "hollow", "dale", "brook", "meadow", "field", "garden", "rest"];
                
                let prefix = prefixes[self.rng.gen_range(0..prefixes.len())];
                let suffix = suffixes[self.rng.gen_range(0..suffixes.len())];
                format!("{}{}", prefix, suffix)
            },
            Race::Orc => {
                let prefixes = vec!["Blood", "War", "Iron", "Dark", "Skull", "Bone", "Sharp", "Grim"];
                let suffixes = vec!["rock", "fang", "claw", "hold", "pit", "gorge", "peak", "stronghold"];
                
                let prefix = prefixes[self.rng.gen_range(0..prefixes.len())];
                let suffix = suffixes[self.rng.gen_range(0..suffixes.len())];
                format!("{} {}", prefix, suffix)
            },
            _ => {
                let prefixes = vec!["New", "Old", "Great", "Little", "High", "Low"];
                let suffixes = vec!["town", "burg", "ville", "haven", "port", "ford"];
                
                let prefix = prefixes[self.rng.gen_range(0..prefixes.len())];
                let suffix = suffixes[self.rng.gen_range(0..suffixes.len())];
                format!("{}{}", prefix, suffix)
            }
        }
    }
    
    /// Generate a location name within a city
    pub fn generate_location_name(&mut self, location_type: &LocationType, race: &Race) -> String {
        match location_type {
            LocationType::Bank => {
                let names = vec!["First Bank", "Royal Treasury", "Merchant's Bank", "Goldkeeper's Hall"];
                names[self.rng.gen_range(0..names.len())].to_string()
            },
            LocationType::Tavern => {
                let adjectives = vec!["Prancing", "Drunken", "Golden", "Silver", "Merry", "Roaring"];
                let nouns = match race {
                    Race::Dwarf => vec!["Hammer", "Anvil", "Beard", "Pickaxe", "Mountain"],
                    Race::Elf => vec!["Leaf", "Moon", "Star", "Arrow", "Bow"],
                    Race::Halfling => vec!["Pony", "Pipe", "Barrel", "Mushroom", "Garden"],
                    _ => vec!["Dragon", "Lion", "Eagle", "Bear", "Wolf"],
                };
                
                let adj = adjectives[self.rng.gen_range(0..adjectives.len())];
                let noun = nouns[self.rng.gen_range(0..nouns.len())];
                format!("The {} {}", adj, noun)
            },
            LocationType::Shop => {
                let types = vec!["General Store", "Weaponsmith", "Armorer", "Alchemist", "Clothier"];
                types[self.rng.gen_range(0..types.len())].to_string()
            },
            LocationType::Stable => {
                let names = vec!["Royal Stables", "Merchant's Mounts", "Swift Hooves", "Noble Steeds"];
                names[self.rng.gen_range(0..names.len())].to_string()
            },
            LocationType::Temple => {
                let deities = vec!["Solace", "Wisdom", "Justice", "Nature", "War", "Peace"];
                let deity = deities[self.rng.gen_range(0..deities.len())];
                format!("Temple of {}", deity)
            },
            LocationType::Market => "Central Market".to_string(),
            LocationType::Harbor => "Harbor District".to_string(),
            LocationType::Gate => {
                let directions = vec!["North", "South", "East", "West"];
                let dir = directions[self.rng.gen_range(0..directions.len())];
                format!("{} Gate", dir)
            },
            _ => format!("{:?}", location_type).replace('_', " "),
        }
    }
    
    /// Generate an NPC name based on race
    pub fn generate_npc_name(&mut self, race: &Race) -> String {
        match race {
            Race::Human => {
                let first_names = vec!["Marcus", "Elena", "Thomas", "Sarah", "William", "Mary", "James", "Anna"];
                let last_names = vec!["Smith", "Johnson", "Brown", "Davis", "Miller", "Wilson", "Moore", "Taylor"];
                
                let first = first_names[self.rng.gen_range(0..first_names.len())];
                let last = last_names[self.rng.gen_range(0..last_names.len())];
                format!("{} {}", first, last)
            },
            Race::Elf => {
                let first_names = vec!["Aelynn", "Celeborn", "Galadriel", "Legolas", "Arwen", "Elrond", "Thranduil", "Nimrodel"];
                let last_names = vec!["Starweaver", "Moonwhisper", "Silverleaf", "Goldenbough", "Nightbreeze", "Dawnstrider"];
                
                let first = first_names[self.rng.gen_range(0..first_names.len())];
                let last = last_names[self.rng.gen_range(0..last_names.len())];
                format!("{} {}", first, last)
            },
            Race::Dwarf => {
                let first_names = vec!["Thorin", "Balin", "Dwalin", "Gloin", "Dain", "Nali", "Ori", "Bifur"];
                let last_names = vec!["Ironforge", "Stonebeard", "Hammerfist", "Goldbeard", "Rockbreaker", "Battleaxe"];
                
                let first = first_names[self.rng.gen_range(0..first_names.len())];
                let last = last_names[self.rng.gen_range(0..last_names.len())];
                format!("{} {}", first, last)
            },
            Race::Gnome => {
                let first_names = vec!["Zook", "Bink", "Fizz", "Gleep", "Nibbles", "Sparky", "Tinker", "Gadget"];
                let last_names = vec!["Sprocketwrench", "Geargrinder", "Boltbucket", "Wirecutter", "Springloaded"];
                
                let first = first_names[self.rng.gen_range(0..first_names.len())];
                let last = last_names[self.rng.gen_range(0..last_names.len())];
                format!("{} {}", first, last)
            },
            Race::Halfling => {
                let first_names = vec!["Frodo", "Samwise", "Pippin", "Merry", "Bilbo", "Rosie", "Daisy", "Poppy"];
                let last_names = vec!["Baggins", "Gamgee", "Took", "Brandybuck", "Greenhill", "Brownlock"];
                
                let first = first_names[self.rng.gen_range(0..first_names.len())];
                let last = last_names[self.rng.gen_range(0..last_names.len())];
                format!("{} {}", first, last)
            },
            Race::Orc => {
                let names = vec!["Grashk", "Ugluk", "Nazgul", "Gothmog", "Azog", "Bolg", "Lurtz", "Sharku"];
                names[self.rng.gen_range(0..names.len())].to_string()
            },
            _ => {
                let names = vec!["Adventurer", "Traveler", "Wanderer", "Stranger", "Visitor"];
                names[self.rng.gen_range(0..names.len())].to_string()
            }
        }
    }
}
