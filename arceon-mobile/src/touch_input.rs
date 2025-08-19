use anyhow::Result;
use crate::{PlatformInfo, PlatformType};
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tracing::{info, debug};
use std::time::{Instant, Duration};
use glam::Vec2;

/// Touch input handler for mobile platforms
pub struct TouchInputHandler {
    pub platform_info: PlatformInfo,
    pub active_touches: HashMap<u32, Touch>,
    pub gesture_recognizer: GestureRecognizer,
    pub virtual_joystick: Option<VirtualJoystick>,
    pub haptic_feedback: HapticFeedback,
    pub touch_sensitivity: f32,
    pub multi_touch_enabled: bool,
}

/// Individual touch point
#[derive(Debug, Clone)]
pub struct Touch {
    pub id: u32,
    pub position: Vec2,
    pub previous_position: Vec2,
    pub start_position: Vec2,
    pub pressure: f32,
    pub radius: Vec2,
    pub start_time: Instant,
    pub phase: TouchPhase,
}

/// Touch phase states
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TouchPhase {
    Began,
    Moved,
    Stationary,
    Ended,
    Cancelled,
}

/// Gesture recognition system
#[derive(Debug)]
pub struct GestureRecognizer {
    pub tap_recognizer: TapRecognizer,
    pub swipe_recognizer: SwipeRecognizer,
    pub pinch_recognizer: PinchRecognizer,
    pub rotation_recognizer: RotationRecognizer,
    pub long_press_recognizer: LongPressRecognizer,
}

/// Tap gesture recognition
#[derive(Debug)]
pub struct TapRecognizer {
    pub max_tap_duration: Duration,
    pub max_tap_distance: f32,
    pub double_tap_interval: Duration,
    pub last_tap_time: Option<Instant>,
    pub last_tap_position: Option<Vec2>,
}

/// Swipe gesture recognition
#[derive(Debug)]
pub struct SwipeRecognizer {
    pub min_swipe_distance: f32,
    pub max_swipe_duration: Duration,
    pub direction_tolerance: f32,
}

/// Pinch/zoom gesture recognition
#[derive(Debug)]
pub struct PinchRecognizer {
    pub min_pinch_distance: f32,
    pub initial_distance: Option<f32>,
    pub current_scale: f32,
}

/// Rotation gesture recognition
#[derive(Debug)]
pub struct RotationRecognizer {
    pub min_rotation_angle: f32,
    pub initial_angle: Option<f32>,
    pub current_rotation: f32,
}

/// Long press gesture recognition
#[derive(Debug)]
pub struct LongPressRecognizer {
    pub min_press_duration: Duration,
    pub max_movement_radius: f32,
    pub active_long_press: Option<(Instant, Vec2)>,
}

/// Virtual joystick for movement input
#[derive(Debug, Clone)]
pub struct VirtualJoystick {
    pub center_position: Vec2,
    pub current_position: Vec2,
    pub max_radius: f32,
    pub dead_zone: f32,
    pub return_to_center: bool,
    pub active_touch_id: Option<u32>,
}

/// Haptic feedback system
#[derive(Debug)]
pub struct HapticFeedback {
    pub enabled: bool,
    pub light_feedback: bool,
    pub medium_feedback: bool,
    pub heavy_feedback: bool,
}

/// Touch input events
#[derive(Debug, Clone)]
pub enum TouchEvent {
    Touch {
        touch: Touch,
    },
    Gesture {
        gesture: RecognizedGesture,
    },
    VirtualJoystick {
        direction: Vec2,
        magnitude: f32,
    },
}

/// Recognized gestures
#[derive(Debug, Clone)]
pub enum RecognizedGesture {
    Tap {
        position: Vec2,
        tap_count: u32,
    },
    Swipe {
        start_position: Vec2,
        end_position: Vec2,
        direction: SwipeDirection,
        velocity: f32,
    },
    Pinch {
        center: Vec2,
        scale: f32,
        velocity: f32,
    },
    Rotation {
        center: Vec2,
        angle: f32,
        velocity: f32,
    },
    LongPress {
        position: Vec2,
        duration: Duration,
    },
}

