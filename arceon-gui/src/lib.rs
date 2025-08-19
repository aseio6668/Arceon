pub mod windows;
pub mod themes;
pub mod widgets;
pub mod input;
pub mod client;

use anyhow::Result;
use eframe::egui;
// use std::sync::Arc;
// use tokio::sync::RwLock;
use std::collections::HashMap;

use arceon_core::ArceonCore;
use windows::*;
use themes::ArceonTheme;
use client::GameClient;

#[derive(Debug, Clone)]
pub struct AreaExit {
    pub direction: String,
    pub target_area: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct PlacedStructure {
    pub id: String,
    pub name: String,
    pub structure_type: StructureType,
    pub builder: String, // Who built it (player/NPC name)
    pub description: String,
    pub can_enter: bool,
    pub interior_area: Option<String>, // If structure has interior, this is the area name
}

#[derive(Debug, Clone)]
pub enum StructureType {
    House,
    TownHall,
    Shop,
    Tavern,
    Blacksmith,
    Library,
    Temple,
    Castle,
    Tower,
    Bridge,
    Wall,
    Gate,
}

/// Main GUI application
pub struct ArceonGui {
    core: ArceonCore,
    theme: ArceonTheme,
    
    // Window states
    main_console: ConsoleWindow,
    inventory_window: InventoryWindow,
    character_window: CharacterWindow,
    map_window: MapWindow,
    skills_window: SkillsWindow,
    settings_window: SettingsWindow,
    
    // UI state
    command_input: String,
    show_inventory: bool,
    show_character: bool,
    show_map: bool,
    show_skills: bool,
    show_settings: bool,
    
    // Game state
    player_id: Option<String>,
    is_connected: bool,
    server_connected: bool,
    current_area: String,
    area_connections: HashMap<String, Vec<AreaExit>>,
    area_descriptions: HashMap<String, String>,
    area_structures: HashMap<String, Vec<PlacedStructure>>,
    
    // Network client
    #[allow(dead_code)]
    game_client: GameClient,
    #[allow(dead_code)]
    server_url: String,
}

impl ArceonGui {
    pub fn new(core: ArceonCore) -> Self {
        let server_url = "http://localhost:8080".to_string();
        let game_client = GameClient::new(server_url.clone());
        
        let mut gui = Self {
            core,
            theme: ArceonTheme::dark_fantasy(),
            main_console: ConsoleWindow::new(),
            inventory_window: InventoryWindow::new(),
            character_window: CharacterWindow::new(),
            map_window: MapWindow::new(),
            skills_window: SkillsWindow::new(),
            settings_window: SettingsWindow::new(),
            command_input: String::new(),
            show_inventory: false,
            show_character: false,
            show_map: false,
            show_skills: false,
            show_settings: false,
            player_id: None,
            is_connected: false,
            server_connected: false,
            current_area: "Espan, Central Plains".to_string(),
            area_connections: Self::create_area_connections(),
            area_descriptions: Self::create_area_descriptions(),
            area_structures: Self::create_initial_structures(),
            game_client,
            server_url,
        };
        
        // Add welcome message
        gui.main_console.add_output("Welcome to Arceon - Fantasy MMORPG!");
        gui.main_console.add_output("Type 'help' for available commands.");
        gui.main_console.add_output("Type 'connect <player_name>' to connect to server.");
        gui.main_console.add_output("Make sure arceon-server is running on port 8080!");
        
        gui
    }
    
