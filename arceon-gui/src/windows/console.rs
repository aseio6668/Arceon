use egui::{ScrollArea, Ui};
use std::collections::VecDeque;

pub struct ConsoleWindow {
    output_buffer: VecDeque<String>,
    max_lines: usize,
    auto_scroll: bool,
}

impl ConsoleWindow {
    pub fn new() -> Self {
        Self {
            output_buffer: VecDeque::new(),
            max_lines: 1000,
            auto_scroll: true,
        }
    }
    
    pub fn add_output(&mut self, text: &str) {
        self.output_buffer.push_back(text.to_string());
        
        // Limit buffer size
        if self.output_buffer.len() > self.max_lines {
            self.output_buffer.pop_front();
        }
    }
    
    pub fn update(&mut self, ui: &mut Ui) {
        self.update_with_height(ui, ui.available_height());
    }
    
    pub fn update_with_height(&mut self, ui: &mut Ui, max_height: f32) {
        ui.vertical(|ui| {
            ui.heading("Game Console");
            ui.separator();
            
            // Calculate available height for scroll area (subtract header and separator)
            let scroll_height = max_height - 60.0; // Reserve space for heading and separator
            
            // Output area with controlled height
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(scroll_height)
                .show(ui, |ui| {
                    for line in &self.output_buffer {
                        ui.label(line);
                    }
                    
                    if self.auto_scroll {
                        ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                    }
                });
        });
    }
}