/// Swipe directions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl TouchInputHandler {
    /// Create new touch input handler
    pub async fn new(platform_info: &PlatformInfo) -> Result<Self> {
        info!("👆 Initializing Touch Input Handler");

        let multi_touch_enabled = matches!(platform_info.platform_type, PlatformType::Mobile { .. });
        let touch_sensitivity = if platform_info.is_tablet { 0.8 } else { 1.0 };

        let haptic_feedback = HapticFeedback {
            enabled: platform_info.supports_haptics,
            light_feedback: true,
            medium_feedback: true,
            heavy_feedback: true,
        };

        let gesture_recognizer = GestureRecognizer {
            tap_recognizer: TapRecognizer {
                max_tap_duration: Duration::from_millis(300),
                max_tap_distance: 20.0,
                double_tap_interval: Duration::from_millis(500),
                last_tap_time: None,
                last_tap_position: None,
            },
            swipe_recognizer: SwipeRecognizer {
                min_swipe_distance: 50.0,
                max_swipe_duration: Duration::from_millis(800),
                direction_tolerance: 30.0, // degrees
            },
            pinch_recognizer: PinchRecognizer {
                min_pinch_distance: 10.0,
                initial_distance: None,
                current_scale: 1.0,
            },
            rotation_recognizer: RotationRecognizer {
                min_rotation_angle: 5.0, // degrees
                initial_angle: None,
                current_rotation: 0.0,
            },
            long_press_recognizer: LongPressRecognizer {
                min_press_duration: Duration::from_millis(800),
                max_movement_radius: 15.0,
                active_long_press: None,
            },
        };

        // Create virtual joystick for mobile platforms
        let virtual_joystick = if matches!(platform_info.platform_type, PlatformType::Mobile { .. }) {
            Some(VirtualJoystick {
                center_position: Vec2::new(100.0, 100.0), // Will be positioned by UI
                current_position: Vec2::new(100.0, 100.0),
                max_radius: 60.0,
                dead_zone: 0.15,
                return_to_center: true,
                active_touch_id: None,
            })
        } else {
            None
        };

        info!("✅ Touch input handler initialized");
        info!("   Multi-touch: {}", multi_touch_enabled);
        info!("   Haptic feedback: {}", haptic_feedback.enabled);
        info!("   Virtual joystick: {}", virtual_joystick.is_some());

        Ok(Self {
            platform_info: platform_info.clone(),
            active_touches: HashMap::new(),
            gesture_recognizer,
            virtual_joystick,
            haptic_feedback,
            touch_sensitivity,
            multi_touch_enabled,
        })
    }

    /// Process touch input event
    pub async fn process_touch_input(&mut self, touch_id: u32, position: Vec2, pressure: f32, phase: TouchPhase) -> Result<Vec<TouchEvent>> {
        let mut events = Vec::new();
        
        // Update or create touch
        let touch = self.update_touch(touch_id, position, pressure, phase);
        
        // Add raw touch event
        events.push(TouchEvent::Touch { touch: touch.clone() });

        // Process virtual joystick input
        if let Some(joystick_events) = self.process_virtual_joystick(&touch).await? {
            events.extend(joystick_events);
        }

        // Process gesture recognition
        if let Some(gesture_events) = self.process_gestures(&touch).await? {
            events.extend(gesture_events);
        }

        // Trigger haptic feedback
        self.trigger_haptic_feedback(&touch, &events).await?;

        // Clean up ended touches
        if matches!(phase, TouchPhase::Ended | TouchPhase::Cancelled) {
            self.active_touches.remove(&touch_id);
        }

        debug!("👆 Processed touch {}: {:?} -> {} events", touch_id, phase, events.len());
        Ok(events)
    }

    /// Update touch state
    fn update_touch(&mut self, touch_id: u32, position: Vec2, pressure: f32, phase: TouchPhase) -> Touch {
        let touch = if let Some(existing_touch) = self.active_touches.get(&touch_id) {
            Touch {
                id: touch_id,
                position,
                previous_position: existing_touch.position,
                start_position: existing_touch.start_position,
                pressure,
                radius: Vec2::splat(pressure * 10.0), // Estimate radius from pressure
                start_time: existing_touch.start_time,
                phase,
            }
        } else {
            Touch {
                id: touch_id,
                position,
                previous_position: position,
                start_position: position,
                pressure,
                radius: Vec2::splat(pressure * 10.0),
                start_time: Instant::now(),
                phase,
            }
        };

        self.active_touches.insert(touch_id, touch.clone());
        touch
    }

    /// Process virtual joystick input
    async fn process_virtual_joystick(&mut self, touch: &Touch) -> Result<Option<Vec<TouchEvent>>> {
        if let Some(joystick) = &mut self.virtual_joystick {
            // Check if touch is within joystick area
            let distance_from_center = (touch.position - joystick.center_position).length();
            
            match touch.phase {
                TouchPhase::Began => {
                    if distance_from_center <= joystick.max_radius {
                        joystick.active_touch_id = Some(touch.id);
                        joystick.current_position = touch.position;
                        return Ok(Some(vec![self.create_joystick_event(joystick)]));
                    }
                }
                TouchPhase::Moved => {
                    if joystick.active_touch_id == Some(touch.id) {
                        // Constrain to max radius
                        let direction = touch.position - joystick.center_position;
                        let distance = direction.length();
                        
                        if distance <= joystick.max_radius {
                            joystick.current_position = touch.position;
                        } else {
                            joystick.current_position = joystick.center_position + direction.normalize() * joystick.max_radius;
                        }
                        
                        return Ok(Some(vec![self.create_joystick_event(joystick)]));
                    }
                }
                TouchPhase::Ended | TouchPhase::Cancelled => {
                    if joystick.active_touch_id == Some(touch.id) {
                        joystick.active_touch_id = None;
                        
                        if joystick.return_to_center {
                            joystick.current_position = joystick.center_position;
                        }
                        
                        return Ok(Some(vec![self.create_joystick_event(joystick)]));
                    }
                }
                _ => {}
            }
        }

        Ok(None)
    }

    /// Create virtual joystick event
    fn create_joystick_event(&self, joystick: &VirtualJoystick) -> TouchEvent {
        let direction = joystick.current_position - joystick.center_position;
        let magnitude = (direction.length() / joystick.max_radius).clamp(0.0, 1.0);
        
        // Apply dead zone
        let adjusted_magnitude = if magnitude < joystick.dead_zone {
            0.0
        } else {
            (magnitude - joystick.dead_zone) / (1.0 - joystick.dead_zone)
        };

        let normalized_direction = if magnitude > 0.0 {
            direction.normalize()
        } else {
            Vec2::ZERO
        };

        TouchEvent::VirtualJoystick {
            direction: normalized_direction,
            magnitude: adjusted_magnitude,
        }
    }

    /// Process gesture recognition
    async fn process_gestures(&mut self, touch: &Touch) -> Result<Option<Vec<TouchEvent>>> {
        let mut events = Vec::new();

        // Single touch gestures
        if let Some(gesture) = self.recognize_tap_gesture(touch).await? {
            events.push(TouchEvent::Gesture { gesture });
        }

        if let Some(gesture) = self.recognize_swipe_gesture(touch).await? {
            events.push(TouchEvent::Gesture { gesture });
        }

        if let Some(gesture) = self.recognize_long_press_gesture(touch).await? {
            events.push(TouchEvent::Gesture { gesture });
        }

        // Multi-touch gestures
        if self.multi_touch_enabled && self.active_touches.len() >= 2 {
            if let Some(gesture) = self.recognize_pinch_gesture().await? {
                events.push(TouchEvent::Gesture { gesture });
            }

            if let Some(gesture) = self.recognize_rotation_gesture().await? {
                events.push(TouchEvent::Gesture { gesture });
            }
        }

        if events.is_empty() {
            Ok(None)
        } else {
            Ok(Some(events))
        }
    }

    /// Recognize tap gestures
    async fn recognize_tap_gesture(&mut self, touch: &Touch) -> Result<Option<RecognizedGesture>> {
        match touch.phase {
            TouchPhase::Ended => {
                let duration = touch.start_time.elapsed();
                let distance = (touch.position - touch.start_position).length();
                let recognizer = &mut self.gesture_recognizer.tap_recognizer;

                if duration <= recognizer.max_tap_duration && distance <= recognizer.max_tap_distance {
                    let mut tap_count = 1;

                    // Check for double tap
                    if let (Some(last_time), Some(last_pos)) = (recognizer.last_tap_time, recognizer.last_tap_position) {
                        let time_diff = touch.start_time.duration_since(last_time);
                        let pos_diff = (touch.position - last_pos).length();

                        if time_diff <= recognizer.double_tap_interval && pos_diff <= recognizer.max_tap_distance {
                            tap_count = 2;
                        }
                    }

                    recognizer.last_tap_time = Some(Instant::now());
                    recognizer.last_tap_position = Some(touch.position);

                    return Ok(Some(RecognizedGesture::Tap {
                        position: touch.position,
                        tap_count,
                    }));
                }
            }
            _ => {}
        }

        Ok(None)
    }

    /// Recognize swipe gestures
    async fn recognize_swipe_gesture(&mut self, touch: &Touch) -> Result<Option<RecognizedGesture>> {
        match touch.phase {
            TouchPhase::Ended => {
                let duration = touch.start_time.elapsed();
                let distance = (touch.position - touch.start_position).length();
                let recognizer = &self.gesture_recognizer.swipe_recognizer;

                if duration <= recognizer.max_swipe_duration && distance >= recognizer.min_swipe_distance {
                    let direction_vector = touch.position - touch.start_position;
                    let direction = Self::get_swipe_direction(direction_vector, recognizer.direction_tolerance);
                    let velocity = distance / duration.as_secs_f32();

                    return Ok(Some(RecognizedGesture::Swipe {
                        start_position: touch.start_position,
                        end_position: touch.position,
                        direction,
                        velocity,
                    }));
                }
            }
            _ => {}
        }

        Ok(None)
    }

    /// Recognize long press gestures
    async fn recognize_long_press_gesture(&mut self, touch: &Touch) -> Result<Option<RecognizedGesture>> {
        let recognizer = &mut self.gesture_recognizer.long_press_recognizer;

        match touch.phase {
            TouchPhase::Began => {
                recognizer.active_long_press = Some((touch.start_time, touch.start_position));
            }
            TouchPhase::Moved => {
                if let Some((start_time, start_pos)) = recognizer.active_long_press {
                    let movement = (touch.position - start_pos).length();
                    
                    if movement > recognizer.max_movement_radius {
                        recognizer.active_long_press = None;
                    } else {
                        let duration = start_time.elapsed();
                        if duration >= recognizer.min_press_duration {
                            recognizer.active_long_press = None;
                            return Ok(Some(RecognizedGesture::LongPress {
                                position: touch.position,
                                duration,
                            }));
                        }
                    }
                }
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                recognizer.active_long_press = None;
            }
            _ => {}
        }

        Ok(None)
    }

    /// Recognize pinch gestures
    async fn recognize_pinch_gesture(&mut self) -> Result<Option<RecognizedGesture>> {
        if self.active_touches.len() != 2 {
            return Ok(None);
        }

        let touches: Vec<&Touch> = self.active_touches.values().collect();
        let current_distance = (touches[0].position - touches[1].position).length();
        let center = (touches[0].position + touches[1].position) * 0.5;

        let recognizer = &mut self.gesture_recognizer.pinch_recognizer;

        if let Some(initial_distance) = recognizer.initial_distance {
            if current_distance > recognizer.min_pinch_distance {
                let scale = current_distance / initial_distance;
                let velocity = (scale - recognizer.current_scale) * 60.0; // Approximate velocity
                
                recognizer.current_scale = scale;

                return Ok(Some(RecognizedGesture::Pinch {
                    center,
                    scale,
                    velocity,
                }));
            }
        } else if current_distance > recognizer.min_pinch_distance {
            recognizer.initial_distance = Some(current_distance);
            recognizer.current_scale = 1.0;
        }

        Ok(None)
    }

    /// Recognize rotation gestures
    async fn recognize_rotation_gesture(&mut self) -> Result<Option<RecognizedGesture>> {
        if self.active_touches.len() != 2 {
            return Ok(None);
        }

        let touches: Vec<&Touch> = self.active_touches.values().collect();
        let direction_vector = touches[1].position - touches[0].position;
        let current_angle = direction_vector.y.atan2(direction_vector.x);
        let center = (touches[0].position + touches[1].position) * 0.5;

        let recognizer = &mut self.gesture_recognizer.rotation_recognizer;

        if let Some(initial_angle) = recognizer.initial_angle {
            let angle_diff = current_angle - initial_angle;
            let angle_diff_degrees = angle_diff.to_degrees();

            if angle_diff_degrees.abs() > recognizer.min_rotation_angle {
                let velocity = (angle_diff - recognizer.current_rotation) * 60.0; // Approximate velocity
                recognizer.current_rotation = angle_diff;

                return Ok(Some(RecognizedGesture::Rotation {
                    center,
                    angle: angle_diff,
                    velocity,
                }));
            }
        } else {
            recognizer.initial_angle = Some(current_angle);
            recognizer.current_rotation = 0.0;
        }

        Ok(None)
    }

    /// Get swipe direction from direction vector
    fn get_swipe_direction(direction: Vec2, tolerance: f32) -> SwipeDirection {
        let angle = direction.y.atan2(direction.x).to_degrees();
        let angle = if angle < 0.0 { angle + 360.0 } else { angle };

        // Define direction ranges with tolerance
        if angle >= 360.0 - tolerance || angle < tolerance {
            SwipeDirection::Right
        } else if angle >= 45.0 - tolerance && angle < 45.0 + tolerance {
            SwipeDirection::UpRight
        } else if angle >= 90.0 - tolerance && angle < 90.0 + tolerance {
            SwipeDirection::Up
        } else if angle >= 135.0 - tolerance && angle < 135.0 + tolerance {
            SwipeDirection::UpLeft
        } else if angle >= 180.0 - tolerance && angle < 180.0 + tolerance {
            SwipeDirection::Left
        } else if angle >= 225.0 - tolerance && angle < 225.0 + tolerance {
            SwipeDirection::DownLeft
        } else if angle >= 270.0 - tolerance && angle < 270.0 + tolerance {
            SwipeDirection::Down
        } else if angle >= 315.0 - tolerance && angle < 315.0 + tolerance {
            SwipeDirection::DownRight
        } else if angle < 45.0 {
            SwipeDirection::Right
        } else if angle < 135.0 {
            SwipeDirection::Up
        } else if angle < 225.0 {
            SwipeDirection::Left
        } else {
            SwipeDirection::Down
        }
    }

    /// Trigger appropriate haptic feedback
    async fn trigger_haptic_feedback(&self, touch: &Touch, events: &[TouchEvent]) -> Result<()> {
        if !self.haptic_feedback.enabled {
            return Ok(());
        }

        for event in events {
            match event {
                TouchEvent::Gesture { gesture } => {
                    match gesture {
                        RecognizedGesture::Tap { tap_count, .. } => {
                            if *tap_count == 1 && self.haptic_feedback.light_feedback {
                                self.trigger_light_haptic().await?;
                            } else if *tap_count >= 2 && self.haptic_feedback.medium_feedback {
                                self.trigger_medium_haptic().await?;
                            }
                        }
                        RecognizedGesture::LongPress { .. } => {
                            if self.haptic_feedback.heavy_feedback {
                                self.trigger_heavy_haptic().await?;
                            }
                        }
                        _ => {
                            if self.haptic_feedback.light_feedback {
                                self.trigger_light_haptic().await?;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Platform-specific haptic feedback implementations
    async fn trigger_light_haptic(&self) -> Result<()> {
        // Platform-specific implementation would go here
        debug!("🔮 Light haptic feedback");
        Ok(())
    }

    async fn trigger_medium_haptic(&self) -> Result<()> {
        debug!("🔮 Medium haptic feedback");
        Ok(())
    }

    async fn trigger_heavy_haptic(&self) -> Result<()> {
        debug!("🔮 Heavy haptic feedback");
        Ok(())
    }

    /// Set virtual joystick position
    pub fn set_virtual_joystick_position(&mut self, center: Vec2, max_radius: f32) {
        if let Some(joystick) = &mut self.virtual_joystick {
            joystick.center_position = center;
            joystick.max_radius = max_radius;
            
            if joystick.return_to_center && joystick.active_touch_id.is_none() {
                joystick.current_position = center;
            }
        }
    }

    /// Configure haptic feedback settings
    pub fn configure_haptic_feedback(&mut self, light: bool, medium: bool, heavy: bool) {
        self.haptic_feedback.light_feedback = light;
        self.haptic_feedback.medium_feedback = medium;
        self.haptic_feedback.heavy_feedback = heavy;
        
        info!("🔮 Haptic feedback configured: light={}, medium={}, heavy={}", light, medium, heavy);
    }

    /// Set touch sensitivity
    pub fn set_touch_sensitivity(&mut self, sensitivity: f32) {
        self.touch_sensitivity = sensitivity.clamp(0.1, 3.0);
        info!("👆 Touch sensitivity set to: {:.2}", self.touch_sensitivity);
    }
}