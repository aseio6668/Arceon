use anyhow::Result;
use crate::{PlatformInfo, PlatformType, UISettings};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tracing::{info, debug};
use std::collections::HashMap;

/// Adaptive UI manager that responds to different screen sizes and capabilities
pub struct AdaptiveUIManager {
    pub platform_info: PlatformInfo,
    pub current_layout: UILayout,
    pub screen_config: ScreenConfiguration,
    pub accessibility_settings: AccessibilitySettings,
    pub responsive_breakpoints: ResponsiveBreakpoints,
    pub ui_themes: HashMap<String, UITheme>,
    pub current_theme: String,
}

/// UI layout configuration
#[derive(Debug, Clone)]
pub struct UILayout {
    pub layout_type: LayoutType,
    pub grid_columns: u32,
    pub spacing: f32,
    pub margins: Margins,
    pub ui_elements: HashMap<String, UIElementLayout>,
}

/// Different layout types for various screen sizes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutType {
    Mobile,      // Single column, large touch targets
    Tablet,      // Two column, medium touch targets
    Desktop,     // Multi-column, small precise targets
    Television,  // Large fonts, simple navigation
    Watch,       // Minimal UI, gesture-based
}

/// Margins for UI layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// Individual UI element layout
#[derive(Debug, Clone)]
pub struct UIElementLayout {
    pub element_id: String,
    pub position: ElementPosition,
    pub size: ElementSize,
    pub visibility: ElementVisibility,
    pub priority: u32, // For layout priority when space is limited
}

/// Element positioning
#[derive(Debug, Clone)]
pub enum ElementPosition {
    Fixed { x: f32, y: f32 },
    Relative { x_percent: f32, y_percent: f32 },
    Flex { flex_grow: f32, flex_shrink: f32 },
    Grid { row: u32, column: u32, row_span: u32, col_span: u32 },
}

/// Element sizing
#[derive(Debug, Clone)]
pub enum ElementSize {
    Fixed { width: f32, height: f32 },
    Relative { width_percent: f32, height_percent: f32 },
    Content, // Size based on content
    Fill,    // Fill available space
}

/// Element visibility rules
#[derive(Debug, Clone)]
pub struct ElementVisibility {
    pub visible: bool,
    pub min_screen_width: Option<f32>,
    pub max_screen_width: Option<f32>,
    pub orientation_dependent: Option<OrientationVisibility>,
    pub platform_dependent: Option<Vec<PlatformType>>,
}

/// Orientation-specific visibility
#[derive(Debug, Clone)]
pub struct OrientationVisibility {
    pub portrait: bool,
    pub landscape: bool,
}

/// Screen configuration
#[derive(Debug, Clone)]
pub struct ScreenConfiguration {
    pub width: u32,
    pub height: u32,
    pub dpi: f32,
    pub scale_factor: f32,
    pub orientation: ScreenOrientation,
    pub safe_area: SafeAreaInsets,
}

/// Screen orientation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScreenOrientation {
    Portrait,
    Landscape,
    Square,
}

/// Safe area insets for modern devices
#[derive(Debug, Clone)]
pub struct SafeAreaInsets {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// Accessibility settings
#[derive(Debug, Clone)]
pub struct AccessibilitySettings {
    pub font_scale: f32,
    pub high_contrast: bool,
    pub reduce_motion: bool,
    pub voice_over_enabled: bool,
    pub button_shapes: bool,
    pub larger_touch_targets: bool,
}

/// Responsive breakpoints
#[derive(Debug, Clone)]
pub struct ResponsiveBreakpoints {
    pub mobile_max: f32,
    pub tablet_max: f32,
    pub desktop_max: f32,
    pub large_desktop_max: f32,
}

/// UI theme configuration
#[derive(Debug, Clone)]
pub struct UITheme {
    pub name: String,
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: SpacingScale,
    pub shadows: ShadowSettings,
    pub animations: AnimationSettings,
}

/// Color palette for themes
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub primary: [f32; 4],
    pub secondary: [f32; 4],
    pub accent: [f32; 4],
    pub background: [f32; 4],
    pub surface: [f32; 4],
    pub text_primary: [f32; 4],
    pub text_secondary: [f32; 4],
    pub error: [f32; 4],
    pub warning: [f32; 4],
    pub success: [f32; 4],
}

