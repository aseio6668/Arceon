use anyhow::Result;
use crate::{PlatformInfo, PlatformType};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tracing::{info, warn};

/// Mobile-specific UI manager
pub struct MobileUIManager {
    pub platform_info: PlatformInfo,
    pub ui_scale: f32,
    pub safe_area: SafeArea,
    pub orientation: Orientation,
    pub virtual_keyboard: VirtualKeyboard,
    pub hud_layout: HUDLayout,
    pub touch_ui_elements: Vec<TouchUIElement>,
}

/// Safe area insets for mobile devices (notches, home indicators)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeArea {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// Device orientation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    PortraitUpsideDown,
    LandscapeLeft,
    LandscapeRight,
}

/// Virtual keyboard state and configuration
#[derive(Debug, Clone)]
pub struct VirtualKeyboard {
    pub visible: bool,
    pub height: f32,
    pub keyboard_type: KeyboardType,
    pub auto_capitalize: bool,
    pub secure_text_entry: bool,
}

/// Types of virtual keyboards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyboardType {
    Default,
    Numeric,
    Email,
    URL,
    Password,
    Search,
}

/// HUD layout configuration for different screen sizes
#[derive(Debug, Clone)]
pub struct HUDLayout {
    pub minimap_position: UIPosition,
    pub health_bar_position: UIPosition,
    pub mana_bar_position: UIPosition,
    pub chat_position: UIPosition,
    pub inventory_button_position: UIPosition,
    pub skill_bar_position: UIPosition,
    pub menu_button_position: UIPosition,
    pub action_buttons: Vec<ActionButton>,
}

/// UI element positioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPosition {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub anchor: Anchor,
}

/// Anchor points for UI elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// Touch-specific UI elements
#[derive(Debug, Clone)]
pub struct TouchUIElement {
    pub id: Uuid,
    pub element_type: TouchElementType,
    pub position: UIPosition,
    pub visible: bool,
    pub enabled: bool,
    pub opacity: f32,
    pub haptic_feedback: bool,
}

/// Types of touch UI elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TouchElementType {
    VirtualJoystick {
        sensitivity: f32,
        dead_zone: f32,
        return_to_center: bool,
    },
    TouchButton {
        label: String,
        icon: Option<String>,
        cooldown: Option<f32>,
    },
    SwipeArea {
        direction: SwipeDirection,
        min_distance: f32,
    },
    PinchArea {
        min_scale: f32,
        max_scale: f32,
    },
    DragArea {
        constrain_to_bounds: bool,
    },
}

/// Swipe directions for touch gestures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
    Any,
}

/// Action buttons for mobile gameplay
#[derive(Debug, Clone)]
pub struct ActionButton {
    pub id: Uuid,
    pub label: String,
    pub icon: Option<String>,
    pub position: UIPosition,
    pub action: GameAction,
    pub cooldown: Option<f32>,
    pub hotkey: Option<String>,
}

/// Game actions that can be triggered by UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameAction {
    Attack,
    UseSkill(String),
    UseItem(Uuid),
    Interact,
    Jump,
    Run,
    Sneak,
    OpenInventory,
    OpenMenu,
    OpenChat,
    OpenMap,
    OpenSkills,
    OpenQuests,
    ToggleAFK,
    EmergencyTeleport,
}

impl MobileUIManager {
    /// Create new mobile UI manager
    pub async fn new(platform_info: &PlatformInfo) -> Result<Self> {
        info!("📱 Initializing Mobile UI Manager");

        let ui_scale = Self::calculate_ui_scale(platform_info);
        let safe_area = Self::detect_safe_area(platform_info).await?;
        let orientation = Self::detect_orientation(platform_info).await?;
        
        let virtual_keyboard = VirtualKeyboard {
            visible: false,
            height: 0.0,
            keyboard_type: KeyboardType::Default,
            auto_capitalize: true,
            secure_text_entry: false,
        };

        let hud_layout = Self::create_default_hud_layout(&orientation, &safe_area, ui_scale);
        let touch_ui_elements = Self::create_default_touch_elements(&orientation, ui_scale);

        info!("✅ Mobile UI initialized with scale: {:.2}, orientation: {:?}", ui_scale, orientation);

        Ok(Self {
            platform_info: platform_info.clone(),
            ui_scale,
            safe_area,
            orientation,
            virtual_keyboard,
            hud_layout,
            touch_ui_elements,
        })
    }