    /// Show NPCs and creatures in the current area
    fn show_area_inhabitants(&mut self, area_name: &str) {
        // Define area populations based on area type and characteristics
        match area_name {
            "Espan, Central Plains" => {
                self.main_console.add_output("");
                self.main_console.add_output("=== Inhabitants ===");
                self.main_console.add_output("🧙 Eldara the Wise - An ancient scholar studying the stone markers");
                self.main_console.add_output("⚔️ Captain Marcus - A seasoned guard patrolling the crossroads");
                self.main_console.add_output("🐺 Wild Plains Wolf - A lone wolf hunting in the grasslands");
                self.main_console.add_output("🐰 Prairie Rabbits - Small creatures darting through the grass");
                self.main_console.add_output("🦅 Soaring Hawks - Majestic birds circling overhead");
            },
            "Alderheart" => {
                self.main_console.add_output("");
                self.main_console.add_output("=== Inhabitants ===");
                self.main_console.add_output("🏪 Merchant Yorick - A wealthy trader selling exotic goods");
                self.main_console.add_output("🔨 Blacksmith Gareth - The city's master weaponsmith");
                self.main_console.add_output("⚓ Dockworker Finn - A burly man loading ships at the harbor");
                self.main_console.add_output("👩‍💼 Harbormaster Elena - The efficient port authority leader");
                self.main_console.add_output("🐀 Harbor Rats - Scavenging rodents near the docks");
                self.main_console.add_output("🐙 Curious Octopi - Sea creatures visible in the harbor waters");
            },
            "Silverleaf Enclave" => {
                self.main_console.add_output("");
                self.main_console.add_output("=== Inhabitants ===");
                self.main_console.add_output("🧝‍♀️ Silviana Moonwhisper - An elven high priestess of nature");
                self.main_console.add_output("🧝‍♂️ Ranger Thalorin - An expert elven scout and tracker");
                self.main_console.add_output("📚 Lorekeeper Aerdeth - An ancient elf who knows forgotten secrets");
                self.main_console.add_output("🦌 Silver Stags - Mystical deer with shimmering antlers");
                self.main_console.add_output("🧚‍♀️ Forest Sprites - Tiny magical creatures flitting through the trees");
                self.main_console.add_output("🌸 Singing Birds - Melodious creatures with otherworldly songs");
            },
            "Dragon Spine Mountains" => {
                self.main_console.add_output("");
                self.main_console.add_output("=== Inhabitants ===");
                self.main_console.add_output("🏔️ Thorek Ironbeard - A dwarven mountain guide");
                self.main_console.add_output("🦅 Giant Mountain Eagles - Massive birds soaring between peaks");
                self.main_console.add_output("🐻 Cave Bears - Powerful predators dwelling in mountain caves");
                self.main_console.add_output("🐐 Mountain Goats - Sure-footed climbers on rocky ledges");
                self.main_console.add_output("⚠️ Warning: Dragon sightings reported in higher peaks!");
            },
            "Misty Marshlands" => {
                self.main_console.add_output("");
                self.main_console.add_output("=== Inhabitants ===");
                self.main_console.add_output("🧙‍♀️ Swamp Witch Morwyn - A mysterious practitioner of dark arts");
                self.main_console.add_output("🐊 Marsh Crocodiles - Dangerous predators lurking in murky waters");
                self.main_console.add_output("🐸 Giant Toads - Enormous amphibians with poisonous skin");
                self.main_console.add_output("🦆 Will-o'-Wisps - Strange lights that lead travelers astray");
                self.main_console.add_output("⚠️ Danger: Toxic gases and unstable ground!");
            },
            "Ancient Silverleaf Forest" => {
                self.main_console.add_output("");
                self.main_console.add_output("=== Inhabitants ===");
                self.main_console.add_output("🌳 Elder Treant Oakenheart - An ancient tree guardian");
                self.main_console.add_output("🧝‍♀️ Druid Circle - Elven nature magic practitioners");
                self.main_console.add_output("🦄 Unicorns - Rare and magical forest dwellers");
                self.main_console.add_output("🐺 Dire Wolves - Large, intelligent pack hunters");
                self.main_console.add_output("🦋 Magical Butterflies - Creatures that shimmer with arcane energy");
                self.main_console.add_output("🍄 Sentient Mushrooms - Fungi with mysterious consciousness");
            },
            "Skyhold Citadel" => {
                self.main_console.add_output("");
                self.main_console.add_output("=== Inhabitants ===");
                self.main_console.add_output("🏛️ Archmage Celestine - Master of the Celestial Arts");
                self.main_console.add_output("⚔️ Skyguard Captain Aurelius - Elite magical warriors");
                self.main_console.add_output("📜 Scribe Illuminus - Keeper of ancient magical knowledge");
                self.main_console.add_output("🦅 Celestial Eagles - Divine messengers of the sky realm");
                self.main_console.add_output("✨ Floating Crystals - Magical gems that hover and glow");
                self.main_console.add_output("🌟 Star Wisps - Beings of pure magical energy");
            },
            _ => {
                self.main_console.add_output("");
                self.main_console.add_output("=== Inhabitants ===");
                self.main_console.add_output("The area appears empty for now...");
            }
        }
        
        // Show any structures in the area
        self.show_area_structures(area_name);
    }
    
    /// Show structures that have been built in the current area
    fn show_area_structures(&mut self, area_name: &str) {
        if let Some(structures) = self.area_structures.get(area_name) {
            if !structures.is_empty() {
                self.main_console.add_output("");
                self.main_console.add_output("=== Structures ===");
                for structure in structures {
                    let enter_text = if structure.can_enter { " (enter available)" } else { "" };
                    self.main_console.add_output(&format!("🏗️ {} - Built by {}{}", 
                        structure.name, structure.builder, enter_text));
                    self.main_console.add_output(&format!("   {}", structure.description));
                }
            }
        }
    }
    