/// Typography settings
#[derive(Debug, Clone)]
pub struct Typography {
    pub font_family: String,
    pub base_font_size: f32,
    pub font_scale_ratio: f32,
    pub line_height_ratio: f32,
    pub font_weights: FontWeights,
}

/// Font weight values
#[derive(Debug, Clone)]
pub struct FontWeights {
    pub light: u16,
    pub regular: u16,
    pub medium: u16,
    pub bold: u16,
}

/// Spacing scale for consistent layouts
#[derive(Debug, Clone)]
pub struct SpacingScale {
    pub base_unit: f32,
    pub scale_ratio: f32,
}

/// Shadow settings for depth
#[derive(Debug, Clone)]
pub struct ShadowSettings {
    pub enabled: bool,
    pub blur_radius: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub color: [f32; 4],
}

/// Animation settings
#[derive(Debug, Clone)]
pub struct AnimationSettings {
    pub enabled: bool,
    pub duration_fast: f32,
    pub duration_medium: f32,
    pub duration_slow: f32,
    pub easing: AnimationEasing,
}

/// Animation easing curves
#[derive(Debug, Clone)]
pub enum AnimationEasing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Cubic { x1: f32, y1: f32, x2: f32, y2: f32 },
}

impl AdaptiveUIManager {
    /// Create new adaptive UI manager
    pub async fn new(platform_info: &PlatformInfo) -> Result<Self> {
        info!("🎨 Initializing Adaptive UI Manager");

        let screen_config = Self::create_screen_configuration(platform_info).await?;
        let layout_type = Self::determine_layout_type(&screen_config, platform_info);
        let current_layout = Self::create_layout_for_type(layout_type.clone(), &screen_config);
        let accessibility_settings = Self::detect_accessibility_settings(platform_info).await?;
        let responsive_breakpoints = Self::create_responsive_breakpoints();
        
        // Create default themes
        let mut ui_themes = HashMap::new();
        ui_themes.insert("light".to_string(), Self::create_light_theme());
        ui_themes.insert("dark".to_string(), Self::create_dark_theme());
        ui_themes.insert("high_contrast".to_string(), Self::create_high_contrast_theme());
        
        let current_theme = if accessibility_settings.high_contrast {
            "high_contrast".to_string()
        } else {
            "light".to_string()
        };

        info!("✅ Adaptive UI initialized");
        info!("   Layout: {:?}", layout_type);
        info!("   Screen: {}x{} @ {:.1} DPI", screen_config.width, screen_config.height, screen_config.dpi);
        info!("   Theme: {}", current_theme);

        Ok(Self {
            platform_info: platform_info.clone(),
            current_layout,
            screen_config,
            accessibility_settings,
            responsive_breakpoints,
            ui_themes,
            current_theme,
        })
    }

    /// Create screen configuration from platform info
    async fn create_screen_configuration(platform_info: &PlatformInfo) -> Result<ScreenConfiguration> {
        let (width, height) = platform_info.screen_size;
        let dpi = platform_info.screen_density * 96.0; // Convert to DPI
        let scale_factor = platform_info.screen_density;
        
        let orientation = if width > height {
            ScreenOrientation::Landscape
        } else if width == height {
            ScreenOrientation::Square
        } else {
            ScreenOrientation::Portrait
        };

        let safe_area = match &platform_info.platform_type {
            PlatformType::Mobile { os } => match os {
                crate::MobileOS::iOS | crate::MobileOS::iPadOS => {
                    SafeAreaInsets {
                        top: 44.0 * scale_factor,
                        bottom: 34.0 * scale_factor,
                        left: 0.0,
                        right: 0.0,
                    }
                }
                crate::MobileOS::Android => {
                    SafeAreaInsets {
                        top: 24.0 * scale_factor,
                        bottom: 48.0 * scale_factor,
                        left: 0.0,
                        right: 0.0,
                    }
                }
                _ => SafeAreaInsets::default(),
            },
            _ => SafeAreaInsets::default(),
        };

        Ok(ScreenConfiguration {
            width,
            height,
            dpi,
            scale_factor,
            orientation,
            safe_area,
        })
    }