    /// Calculate appropriate UI scale based on device
    fn calculate_ui_scale(platform_info: &PlatformInfo) -> f32 {
        match &platform_info.platform_type {
            PlatformType::Mobile { .. } => {
                // Base scale on screen density and size
                let density_scale = platform_info.screen_density.clamp(0.75, 3.0);
                let size_factor = if platform_info.is_tablet { 0.9 } else { 1.1 };
                
                (density_scale * size_factor).clamp(0.8, 2.5)
            }
            PlatformType::Web { .. } => {
                // Web scales differently
                platform_info.screen_density.clamp(0.9, 2.0)
            }
            _ => 1.0, // Desktop default
        }
    }

    /// Detect device safe area (notches, home indicators)
    async fn detect_safe_area(platform_info: &PlatformInfo) -> Result<SafeArea> {
        match &platform_info.platform_type {
            PlatformType::Mobile { os } => match os {
                crate::MobileOS::iOS | crate::MobileOS::iPadOS => {
                    // iOS devices may have notches or home indicators
                    Ok(SafeArea {
                        top: if platform_info.device_model.contains("iPhone") { 44.0 } else { 20.0 },
                        bottom: 34.0, // Home indicator
                        left: 0.0,
                        right: 0.0,
                    })
                }
                crate::MobileOS::Android => {
                    // Android devices vary widely
                    Ok(SafeArea {
                        top: 24.0, // Status bar
                        bottom: 48.0, // Navigation bar
                        left: 0.0,
                        right: 0.0,
                    })
                }
                _ => Ok(SafeArea::default()),
            },
            _ => Ok(SafeArea::default()),
        }
    }

    /// Detect current device orientation
    async fn detect_orientation(platform_info: &PlatformInfo) -> Result<Orientation> {
        let (width, height) = platform_info.screen_size;
        
        // Simple orientation detection based on aspect ratio
        if width > height {
            Ok(Orientation::LandscapeRight)
        } else {
            Ok(Orientation::Portrait)
        }
    }

    /// Create default HUD layout based on orientation and safe areas
    fn create_default_hud_layout(orientation: &Orientation, safe_area: &SafeArea, scale: f32) -> HUDLayout {
        match orientation {
            Orientation::Portrait | Orientation::PortraitUpsideDown => {
                HUDLayout {
                    minimap_position: UIPosition {
                        x: -120.0 * scale,
                        y: safe_area.top + 20.0 * scale,
                        width: 100.0 * scale,
                        height: 100.0 * scale,
                        anchor: Anchor::TopRight,
                    },
                    health_bar_position: UIPosition {
                        x: 20.0 * scale,
                        y: safe_area.top + 20.0 * scale,
                        width: 200.0 * scale,
                        height: 24.0 * scale,
                        anchor: Anchor::TopLeft,
                    },
                    mana_bar_position: UIPosition {
                        x: 20.0 * scale,
                        y: safe_area.top + 50.0 * scale,
                        width: 200.0 * scale,
                        height: 20.0 * scale,
                        anchor: Anchor::TopLeft,
                    },
                    chat_position: UIPosition {
                        x: 20.0 * scale,
                        y: -200.0 * scale - safe_area.bottom,
                        width: 300.0 * scale,
                        height: 150.0 * scale,
                        anchor: Anchor::BottomLeft,
                    },
                    inventory_button_position: UIPosition {
                        x: -70.0 * scale,
                        y: -70.0 * scale - safe_area.bottom,
                        width: 50.0 * scale,
                        height: 50.0 * scale,
                        anchor: Anchor::BottomRight,
                    },
                    skill_bar_position: UIPosition {
                        x: 0.0,
                        y: -120.0 * scale - safe_area.bottom,
                        width: 320.0 * scale,
                        height: 60.0 * scale,
                        anchor: Anchor::BottomCenter,
                    },
                    menu_button_position: UIPosition {
                        x: 20.0 * scale,
                        y: safe_area.top + 20.0 * scale,
                        width: 40.0 * scale,
                        height: 40.0 * scale,
                        anchor: Anchor::TopLeft,
                    },
                    action_buttons: Vec::new(),
                }
            }
            Orientation::LandscapeLeft | Orientation::LandscapeRight => {
                HUDLayout {
                    minimap_position: UIPosition {
                        x: -120.0 * scale,
                        y: 20.0 * scale,
                        width: 100.0 * scale,
                        height: 100.0 * scale,
                        anchor: Anchor::TopRight,
                    },
                    health_bar_position: UIPosition {
                        x: 20.0 * scale,
                        y: 20.0 * scale,
                        width: 200.0 * scale,
                        height: 24.0 * scale,
                        anchor: Anchor::TopLeft,
                    },
                    mana_bar_position: UIPosition {
                        x: 20.0 * scale,
                        y: 50.0 * scale,
                        width: 200.0 * scale,
                        height: 20.0 * scale,
                        anchor: Anchor::TopLeft,
                    },
                    chat_position: UIPosition {
                        x: 20.0 * scale,
                        y: -150.0 * scale,
                        width: 250.0 * scale,
                        height: 120.0 * scale,
                        anchor: Anchor::BottomLeft,
                    },
                    inventory_button_position: UIPosition {
                        x: -70.0 * scale,
                        y: -70.0 * scale,
                        width: 50.0 * scale,
                        height: 50.0 * scale,
                        anchor: Anchor::BottomRight,
                    },
                    skill_bar_position: UIPosition {
                        x: -50.0 * scale,
                        y: 0.0,
                        width: 60.0 * scale,
                        height: 280.0 * scale,
                        anchor: Anchor::CenterRight,
                    },
                    menu_button_position: UIPosition {
                        x: 20.0 * scale,
                        y: 20.0 * scale,
                        width: 40.0 * scale,
                        height: 40.0 * scale,
                        anchor: Anchor::TopLeft,
                    },
                    action_buttons: Vec::new(),
                }
            }
        }
    }