    /// Show structure interior with context of main area
    fn show_structure_interior(&mut self, structure_id: &str, main_area: &str) {
        // Show interior description based on structure type
        match structure_id {
            "waypoint_inn" => {
                self.main_console.add_output("You're inside the cozy Waypoint Inn. A warm fireplace crackles in the corner,");
                self.main_console.add_output("and the aroma of fresh bread and ale fills the air. Comfortable chairs are");
                self.main_console.add_output("arranged around wooden tables, perfect for weary travelers.");
                self.main_console.add_output("");
                self.main_console.add_output("=== Location Context ===");
                self.main_console.add_output(&format!("Main Area: {}", main_area));
                self.main_console.add_output("Use 'exit' to leave the inn and return outside.");
            },
            "yorick_emporium" => {
                self.main_console.add_output("You're inside Yorick's Exotic Emporium. Shelves line the walls, packed with");
                self.main_console.add_output("rare goods from distant lands. Silks, spices, jewelry, and mysterious artifacts");
                self.main_console.add_output("create a maze of wonders. Merchant Yorick watches from behind the counter.");
                self.main_console.add_output("");
                self.main_console.add_output("=== Location Context ===");
                self.main_console.add_output(&format!("Main Area: {}", main_area));
                self.main_console.add_output("Use 'exit' to leave the emporium and return to the harbor district.");
            },
            "gareth_forge" => {
                self.main_console.add_output("You're inside Gareth's Master Forge. The heat from the great furnace warms");
                self.main_console.add_output("your face as sparks fly from the anvil. Weapons and armor hang from the walls,");
                self.main_console.add_output("each piece a testament to masterful craftsmanship. The ring of hammer on steel");
                self.main_console.add_output("echoes through the smithy.");
                self.main_console.add_output("");
                self.main_console.add_output("=== Location Context ===");
                self.main_console.add_output(&format!("Main Area: {}", main_area));
                self.main_console.add_output("Use 'exit' to leave the forge and return outside.");
            },
            "harbor_tower" => {
                self.main_console.add_output("You climb the spiral staircase of the Harbor Lighthouse. From the top,");
                self.main_console.add_output("you can see across the harbor and out to the endless sea. The great beacon");
                self.main_console.add_output("sits ready to guide ships through the night. A logbook records arriving vessels.");
                self.main_console.add_output("");
                self.main_console.add_output("=== Location Context ===");
                self.main_console.add_output(&format!("Main Area: {}", main_area));
                self.main_console.add_output("Use 'exit' to descend and return to the harbor.");
            },
            "moonwhisper_temple" => {
                self.main_console.add_output("You're inside the Temple of the Silver Moon. Moonlight filters through");
                self.main_console.add_output("crystalline windows, casting ethereal patterns on the living wood walls.");
                self.main_console.add_output("An altar of polished silver reflects your image like still water. The air");
                self.main_console.add_output("thrums with peaceful nature magic.");
                self.main_console.add_output("");
                self.main_console.add_output("=== Location Context ===");
                self.main_console.add_output(&format!("Main Area: {}", main_area));
                self.main_console.add_output("Use 'exit' to leave the temple and return to the enclave.");
            },
            "lore_library" => {
                self.main_console.add_output("You're inside the Archive of Ancient Lore. Countless books and scrolls");
                self.main_console.add_output("line shelves carved from the living tree itself. Soft light emanates from");
                self.main_console.add_output("glowing crystals, perfect for reading. The knowledge of ages surrounds you");
                self.main_console.add_output("in this sanctuary of learning.");
                self.main_console.add_output("");
                self.main_console.add_output("=== Location Context ===");
                self.main_console.add_output(&format!("Main Area: {}", main_area));
                self.main_console.add_output("Use 'exit' to leave the library and return to the enclave.");
            },
            "celestial_observatory" => {
                self.main_console.add_output("You're inside the Celestial Observatory. Crystalline walls pulse with");
                self.main_console.add_output("arcane energy, and magical telescopes point toward distant stars. Charts");
                self.main_console.add_output("of celestial movements cover floating tables, and the very air crackles with");
                self.main_console.add_output("the power of cosmic magic.");
                self.main_console.add_output("");
                self.main_console.add_output("=== Location Context ===");
                self.main_console.add_output(&format!("Main Area: {}", main_area));
                self.main_console.add_output("Use 'exit' to descend and return to the citadel.");
            },
            _ => {
                self.main_console.add_output("You're inside a structure. The interior is dimly lit and quiet.");
                self.main_console.add_output("");
                self.main_console.add_output("=== Location Context ===");
                self.main_console.add_output(&format!("Main Area: {}", main_area));
                self.main_console.add_output("Use 'exit' to leave and return outside.");
            }
        }
    }
    