    /// Determine appropriate layout type
    fn determine_layout_type(screen_config: &ScreenConfiguration, platform_info: &PlatformInfo) -> LayoutType {
        match &platform_info.platform_type {
            PlatformType::Mobile { .. } => {
                if platform_info.is_tablet {
                    LayoutType::Tablet
                } else {
                    LayoutType::Mobile
                }
            }
            PlatformType::Web { .. } => {
                let screen_width = screen_config.width as f32;
                if screen_width < 768.0 {
                    LayoutType::Mobile
                } else if screen_width < 1024.0 {
                    LayoutType::Tablet
                } else {
                    LayoutType::Desktop
                }
            }
            PlatformType::Desktop { .. } => {
                LayoutType::Desktop
            }
            PlatformType::Console { .. } => {
                LayoutType::Television
            }
        }
    }

    /// Create layout for specific type
    fn create_layout_for_type(layout_type: LayoutType, screen_config: &ScreenConfiguration) -> UILayout {
        let mut ui_elements = HashMap::new();

        match layout_type {
            LayoutType::Mobile => {
                let margins = Margins {
                    top: screen_config.safe_area.top + 16.0,
                    bottom: screen_config.safe_area.bottom + 16.0,
                    left: 16.0,
                    right: 16.0,
                };

                // Health bar - top left
                ui_elements.insert("health_bar".to_string(), UIElementLayout {
                    element_id: "health_bar".to_string(),
                    position: ElementPosition::Fixed { x: margins.left, y: margins.top },
                    size: ElementSize::Fixed { width: 200.0, height: 32.0 },
                    visibility: ElementVisibility::always_visible(),
                    priority: 100,
                });

                // Virtual joystick - bottom left
                ui_elements.insert("virtual_joystick".to_string(), UIElementLayout {
                    element_id: "virtual_joystick".to_string(),
                    position: ElementPosition::Fixed { 
                        x: margins.left + 80.0, 
                        y: screen_config.height as f32 - margins.bottom - 120.0 
                    },
                    size: ElementSize::Fixed { width: 120.0, height: 120.0 },
                    visibility: ElementVisibility::mobile_only(),
                    priority: 90,
                });

                // Action buttons - bottom right
                ui_elements.insert("action_buttons".to_string(), UIElementLayout {
                    element_id: "action_buttons".to_string(),
                    position: ElementPosition::Fixed { 
                        x: screen_config.width as f32 - margins.right - 80.0, 
                        y: screen_config.height as f32 - margins.bottom - 80.0 
                    },
                    size: ElementSize::Fixed { width: 80.0, height: 160.0 },
                    visibility: ElementVisibility::always_visible(),
                    priority: 85,
                });

                UILayout {
                    layout_type,
                    grid_columns: 1,
                    spacing: 16.0,
                    margins,
                    ui_elements,
                }
            }
            LayoutType::Tablet => {
                let margins = Margins {
                    top: screen_config.safe_area.top + 20.0,
                    bottom: screen_config.safe_area.bottom + 20.0,
                    left: 24.0,
                    right: 24.0,
                };

                // More spread out layout for tablets
                ui_elements.insert("health_bar".to_string(), UIElementLayout {
                    element_id: "health_bar".to_string(),
                    position: ElementPosition::Fixed { x: margins.left, y: margins.top },
                    size: ElementSize::Fixed { width: 250.0, height: 28.0 },
                    visibility: ElementVisibility::always_visible(),
                    priority: 100,
                });

                ui_elements.insert("minimap".to_string(), UIElementLayout {
                    element_id: "minimap".to_string(),
                    position: ElementPosition::Fixed { 
                        x: screen_config.width as f32 - margins.right - 150.0, 
                        y: margins.top 
                    },
                    size: ElementSize::Fixed { width: 150.0, height: 150.0 },
                    visibility: ElementVisibility::tablet_desktop(),
                    priority: 80,
                });

                UILayout {
                    layout_type,
                    grid_columns: 2,
                    spacing: 20.0,
                    margins,
                    ui_elements,
                }
            }
            LayoutType::Desktop => {
                let margins = Margins {
                    top: 20.0,
                    bottom: 20.0,
                    left: 20.0,
                    right: 20.0,
                };

                // Traditional MMO layout
                ui_elements.insert("health_bar".to_string(), UIElementLayout {
                    element_id: "health_bar".to_string(),
                    position: ElementPosition::Fixed { x: margins.left, y: margins.top },
                    size: ElementSize::Fixed { width: 300.0, height: 24.0 },
                    visibility: ElementVisibility::always_visible(),
                    priority: 100,
                });

                ui_elements.insert("minimap".to_string(), UIElementLayout {
                    element_id: "minimap".to_string(),
                    position: ElementPosition::Fixed { 
                        x: screen_config.width as f32 - margins.right - 200.0, 
                        y: margins.top 
                    },
                    size: ElementSize::Fixed { width: 200.0, height: 200.0 },
                    visibility: ElementVisibility::always_visible(),
                    priority: 70,
                });

                ui_elements.insert("chat_window".to_string(), UIElementLayout {
                    element_id: "chat_window".to_string(),
                    position: ElementPosition::Fixed { 
                        x: margins.left, 
                        y: screen_config.height as f32 - margins.bottom - 200.0 
                    },
                    size: ElementSize::Fixed { width: 400.0, height: 200.0 },
                    visibility: ElementVisibility::desktop_only(),
                    priority: 60,
                });

                UILayout {
                    layout_type,
                    grid_columns: 3,
                    spacing: 12.0,
                    margins,
                    ui_elements,
                }
            }
            LayoutType::Television => {
                // TV-friendly layout with large elements
                let margins = Margins {
                    top: 60.0,
                    bottom: 60.0,
                    left: 80.0,
                    right: 80.0,
                };

                UILayout {
                    layout_type,
                    grid_columns: 2,
                    spacing: 40.0,
                    margins,
                    ui_elements,
                }
            }
            LayoutType::Watch => {
                // Minimal layout for small screens
                let margins = Margins::all(8.0);

                UILayout {
                    layout_type,
                    grid_columns: 1,
                    spacing: 4.0,
                    margins,
                    ui_elements,
                }
            }
        }
    }