    /// Create default touch UI elements
    fn create_default_touch_elements(_orientation: &Orientation, scale: f32) -> Vec<TouchUIElement> {
        let mut elements = Vec::new();

        // Virtual joystick for movement
        elements.push(TouchUIElement {
            id: Uuid::new_v4(),
            element_type: TouchElementType::VirtualJoystick {
                sensitivity: 1.0,
                dead_zone: 0.1,
                return_to_center: true,
            },
            position: UIPosition {
                x: 80.0 * scale,
                y: -80.0 * scale,
                width: 120.0 * scale,
                height: 120.0 * scale,
                anchor: Anchor::BottomLeft,
            },
            visible: true,
            enabled: true,
            opacity: 0.7,
            haptic_feedback: true,
        });

        // Attack button
        elements.push(TouchUIElement {
            id: Uuid::new_v4(),
            element_type: TouchElementType::TouchButton {
                label: "Attack".to_string(),
                icon: Some("⚔️".to_string()),
                cooldown: None,
            },
            position: UIPosition {
                x: -120.0 * scale,
                y: -120.0 * scale,
                width: 70.0 * scale,
                height: 70.0 * scale,
                anchor: Anchor::BottomRight,
            },
            visible: true,
            enabled: true,
            opacity: 0.8,
            haptic_feedback: true,
        });

        // Interact button
        elements.push(TouchUIElement {
            id: Uuid::new_v4(),
            element_type: TouchElementType::TouchButton {
                label: "Interact".to_string(),
                icon: Some("👆".to_string()),
                cooldown: None,
            },
            position: UIPosition {
                x: -200.0 * scale,
                y: -80.0 * scale,
                width: 60.0 * scale,
                height: 60.0 * scale,
                anchor: Anchor::BottomRight,
            },
            visible: true,
            enabled: true,
            opacity: 0.8,
            haptic_feedback: true,
        });

        // Jump button
        elements.push(TouchUIElement {
            id: Uuid::new_v4(),
            element_type: TouchElementType::TouchButton {
                label: "Jump".to_string(),
                icon: Some("⬆️".to_string()),
                cooldown: None,
            },
            position: UIPosition {
                x: -60.0 * scale,
                y: -200.0 * scale,
                width: 60.0 * scale,
                height: 60.0 * scale,
                anchor: Anchor::BottomRight,
            },
            visible: true,
            enabled: true,
            opacity: 0.8,
            haptic_feedback: true,
        });

        elements
    }