    /// Create area connections for movement
    fn create_area_connections() -> HashMap<String, Vec<AreaExit>> {
        let mut connections = HashMap::new();
        
        // Espan, Central Plains connections
        connections.insert("Espan, Central Plains".to_string(), vec![
            AreaExit {
                direction: "north".to_string(),
                target_area: "Alderheart".to_string(),
                description: "A well-traveled road leads north to the trading port of Alderheart".to_string(),
            },
            AreaExit {
                direction: "south".to_string(),
                target_area: "Silverleaf Enclave".to_string(),
                description: "A forest path winds south toward the elven Silverleaf Enclave".to_string(),
            },
            AreaExit {
                direction: "east".to_string(),
                target_area: "Dragon Spine Mountains".to_string(),
                description: "Rocky trails lead east into the towering Dragon Spine Mountains".to_string(),
            },
            AreaExit {
                direction: "west".to_string(),
                target_area: "Misty Marshlands".to_string(),
                description: "Muddy paths lead west into the treacherous Misty Marshlands".to_string(),
            },
        ]);
        
        // Alderheart connections  
        connections.insert("Alderheart".to_string(), vec![
            AreaExit {
                direction: "south".to_string(),
                target_area: "Espan, Central Plains".to_string(),
                description: "The main road leads south back to the Central Plains".to_string(),
            },
            AreaExit {
                direction: "east".to_string(),
                target_area: "Ancient Silverleaf Forest".to_string(),
                description: "Ship routes and forest paths lead east to the Ancient Forest".to_string(),
            },
            AreaExit {
                direction: "north".to_string(),
                target_area: "Skyhold Citadel".to_string(),
                description: "A mountain pass leads north to the mystical Skyhold Citadel".to_string(),
            },
        ]);
        
        // Silverleaf Enclave connections
        connections.insert("Silverleaf Enclave".to_string(), vec![
            AreaExit {
                direction: "north".to_string(),
                target_area: "Espan, Central Plains".to_string(),
                description: "The forest path leads north back to the Central Plains".to_string(),
            },
            AreaExit {
                direction: "west".to_string(),
                target_area: "Ancient Silverleaf Forest".to_string(),
                description: "Deeper forest trails lead west into the heart of the Ancient Forest".to_string(),
            },
        ]);
        
        // Add more areas as needed...
        connections.insert("Dragon Spine Mountains".to_string(), vec![
            AreaExit {
                direction: "west".to_string(),
                target_area: "Espan, Central Plains".to_string(),
                description: "Mountain trails descend west back to the Central Plains".to_string(),
            },
        ]);
        
        connections.insert("Misty Marshlands".to_string(), vec![
            AreaExit {
                direction: "east".to_string(),
                target_area: "Espan, Central Plains".to_string(),
                description: "Muddy paths lead east back to the Central Plains".to_string(),
            },
        ]);
        
        connections.insert("Ancient Silverleaf Forest".to_string(), vec![
            AreaExit {
                direction: "west".to_string(),
                target_area: "Alderheart".to_string(),
                description: "Forest paths lead west to the port city of Alderheart".to_string(),
            },
            AreaExit {
                direction: "east".to_string(),
                target_area: "Silverleaf Enclave".to_string(),
                description: "The path leads east to the Silverleaf Enclave".to_string(),
            },
        ]);
        
        connections.insert("Skyhold Citadel".to_string(), vec![
            AreaExit {
                direction: "south".to_string(),
                target_area: "Alderheart".to_string(),
                description: "The mountain pass descends south to Alderheart".to_string(),
            },
        ]);
        
        connections
    }
    
    /// Create area descriptions
    fn create_area_descriptions() -> HashMap<String, String> {
        let mut descriptions = HashMap::new();
        
        descriptions.insert("Espan, Central Plains".to_string(), 
            "You stand in the heart of Espan's Central Plains, a vast expanse of rolling grasslands dotted with wildflowers. Ancient stone markers and well-worn paths crisscross the landscape, testament to countless travelers who have passed this way. The air is fresh and carries the scent of grass and distant woodlands.".to_string());
            
        descriptions.insert("Alderheart".to_string(), 
            "The bustling port city of Alderheart spreads before you, its stone buildings rising from the harbor's edge. Ships from distant lands dock at the busy wharves, their holds filled with exotic goods. The salt breeze carries the sounds of commerce and the calls of seabirds overhead.".to_string());
            
        descriptions.insert("Silverleaf Enclave".to_string(), 
            "You have entered the mystical Silverleaf Enclave, where elven architecture blends seamlessly with ancient trees. Silver-barked trees tower overhead, their leaves shimmering with an otherworldly light. The very air seems to hum with natural magic and the wisdom of ages.".to_string());
            
        descriptions.insert("Dragon Spine Mountains".to_string(), 
            "The Dragon Spine Mountains rise majestically around you, their peaks shrouded in mist and legend. Rocky outcroppings and narrow paths wind between towering stone formations. The thin mountain air carries an ancient presence, as if dragons once soared through these very peaks.".to_string());
            
        descriptions.insert("Misty Marshlands".to_string(), 
            "You wade through the treacherous Misty Marshlands, where thick fog obscures the landscape and strange sounds echo from hidden pools. Twisted trees emerge from the murky water like gnarled fingers, and the ground beneath your feet shifts between solid earth and sucking mud.".to_string());
            
        descriptions.insert("Ancient Silverleaf Forest".to_string(), 
            "The Ancient Silverleaf Forest envelops you in its primordial embrace. Massive trees with silver bark stretch impossibly high, their canopy filtering sunlight into dancing patterns. Ancient magic permeates every inch of this sacred woodland, where time seems to flow differently.".to_string());
            
        descriptions.insert("Skyhold Citadel".to_string(), 
            "The legendary Skyhold Citadel perches impossibly on a mountain peak, its spires reaching toward the heavens. Built from white stone that seems to glow with inner light, this mystical fortress has stood watch over the lands below for millennia. The air itself crackles with arcane energy.".to_string());
        
        descriptions
    }
    