    /// Detect accessibility settings
    async fn detect_accessibility_settings(platform_info: &PlatformInfo) -> Result<AccessibilitySettings> {
        // Platform-specific accessibility detection would go here
        Ok(AccessibilitySettings {
            font_scale: 1.0,
            high_contrast: false,
            reduce_motion: false,
            voice_over_enabled: false,
            button_shapes: false,
            larger_touch_targets: matches!(platform_info.platform_type, PlatformType::Mobile { .. }),
        })
    }

    /// Create responsive breakpoints
    fn create_responsive_breakpoints() -> ResponsiveBreakpoints {
        ResponsiveBreakpoints {
            mobile_max: 767.0,
            tablet_max: 1023.0,
            desktop_max: 1439.0,
            large_desktop_max: 1920.0,
        }
    }

    /// Update layout when screen size changes
    pub async fn update_screen_size(&mut self, width: u32, height: u32) -> Result<()> {
        info!("🔄 Updating screen size: {}x{}", width, height);

        self.screen_config.width = width;
        self.screen_config.height = height;
        
        let new_orientation = if width > height {
            ScreenOrientation::Landscape
        } else if width == height {
            ScreenOrientation::Square
        } else {
            ScreenOrientation::Portrait
        };

        if new_orientation != self.screen_config.orientation {
            self.screen_config.orientation = new_orientation;
            info!("📱 Orientation changed to: {:?}", self.screen_config.orientation);
        }

        // Recalculate layout
        let new_layout_type = Self::determine_layout_type(&self.screen_config, &self.platform_info);
        if new_layout_type != self.current_layout.layout_type {
            info!("🎨 Layout type changed: {:?} -> {:?}", self.current_layout.layout_type, new_layout_type);
            self.current_layout = Self::create_layout_for_type(new_layout_type, &self.screen_config);
        }

        Ok(())
    }