    /// Update orientation when device is rotated
    pub async fn update_orientation(&mut self, new_orientation: Orientation) -> Result<()> {
        if self.orientation != new_orientation {
            info!("📱 Orientation changed: {:?} -> {:?}", self.orientation, new_orientation);
            
            self.orientation = new_orientation;
            self.hud_layout = Self::create_default_hud_layout(&self.orientation, &self.safe_area, self.ui_scale);
            self.touch_ui_elements = Self::create_default_touch_elements(&self.orientation, self.ui_scale);
        }
        
        Ok(())
    }

    /// Show/hide virtual keyboard
    pub fn set_virtual_keyboard(&mut self, visible: bool, keyboard_type: KeyboardType) {
        self.virtual_keyboard.visible = visible;
        self.virtual_keyboard.keyboard_type = keyboard_type.clone();
        
        if visible {
            // Adjust UI layout for keyboard
            self.virtual_keyboard.height = match self.orientation {
                Orientation::Portrait | Orientation::PortraitUpsideDown => 300.0 * self.ui_scale,
                _ => 250.0 * self.ui_scale,
            };
        } else {
            self.virtual_keyboard.height = 0.0;
        }
        
        info!("⌨️ Virtual keyboard: {}, type: {:?}", if visible { "shown" } else { "hidden" }, keyboard_type);
    }

    /// Get UI element at screen position
    pub fn get_ui_element_at_position(&self, x: f32, y: f32) -> Option<&TouchUIElement> {
        for element in &self.touch_ui_elements {
            if element.visible && element.enabled {
                let pos = &element.position;
                // Simple bounds check (would need proper anchor calculation)
                if x >= pos.x && x <= pos.x + pos.width && y >= pos.y && y <= pos.y + pos.height {
                    return Some(element);
                }
            }
        }
        None
    }

    /// Update UI scale based on accessibility settings
    pub fn set_ui_scale(&mut self, scale: f32) {
        self.ui_scale = scale.clamp(0.5, 3.0);
        self.hud_layout = Self::create_default_hud_layout(&self.orientation, &self.safe_area, self.ui_scale);
        self.touch_ui_elements = Self::create_default_touch_elements(&self.orientation, self.ui_scale);
        
        info!("📏 UI scale updated to: {:.2}", self.ui_scale);
    }

    /// Enable/disable touch UI elements
    pub fn set_touch_ui_enabled(&mut self, enabled: bool) {
        for element in &mut self.touch_ui_elements {
            element.enabled = enabled;
        }
        
        info!("👆 Touch UI: {}", if enabled { "enabled" } else { "disabled" });
    }

    /// Get recommended UI settings for current device
    pub fn get_recommended_ui_settings(&self) -> UISettings {
        match &self.platform_info.platform_type {
            PlatformType::Mobile { .. } => {
                if self.platform_info.is_tablet {
                    UISettings::tablet_default()
                } else {
                    UISettings::mobile_default()
                }
            }
            PlatformType::Web { .. } => {
                UISettings::web_default()
            }
            _ => UISettings::desktop_default(),
        }
    }
}

impl Default for SafeArea {
    fn default() -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }
}

/// UI settings for different platforms
#[derive(Debug, Clone)]
pub struct UISettings {
    pub show_tooltips: bool,
    pub enable_animations: bool,
    pub haptic_feedback: bool,
    pub auto_hide_ui: bool,
    pub ui_opacity: f32,
    pub font_size: f32,
    pub button_size: f32,
}

impl UISettings {
    pub fn mobile_default() -> Self {
        Self {
            show_tooltips: false, // Too cluttered on small screens
            enable_animations: true,
            haptic_feedback: true,
            auto_hide_ui: true,
            ui_opacity: 0.8,
            font_size: 1.2,
            button_size: 1.3,
        }
    }

    pub fn tablet_default() -> Self {
        Self {
            show_tooltips: true,
            enable_animations: true,
            haptic_feedback: true,
            auto_hide_ui: false,
            ui_opacity: 0.9,
            font_size: 1.1,
            button_size: 1.1,
        }
    }

    pub fn web_default() -> Self {
        Self {
            show_tooltips: true,
            enable_animations: true,
            haptic_feedback: false,
            auto_hide_ui: false,
            ui_opacity: 1.0,
            font_size: 1.0,
            button_size: 1.0,
        }
    }

    pub fn desktop_default() -> Self {
        Self {
            show_tooltips: true,
            enable_animations: true,
            haptic_feedback: false,
            auto_hide_ui: false,
            ui_opacity: 1.0,
            font_size: 1.0,
            button_size: 1.0,
        }
    }
}