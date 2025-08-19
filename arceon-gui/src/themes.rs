use egui::{Context, Color32, Rounding, Shadow};

pub struct ArceonTheme {
    pub name: String,
    pub background_color: Color32,
    pub panel_color: Color32,
    pub text_color: Color32,
    pub accent_color: Color32,
}

impl ArceonTheme {
    pub fn dark_fantasy() -> Self {
        Self {
            name: "Dark Fantasy".to_string(),
            background_color: Color32::from_rgb(20, 20, 25),
            panel_color: Color32::from_rgb(35, 35, 45),
            text_color: Color32::from_rgb(220, 220, 220),
            accent_color: Color32::from_rgb(180, 140, 70), // Gold accent
        }
    }
    
    pub fn apply_to_context(&self, ctx: &Context) {
        let mut style = (*ctx.style()).clone();
        
        // Colors
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(self.text_color);
        style.visuals.panel_fill = self.panel_color;
        style.visuals.window_fill = self.panel_color;
        style.visuals.extreme_bg_color = self.background_color;
        
        // Rounded corners for fantasy feel
        style.visuals.window_rounding = Rounding::same(8.0);
        style.visuals.menu_rounding = Rounding::same(6.0);
        
        // Subtle shadows
        style.visuals.window_shadow = Shadow {
            offset: egui::Vec2::new(2.0, 4.0),
            blur: 8.0,
            spread: 0.0,
            color: egui::Color32::from_black_alpha(80),
        };
        
        ctx.set_style(style);
    }
}