    /// Switch UI theme
    pub fn set_theme(&mut self, theme_name: &str) -> Result<()> {
        if self.ui_themes.contains_key(theme_name) {
            self.current_theme = theme_name.to_string();
            info!("🎨 Switched to theme: {}", theme_name);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Theme '{}' not found", theme_name))
        }
    }

    /// Apply accessibility settings
    pub fn apply_accessibility_settings(&mut self, settings: AccessibilitySettings) {
        self.accessibility_settings = settings;

        // Adjust UI based on accessibility needs
        if self.accessibility_settings.larger_touch_targets {
            self.scale_touch_targets(1.3);
        }

        if self.accessibility_settings.high_contrast {
            self.current_theme = "high_contrast".to_string();
        }

        info!("♿ Applied accessibility settings");
    }

    /// Scale touch targets for accessibility
    fn scale_touch_targets(&mut self, scale_factor: f32) {
        for element in self.current_layout.ui_elements.values_mut() {
            if let ElementSize::Fixed { width, height } = &mut element.size {
                // Only scale interactive elements
                if element.element_id.contains("button") || element.element_id.contains("joystick") {
                    *width *= scale_factor;
                    *height *= scale_factor;
                }
            }
        }
    }

    /// Get current theme
    pub fn get_current_theme(&self) -> &UITheme {
        self.ui_themes.get(&self.current_theme)
            .unwrap_or(self.ui_themes.get("light").unwrap())
    }

    /// Create light theme
    fn create_light_theme() -> UITheme {
        UITheme {
            name: "Light".to_string(),
            colors: ColorPalette {
                primary: [0.2, 0.5, 1.0, 1.0],
                secondary: [0.6, 0.6, 0.6, 1.0],
                accent: [1.0, 0.3, 0.3, 1.0],
                background: [0.98, 0.98, 0.98, 1.0],
                surface: [1.0, 1.0, 1.0, 1.0],
                text_primary: [0.1, 0.1, 0.1, 1.0],
                text_secondary: [0.4, 0.4, 0.4, 1.0],
                error: [0.8, 0.1, 0.1, 1.0],
                warning: [1.0, 0.6, 0.0, 1.0],
                success: [0.1, 0.7, 0.1, 1.0],
            },
            typography: Typography::default(),
            spacing: SpacingScale { base_unit: 8.0, scale_ratio: 1.618 },
            shadows: ShadowSettings::default(),
            animations: AnimationSettings::default(),
        }
    }

    /// Create dark theme
    fn create_dark_theme() -> UITheme {
        UITheme {
            name: "Dark".to_string(),
            colors: ColorPalette {
                primary: [0.3, 0.6, 1.0, 1.0],
                secondary: [0.5, 0.5, 0.5, 1.0],
                accent: [1.0, 0.4, 0.4, 1.0],
                background: [0.1, 0.1, 0.1, 1.0],
                surface: [0.15, 0.15, 0.15, 1.0],
                text_primary: [0.9, 0.9, 0.9, 1.0],
                text_secondary: [0.6, 0.6, 0.6, 1.0],
                error: [1.0, 0.3, 0.3, 1.0],
                warning: [1.0, 0.7, 0.2, 1.0],
                success: [0.3, 0.8, 0.3, 1.0],
            },
            typography: Typography::default(),
            spacing: SpacingScale { base_unit: 8.0, scale_ratio: 1.618 },
            shadows: ShadowSettings::dark(),
            animations: AnimationSettings::default(),
        }
    }