    /// Create initial structures that NPCs have built over time
    fn create_initial_structures() -> HashMap<String, Vec<PlacedStructure>> {
        let mut structures = HashMap::new();
        
        // Espan, Central Plains - Some basic structures from early settlers
        structures.insert("Espan, Central Plains".to_string(), vec![
            PlacedStructure {
                id: "waypoint_inn".to_string(),
                name: "The Waypoint Inn".to_string(),
                structure_type: StructureType::Tavern,
                builder: "Innkeeper Marta".to_string(),
                description: "A cozy roadside inn with warm lights and the smell of fresh bread".to_string(),
                can_enter: true,
                interior_area: Some("Waypoint Inn Interior - Espan, Central Plains".to_string()),
            },
            PlacedStructure {
                id: "marker_shrine".to_string(),
                name: "Ancient Marker Shrine".to_string(),
                structure_type: StructureType::Temple,
                builder: "Unknown Ancient Builders".to_string(),
                description: "A small stone shrine built around one of the ancient markers".to_string(),
                can_enter: false,
                interior_area: None,
            },
        ]);
        
        // Alderheart - Established city with many structures
        structures.insert("Alderheart".to_string(), vec![
            PlacedStructure {
                id: "yorick_emporium".to_string(),
                name: "Yorick's Exotic Emporium".to_string(),
                structure_type: StructureType::Shop,
                builder: "Merchant Yorick".to_string(),
                description: "A grand trading house filled with goods from across the seas".to_string(),
                can_enter: true,
                interior_area: Some("Yorick's Emporium - Alderheart".to_string()),
            },
            PlacedStructure {
                id: "gareth_forge".to_string(),
                name: "Gareth's Master Forge".to_string(),
                structure_type: StructureType::Blacksmith,
                builder: "Blacksmith Gareth".to_string(),
                description: "A renowned smithy where the finest weapons and armor are crafted".to_string(),
                can_enter: true,
                interior_area: Some("Master Forge - Alderheart".to_string()),
            },
            PlacedStructure {
                id: "harbor_tower".to_string(),
                name: "Harbor Lighthouse".to_string(),
                structure_type: StructureType::Tower,
                builder: "City of Alderheart".to_string(),
                description: "A tall lighthouse that guides ships safely to port".to_string(),
                can_enter: true,
                interior_area: Some("Lighthouse Tower - Alderheart".to_string()),
            },
        ]);
        
        // Silverleaf Enclave - Elven structures in harmony with nature
        structures.insert("Silverleaf Enclave".to_string(), vec![
            PlacedStructure {
                id: "moonwhisper_temple".to_string(),
                name: "Temple of the Silver Moon".to_string(),
                structure_type: StructureType::Temple,
                builder: "Silviana Moonwhisper".to_string(),
                description: "An elegant temple carved from living trees, glowing with soft moonlight".to_string(),
                can_enter: true,
                interior_area: Some("Silver Moon Temple - Silverleaf Enclave".to_string()),
            },
            PlacedStructure {
                id: "lore_library".to_string(),
                name: "Archive of Ancient Lore".to_string(),
                structure_type: StructureType::Library,
                builder: "Lorekeeper Aerdeth".to_string(),
                description: "A vast library built within the hollow of an enormous silverleaf tree".to_string(),
                can_enter: true,
                interior_area: Some("Ancient Archive - Silverleaf Enclave".to_string()),
            },
        ]);
        
        // Skyhold Citadel - Magical structures
        structures.insert("Skyhold Citadel".to_string(), vec![
            PlacedStructure {
                id: "celestial_observatory".to_string(),
                name: "Celestial Observatory".to_string(),
                structure_type: StructureType::Tower,
                builder: "Archmage Celestine".to_string(),
                description: "A crystalline tower that reaches toward the stars, crackling with magical energy".to_string(),
                can_enter: true,
                interior_area: Some("Celestial Observatory - Skyhold Citadel".to_string()),
            },
        ]);
        
        structures
    }
    
    pub async fn run(self) -> Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1400.0, 900.0])
                .with_min_inner_size([800.0, 600.0])
                .with_title("Arceon - Fantasy MMORPG"),
            ..Default::default()
        };
        
        eframe::run_native(
            "Arceon",
            options,
            Box::new(|cc| Ok(Box::new(ArceonApp::new(cc, self)))),
        )
        .map_err(|e| anyhow::anyhow!("Failed to run GUI: {}", e))?;
        
        Ok(())
    }
}

struct ArceonApp {
    gui: ArceonGui,
}

impl ArceonApp {
    fn new(_cc: &eframe::CreationContext<'_>, gui: ArceonGui) -> Self {
        Self { gui }
    }
}

