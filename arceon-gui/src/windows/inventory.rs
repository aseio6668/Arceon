use egui::{Context, Window};

pub struct InventoryWindow {
    // TODO: Add inventory state
}

impl InventoryWindow {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn update(&mut self, ctx: &Context, open: &mut bool) {
        Window::new("Inventory")
            .open(open)
            .default_size([400.0, 500.0])
            .show(ctx, |ui| {
                ui.label("Your inventory is empty.");
                // TODO: Implement inventory grid
            });
    }
}

pub struct CharacterWindow;

impl CharacterWindow {
    pub fn new() -> Self {
        Self
    }
    
    pub fn update(&mut self, ctx: &Context, open: &mut bool) {
        Window::new("Character")
            .open(open)
            .default_size([350.0, 400.0])
            .show(ctx, |ui| {
                ui.label("Character stats will appear here.");
                // TODO: Implement character sheet
            });
    }
}

pub struct MapWindow;

impl MapWindow {
    pub fn new() -> Self {
        Self
    }
    
    pub fn update(&mut self, ctx: &Context, open: &mut bool) {
        Window::new("World Map")
            .open(open)
            .default_size([600.0, 450.0])
            .show(ctx, |ui| {
                ui.label("Map of Espan will be displayed here.");
                // TODO: Implement world map
            });
    }
}

pub struct SkillsWindow;

impl SkillsWindow {
    pub fn new() -> Self {
        Self
    }
    
    pub fn update(&mut self, ctx: &Context, open: &mut bool) {
        Window::new("Skills")
            .open(open)
            .default_size([450.0, 500.0])
            .show(ctx, |ui| {
                ui.label("Skill progression will be shown here.");
                // TODO: Implement skills display
            });
    }
}

pub struct SettingsWindow;

impl SettingsWindow {
    pub fn new() -> Self {
        Self
    }
    
    pub fn update(&mut self, ctx: &Context, open: &mut bool) {
        Window::new("Settings")
            .open(open)
            .default_size([400.0, 300.0])
            .show(ctx, |ui| {
                ui.label("Game settings will be configurable here.");
                // TODO: Implement settings
            });
    }
}