    /// Create high contrast theme
    fn create_high_contrast_theme() -> UITheme {
        UITheme {
            name: "High Contrast".to_string(),
            colors: ColorPalette {
                primary: [0.0, 0.0, 1.0, 1.0],
                secondary: [0.5, 0.5, 0.5, 1.0],
                accent: [1.0, 0.0, 0.0, 1.0],
                background: [1.0, 1.0, 1.0, 1.0],
                surface: [0.95, 0.95, 0.95, 1.0],
                text_primary: [0.0, 0.0, 0.0, 1.0],
                text_secondary: [0.2, 0.2, 0.2, 1.0],
                error: [1.0, 0.0, 0.0, 1.0],
                warning: [0.8, 0.4, 0.0, 1.0],
                success: [0.0, 0.6, 0.0, 1.0],
            },
            typography: Typography::high_contrast(),
            spacing: SpacingScale { base_unit: 12.0, scale_ratio: 1.5 },
            shadows: ShadowSettings::disabled(),
            animations: AnimationSettings::reduced(),
        }
    }
}

// Helper implementations
impl ElementVisibility {
    pub fn always_visible() -> Self {
        Self {
            visible: true,
            min_screen_width: None,
            max_screen_width: None,
            orientation_dependent: None,
            platform_dependent: None,
        }
    }

    pub fn mobile_only() -> Self {
        Self {
            visible: true,
            min_screen_width: None,
            max_screen_width: Some(767.0),
            orientation_dependent: None,
            platform_dependent: Some(vec![
                PlatformType::Mobile { os: crate::MobileOS::Android },
                PlatformType::Mobile { os: crate::MobileOS::iOS },
            ]),
        }
    }

    pub fn tablet_desktop() -> Self {
        Self {
            visible: true,
            min_screen_width: Some(768.0),
            max_screen_width: None,
            orientation_dependent: None,
            platform_dependent: None,
        }
    }

    pub fn desktop_only() -> Self {
        Self {
            visible: true,
            min_screen_width: Some(1024.0),
            max_screen_width: None,
            orientation_dependent: None,
            platform_dependent: Some(vec![
                PlatformType::Desktop { os: crate::DesktopOS::Windows },
                PlatformType::Desktop { os: crate::DesktopOS::MacOS },
                PlatformType::Desktop { os: crate::DesktopOS::Linux },
            ]),
        }
    }
}

impl Default for SafeAreaInsets {
    fn default() -> Self {
        Self { top: 0.0, bottom: 0.0, left: 0.0, right: 0.0 }
    }
}

impl Margins {
    pub fn all(value: f32) -> Self {
        Self { top: value, bottom: value, left: value, right: value }
    }
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: "System".to_string(),
            base_font_size: 16.0,
            font_scale_ratio: 1.25,
            line_height_ratio: 1.5,
            font_weights: FontWeights {
                light: 300,
                regular: 400,
                medium: 500,
                bold: 700,
            },
        }
    }
}

impl Typography {
    pub fn high_contrast() -> Self {
        Self {
            font_family: "System".to_string(),
            base_font_size: 18.0,
            font_scale_ratio: 1.2,
            line_height_ratio: 1.6,
            font_weights: FontWeights {
                light: 400,
                regular: 600,
                medium: 700,
                bold: 900,
            },
        }
    }
}

impl Default for ShadowSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            blur_radius: 4.0,
            offset_x: 0.0,
            offset_y: 2.0,
            color: [0.0, 0.0, 0.0, 0.1],
        }
    }
}

impl ShadowSettings {
    pub fn dark() -> Self {
        Self {
            enabled: true,
            blur_radius: 6.0,
            offset_x: 0.0,
            offset_y: 3.0,
            color: [0.0, 0.0, 0.0, 0.3],
        }
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            blur_radius: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl Default for AnimationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_fast: 0.15,
            duration_medium: 0.3,
            duration_slow: 0.6,
            easing: AnimationEasing::EaseInOut,
        }
    }
}

impl AnimationSettings {
    pub fn reduced() -> Self {
        Self {
            enabled: false,
            duration_fast: 0.0,
            duration_medium: 0.0,
            duration_slow: 0.0,
            easing: AnimationEasing::Linear,
        }
    }
}