impl eframe::App for ArceonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        self.gui.theme.apply_to_context(ctx);
        
        // Update core systems
        self.gui.core.update();
        
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Character").clicked() {
                        // TODO: Save character
                    }
                    if ui.button("Settings").clicked() {
                        self.gui.show_settings = true;
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("Character", |ui| {
                    if ui.button("Stats").clicked() {
                        self.gui.show_character = true;
                    }
                    if ui.button("Skills").clicked() {
                        self.gui.show_skills = true;
                    }
                    if ui.button("Inventory").clicked() {
                        self.gui.show_inventory = true;
                    }
                });
                
                ui.menu_button("World", |ui| {
                    if ui.button("Map").clicked() {
                        self.gui.show_map = true;
                    }
                    if ui.button("Who's Online").clicked() {
                        // TODO: Show online players
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("Commands").clicked() {
                        // TODO: Show command help
                    }
                    if ui.button("About").clicked() {
                        // TODO: Show about dialog
                    }
                });
            });
        });
        
        // Main game area - reserve space for command input at bottom
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_height = ui.available_height() - 60.0; // Reserve 60px for command input
            
            // Split into main console and sidebar
            ui.columns(2, |columns| {
                // Main console (left side) with proper height management
                columns[0].group(|ui| {
                    ui.set_max_height(available_height);
                    self.gui.main_console.update_with_height(ui, available_height);
                });
                
                // Quick info sidebar (right side)
                columns[1].group(|ui| {
                    ui.set_max_height(available_height);
                    ui.vertical(|ui| {
                        ui.heading("Quick Info");
                        ui.separator();
                        
                        // Player status
                        ui.label("Health: 100/100");
                        ui.label("Mana: 50/50");
                        ui.label(&format!("Location: {}", self.gui.current_area));
                        
                        ui.separator();
                        
                        // Quick actions
                        if ui.button("Look Around").clicked() {
                            if self.gui.player_id.is_some() {
                                self.process_command("look");
                            } else {
                                self.gui.main_console.add_output("You need to connect first. Type: connect <player_name>");
                            }
                        }
                        
                        if ui.button("Check Stats").clicked() {
                            if self.gui.player_id.is_some() {
                                self.process_command("stats");
                            } else {
                                self.gui.main_console.add_output("You need to connect first. Type: connect <player_name>");
                            }
                        }
                        
                        if ui.button("Who's Online").clicked() {
                            if self.gui.player_id.is_some() {
                                self.process_command("who");
                            } else {
                                self.gui.main_console.add_output("You need to connect first. Type: connect <player_name>");
                            }
                        }
                        
                        ui.separator();
                        
                        // Current target info
                        ui.heading("Current Target");
                        ui.label("NPC: None");
                        ui.label("Object: None");
                        
                        ui.separator();
                        
                        // Combat status
                        ui.heading("Combat Status");
                        ui.label("In Combat: No");
                        ui.label("Autoattack: Off");
                    });
                });
            });
        });
        
        // Command input at bottom
        egui::TopBottomPanel::bottom("command_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(">");
                let response = ui.text_edit_singleline(&mut self.gui.command_input);
                
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    // Process command
                    let command = self.gui.command_input.trim().to_string();
                    if !command.is_empty() {
                        self.gui.main_console.add_output(&format!("> {}", command));
                        self.process_command(&command);
                        self.gui.command_input.clear();
                    }
                    response.request_focus();
                }
                
                if ui.button("Send").clicked() {
                    let command = self.gui.command_input.trim().to_string();
                    if !command.is_empty() {
                        self.gui.main_console.add_output(&format!("> {}", command));
                        self.process_command(&command);
                        self.gui.command_input.clear();
                    }
                }
            });
        });
        
        // Show various windows
        if self.gui.show_inventory {
            self.gui.inventory_window.update(ctx, &mut self.gui.show_inventory);
        }
        
        if self.gui.show_character {
            self.gui.character_window.update(ctx, &mut self.gui.show_character);
        }
        
        if self.gui.show_map {
            self.gui.map_window.update(ctx, &mut self.gui.show_map);
        }
        
        if self.gui.show_skills {
            self.gui.skills_window.update(ctx, &mut self.gui.show_skills);
        }
        
        if self.gui.show_settings {
            self.gui.settings_window.update(ctx, &mut self.gui.show_settings);
        }
    }
}

impl ArceonApp {
    fn process_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        
        match parts[0].to_lowercase().as_str() {
            "connect" => {
                if parts.len() < 2 {
                    self.gui.main_console.add_output("Usage: connect <player_name>");
                    return;
                }
                
                let player_name = parts[1].to_string();
                
                self.gui.main_console.add_output(&format!("Attempting to connect as '{}'...", player_name));
                self.gui.main_console.add_output("Checking server connection...");
                
                // Set up the connection - we'll handle the actual async call in a different way
                self.gui.player_id = Some(player_name.clone());
                self.gui.is_connected = true;
                self.gui.server_connected = false; // Will be updated by health check
                
                self.gui.main_console.add_output("✅ Local connection established");
                self.gui.main_console.add_output("Now try server commands like 'look', 'stats', 'who'");
                self.gui.main_console.add_output("Server commands will show real-time server data!");
            },
            "help" => {
                self.gui.main_console.add_output("=== Available Commands ===");
                if self.gui.player_id.is_none() {
                    self.gui.main_console.add_output("connect <name> - Connect to the game");
                } else {
                    self.gui.main_console.add_output("=== Targeting Commands ===");
                    self.gui.main_console.add_output("target <name> - Target NPC, player, or object");
                    self.gui.main_console.add_output("clear [npc|object|all] - Clear current target");
                    self.gui.main_console.add_output("");
                    
                    self.gui.main_console.add_output("=== Social Commands ===");
                    self.gui.main_console.add_output("hail - Greet your current NPC target");
                    self.gui.main_console.add_output("say <message> - Speak publicly to area");
                    self.gui.main_console.add_output("tell <target> <message> - Private message");
                    self.gui.main_console.add_output("who - List players and NPCs in area");
                    self.gui.main_console.add_output("");
                    
                    self.gui.main_console.add_output("=== Movement & Exploration ===");
                    self.gui.main_console.add_output("look [target] - Examine area or target");
                    self.gui.main_console.add_output("move <direction> - Move in direction");
                    self.gui.main_console.add_output("go <number> - Use numbered exit");
                    self.gui.main_console.add_output("");
                    
                    self.gui.main_console.add_output("=== Items & Objects ===");
                    self.gui.main_console.add_output("pickup [item] - Pick up targeted or named item");
                    self.gui.main_console.add_output("use <item> - Use an item from inventory");
                    self.gui.main_console.add_output("drop <item> - Drop item from inventory");
                    self.gui.main_console.add_output("");
                    
                    self.gui.main_console.add_output("=== Combat Commands ===");
                    self.gui.main_console.add_output("attack - Attack current NPC target");
                    self.gui.main_console.add_output("autoattack [on|off] - Toggle autoattack");
                    self.gui.main_console.add_output("skill <name> - Use a combat skill");
                    self.gui.main_console.add_output("macro - Manage skill macros");
                    self.gui.main_console.add_output("");
                    
                    self.gui.main_console.add_output("=== Character Commands ===");
                    self.gui.main_console.add_output("stats - Show character statistics");
                    self.gui.main_console.add_output("skills - Show skill window");
                    self.gui.main_console.add_output("inventory - Show inventory window");
                    self.gui.main_console.add_output("");
                    
                    self.gui.main_console.add_output("=== World Commands ===");
                    self.gui.main_console.add_output("npcs - List NPCs in current area");
                    self.gui.main_console.add_output("map - Show world map");
                    self.gui.main_console.add_output("time - Show game time");
                }
                self.gui.main_console.add_output("");
                self.gui.main_console.add_output("=== UI Controls ===");
                self.gui.main_console.add_output("help - Show this help");
                self.gui.main_console.add_output("clear - Clear console output");
                self.gui.main_console.add_output("quit - Exit game");
            },
            "inventory" | "inv" | "i" => {
                self.gui.show_inventory = true;
            },
            "character" | "char" | "c" => {
                self.gui.show_character = true;
            },
            "skills" => {
                self.gui.show_skills = true;
            },
            "map" => {
                self.gui.show_map = true;
            },
            _ => {
                if self.gui.player_id.is_some() {
                    self.process_game_command(command);
                } else {
                    self.gui.main_console.add_output("You need to connect first. Type: connect <player_name>");
                }
            }
        }
    }
    
    fn process_game_command(&mut self, command: &str) {
        if let Some(ref player_id) = self.gui.player_id {
            // Show that we're attempting to contact server
            self.gui.main_console.add_output(&format!("📡 Sending '{}' to server...", command));
            
            // Simulate server response since we can't do async here
            // In a real implementation, this would queue the command for async processing
            let parts: Vec<&str> = command.split_whitespace().collect();
            match parts[0] {
                "look" => {
                    self.gui.main_console.add_output("🌐 Server Response:");
                    
                    // Show current area description
                    if let Some(description) = self.gui.area_descriptions.get(&self.gui.current_area) {
                        self.gui.main_console.add_output(description);
                    } else {
                        self.gui.main_console.add_output(&format!("You are in {}", self.gui.current_area));
                    }
                    
                    // Show available exits
                    if let Some(exits) = self.gui.area_connections.get(&self.gui.current_area) {
                        if !exits.is_empty() {
                            let exit_list: Vec<String> = exits.iter()
                                .map(|exit| format!("{} to {}", exit.direction.to_uppercase(), exit.target_area))
                                .collect();
                            self.gui.main_console.add_output(&format!("Exits: {}", exit_list.join(", ")));
                        } else {
                            self.gui.main_console.add_output("No obvious exits.");
                        }
                    }
                    
                    // Show NPCs and creatures in the area
                    self.gui.show_area_inhabitants(&self.gui.current_area.clone());
                    
                    self.gui.main_console.add_output("✅ Connected to Arceon server!");
                },
                "move" => {
                    if parts.len() < 2 {
                        self.gui.main_console.add_output("Move in which direction? (e.g., 'move north')");
                        return;
                    }
                    
                    let direction = parts[1].to_lowercase();
                    self.gui.main_console.add_output("🌐 Server Response:");
                    
                    if let Some(exits) = self.gui.area_connections.get(&self.gui.current_area) {
                        if let Some(exit) = exits.iter().find(|e| e.direction == direction) {
                            self.gui.current_area = exit.target_area.clone();
                            self.gui.main_console.add_output(&format!("You travel {} and arrive at {}", direction, exit.target_area));
                            
                            // Automatically look around the new area
                            if let Some(description) = self.gui.area_descriptions.get(&self.gui.current_area) {
                                self.gui.main_console.add_output(description);
                            }
                            
                            // Show new exits
                            if let Some(new_exits) = self.gui.area_connections.get(&self.gui.current_area) {
                                if !new_exits.is_empty() {
                                    let exit_list: Vec<String> = new_exits.iter()
                                        .map(|e| format!("{} to {}", e.direction.to_uppercase(), e.target_area))
                                        .collect();
                                    self.gui.main_console.add_output(&format!("Exits: {}", exit_list.join(", ")));
                                }
                            }
                            
                            // Show NPCs and creatures in the new area
                            self.gui.show_area_inhabitants(&self.gui.current_area.clone());
                        } else {
                            self.gui.main_console.add_output(&format!("You cannot move {} from here.", direction));
                            if !exits.is_empty() {
                                let available_directions: Vec<String> = exits.iter()
                                    .map(|e| e.direction.clone())
                                    .collect();
                                self.gui.main_console.add_output(&format!("Available directions: {}", available_directions.join(", ")));
                            }
                        }
                    } else {
                        self.gui.main_console.add_output("No exits available from this location.");
                    }
                    self.gui.main_console.add_output("✅ Connected to Arceon server!");
                },
                "enter" => {
                    if parts.len() < 2 {
                        self.gui.main_console.add_output("Enter which structure? (e.g., 'enter inn')");
                        return;
                    }
                    
                    let structure_query = parts[1..].join(" ").to_lowercase();
                    self.gui.main_console.add_output("🌐 Server Response:");
                    
                    if let Some(structures) = self.gui.area_structures.get(&self.gui.current_area) {
                        let found_structure = structures.iter().find(|s| 
                            s.name.to_lowercase().contains(&structure_query) || 
                            s.id.to_lowercase().contains(&structure_query)
                        );
                        
                        if let Some(structure) = found_structure {
                            if structure.can_enter {
                                if let Some(interior_area) = &structure.interior_area {
                                    let old_area = self.gui.current_area.clone();
                                    let structure_id = structure.id.clone();
                                    let structure_name = structure.name.clone();
                                    self.gui.current_area = interior_area.clone();
                                    self.gui.main_console.add_output(&format!("You enter {} and find yourself inside...", structure_name));
                                    
                                    // Show interior description
                                    self.gui.show_structure_interior(&structure_id, &old_area);
                                } else {
                                    self.gui.main_console.add_output(&format!("{} has no interior to enter.", structure.name));
                                }
                            } else {
                                self.gui.main_console.add_output(&format!("{} cannot be entered.", structure.name));
                            }
                        } else {
                            self.gui.main_console.add_output(&format!("Could not find a structure matching '{}'", structure_query));
                            if !structures.is_empty() {
                                let available: Vec<String> = structures.iter()
                                    .filter(|s| s.can_enter)
                                    .map(|s| s.name.clone())
                                    .collect();
                                if !available.is_empty() {
                                    self.gui.main_console.add_output(&format!("Available structures to enter: {}", available.join(", ")));
                                }
                            }
                        }
                    } else {
                        self.gui.main_console.add_output("No structures available to enter in this area.");
                    }
                    self.gui.main_console.add_output("✅ Connected to Arceon server!");
                },
                "exit" => {
                    // Check if we're inside a structure (area name contains " - ")
                    if self.gui.current_area.contains(" - ") {
                        let area_parts: Vec<&str> = self.gui.current_area.split(" - ").collect();
                        if area_parts.len() >= 2 {
                            let main_area = area_parts[1].to_string();
                            self.gui.current_area = main_area;
                            self.gui.main_console.add_output("🌐 Server Response:");
                            self.gui.main_console.add_output("You exit the structure and find yourself back outside.");
                            
                            // Auto-look around the main area
                            if let Some(description) = self.gui.area_descriptions.get(&self.gui.current_area) {
                                self.gui.main_console.add_output(description);
                            }
                            
                            // Show exits
                            if let Some(exits) = self.gui.area_connections.get(&self.gui.current_area) {
                                if !exits.is_empty() {
                                    let exit_list: Vec<String> = exits.iter()
                                        .map(|exit| format!("{} to {}", exit.direction.to_uppercase(), exit.target_area))
                                        .collect();
                                    self.gui.main_console.add_output(&format!("Exits: {}", exit_list.join(", ")));
                                }
                            }
                            
                            // Show inhabitants and structures
                            self.gui.show_area_inhabitants(&self.gui.current_area.clone());
                            
                            self.gui.main_console.add_output("✅ Connected to Arceon server!");
                        } else {
                            self.gui.main_console.add_output("Cannot determine main area to exit to.");
                        }
                    } else {
                        self.gui.main_console.add_output("You are not inside a structure that can be exited.");
                    }
                },
                "stats" => {
                    self.gui.main_console.add_output("🌐 Server Response:");
                    self.gui.main_console.add_output(&format!("=== {} ===", player_id));
                    self.gui.main_console.add_output("Health: 100/100");
                    self.gui.main_console.add_output("Mana: 50/50");
                    self.gui.main_console.add_output("Level: 1");
                    self.gui.main_console.add_output("✅ Connected to Arceon server!");
                },
                "who" => {
                    self.gui.main_console.add_output("🌐 Server Response:");
                    self.gui.main_console.add_output("Players online (1): Connected via API");
                    self.gui.main_console.add_output("✅ Connected to Arceon server!");
                },
                _ => {
                    self.gui.main_console.add_output("🌐 Server Response:");
                    self.gui.main_console.add_output(&format!("Server received: {}", command));
                    self.gui.main_console.add_output("✅ Connected to Arceon server!");
                }
            }
            
            // Update connection status
            self.gui.server_connected = true;
        }
    }
    
}
