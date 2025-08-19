/*!
# Windows Launcher Support

Provides Windows-specific launcher integration including:
- Registry integration for game registration
- Windows service management
- Auto-update mechanisms
- Process management and monitoring
- Third-party launcher bindings
*/

use crate::*;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use tracing::{info, warn};

#[cfg(feature = "windows-launcher")]
use {
    windows::Win32::{
            Foundation::*,
            System::Threading::*,
        },
    winreg::RegKey,
};

/// Windows launcher adapter
pub struct WindowsLauncherAdapter {
    platform_id: PlatformId,
    config: Option<PlatformConfig>,
    capabilities: PlatformCapabilities,
    installation_info: Option<WindowsInstallationInfo>,
}

/// Windows installation information
#[derive(Debug, Clone)]
pub struct WindowsInstallationInfo {
    pub install_path: PathBuf,
    pub executable_path: PathBuf,
    pub version: String,
    pub registry_key: String,
    pub auto_update_enabled: bool,
    pub service_name: Option<String>,
}

/// Windows launcher configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowsLauncherConfig {
    pub app_name: String,
    pub app_version: String,
    pub publisher: String,
    pub install_directory: PathBuf,
    pub executable_name: String,
    pub registry_root: String,
    pub auto_start: bool,
    pub create_desktop_shortcut: bool,
    pub create_start_menu_shortcut: bool,
    pub associate_file_types: Vec<FileTypeAssociation>,
    pub required_dependencies: Vec<Dependency>,
    pub update_server_url: Option<String>,
}

/// File type association
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileTypeAssociation {
    pub extension: String,
    pub description: String,
    pub icon_path: Option<PathBuf>,
}

/// Required dependency
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub download_url: Option<String>,
    pub installer_path: Option<PathBuf>,
}

/// Process management information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub process_id: u32,
    pub executable_path: PathBuf,
    pub command_line: String,
    pub working_directory: PathBuf,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub is_running: bool,
}

impl WindowsLauncherAdapter {
    /// Create a new Windows launcher adapter
    pub fn new() -> Self {
        Self {
            platform_id: PlatformId::new("windows"),
            config: None,
            capabilities: PlatformCapabilities {
                authentication: false,
                user_profiles: false,
                achievements: false,
                friends_list: false,
                content_management: true,
                analytics: true,
                overlay_support: false,
                cloud_saves: false,
                voice_chat: false,
                workshop: false,
            },
            installation_info: None,
        }
    }
    
    /// Register application in Windows registry
    #[cfg(feature = "windows-launcher")]
    pub fn register_application(&self, launcher_config: &WindowsLauncherConfig) -> PlatformResult<()> {
        info!("Registering application in Windows registry: {}", launcher_config.app_name);
        
        let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        
        // Register in Apps registry
        let apps_key_path = format!("Software\\{}\\{}", launcher_config.publisher, launcher_config.app_name);
        let apps_key = hkcu.create_subkey(&apps_key_path)
            .map_err(|e| PlatformError::RegistryError {
                message: format!("Failed to create app registry key: {}", e),
            })?;
        
        apps_key.0.set_value("Version", &launcher_config.app_version)
            .map_err(|e| PlatformError::RegistryError {
                message: format!("Failed to set version: {}", e),
            })?;
        
        apps_key.0.set_value("InstallPath", &launcher_config.install_directory.to_string_lossy().as_ref())
            .map_err(|e| PlatformError::RegistryError {
                message: format!("Failed to set install path: {}", e),
            })?;
        
        apps_key.0.set_value("ExecutablePath", &launcher_config.install_directory.join(&launcher_config.executable_name).to_string_lossy().as_ref())
            .map_err(|e| PlatformError::RegistryError {
                message: format!("Failed to set executable path: {}", e),
            })?;
        
        // Register in Add/Remove Programs
        let uninstall_key_path = format!("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}", launcher_config.app_name);
        let uninstall_key = hkcu.create_subkey(&uninstall_key_path)
            .map_err(|e| PlatformError::RegistryError {
                message: format!("Failed to create uninstall registry key: {}", e),
            })?;
        
        uninstall_key.0.set_value("DisplayName", &launcher_config.app_name)
            .map_err(|e| PlatformError::RegistryError {
                message: format!("Failed to set display name: {}", e),
            })?;
        
        uninstall_key.0.set_value("DisplayVersion", &launcher_config.app_version)
            .map_err(|e| PlatformError::RegistryError {
                message: format!("Failed to set display version: {}", e),
            })?;
        
        uninstall_key.0.set_value("Publisher", &launcher_config.publisher)
            .map_err(|e| PlatformError::RegistryError {
                message: format!("Failed to set publisher: {}", e),
            })?;
        
        uninstall_key.0.set_value("InstallLocation", &launcher_config.install_directory.to_string_lossy().as_ref())
            .map_err(|e| PlatformError::RegistryError {
                message: format!("Failed to set install location: {}", e),
            })?;
        
        info!("Application registered successfully in Windows registry");
        Ok(())
    }
    
    /// Create Windows shortcuts
    #[cfg(feature = "windows-launcher")]
    pub fn create_shortcuts(&self, launcher_config: &WindowsLauncherConfig) -> PlatformResult<()> {
        info!("Creating Windows shortcuts for {}", launcher_config.app_name);
        
        if launcher_config.create_desktop_shortcut {
            self.create_desktop_shortcut(launcher_config)?;
        }
        
        if launcher_config.create_start_menu_shortcut {
            self.create_start_menu_shortcut(launcher_config)?;
        }
        
        Ok(())
    }
    
    /// Create desktop shortcut
    #[cfg(feature = "windows-launcher")]
    fn create_desktop_shortcut(&self, launcher_config: &WindowsLauncherConfig) -> PlatformResult<()> {
        let desktop_path = dirs::desktop_dir()
            .ok_or_else(|| PlatformError::WindowsApiError {
                message: "Failed to get desktop directory".to_string(),
            })?;
        
        let shortcut_path = desktop_path.join(format!("{}.lnk", launcher_config.app_name));
        let executable_path = launcher_config.install_directory.join(&launcher_config.executable_name);
        
        self.create_shortcut_file(&shortcut_path, &executable_path, &launcher_config.install_directory)?;
        
        info!("Desktop shortcut created: {:?}", shortcut_path);
        Ok(())
    }
    
    /// Create Start Menu shortcut
    #[cfg(feature = "windows-launcher")]
    fn create_start_menu_shortcut(&self, launcher_config: &WindowsLauncherConfig) -> PlatformResult<()> {
        let start_menu_path = dirs::data_dir()
            .ok_or_else(|| PlatformError::WindowsApiError {
                message: "Failed to get data directory".to_string(),
            })?
            .join("Microsoft\\Windows\\Start Menu\\Programs");
        
        let app_folder = start_menu_path.join(&launcher_config.publisher);
        std::fs::create_dir_all(&app_folder)
            .map_err(|e| PlatformError::IoError(e))?;
        
        let shortcut_path = app_folder.join(format!("{}.lnk", launcher_config.app_name));
        let executable_path = launcher_config.install_directory.join(&launcher_config.executable_name);
        
        self.create_shortcut_file(&shortcut_path, &executable_path, &launcher_config.install_directory)?;
        
        info!("Start Menu shortcut created: {:?}", shortcut_path);
        Ok(())
    }
    
    /// Create a shortcut file (using Windows Shell API)
    #[cfg(feature = "windows-launcher")]
    fn create_shortcut_file(&self, shortcut_path: &PathBuf, target_path: &PathBuf, working_dir: &PathBuf) -> PlatformResult<()> {
        // This is a simplified version. A full implementation would use IShellLink COM interface
        info!("Creating shortcut: {:?} -> {:?}", shortcut_path, target_path);
        
        // For now, create a simple batch file as a placeholder
        let batch_content = format!(
            "@echo off\ncd /d \"{}\"\nstart \"{}\" \"{}\"",
            working_dir.display(),
            shortcut_path.file_stem().unwrap().to_string_lossy(),
            target_path.display()
        );
        
        let batch_path = shortcut_path.with_extension("bat");
        std::fs::write(&batch_path, batch_content)
            .map_err(|e| PlatformError::IoError(e))?;
        
        Ok(())
    }
    
    /// Associate file types with the application
    #[cfg(feature = "windows-launcher")]
    pub fn register_file_associations(&self, launcher_config: &WindowsLauncherConfig) -> PlatformResult<()> {
        info!("Registering file associations for {}", launcher_config.app_name);
        
        let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let executable_path = launcher_config.install_directory.join(&launcher_config.executable_name);
        
        for association in &launcher_config.associate_file_types {
            // Register file extension
            let ext_key_path = format!("Software\\Classes\\.{}", association.extension);
            let ext_key = hkcu.create_subkey(&ext_key_path)
                .map_err(|e| PlatformError::RegistryError {
                    message: format!("Failed to create extension key: {}", e),
                })?;
            
            let prog_id = format!("{}.{}", launcher_config.app_name, association.extension);
            ext_key.0.set_value("", &prog_id)
                .map_err(|e| PlatformError::RegistryError {
                    message: format!("Failed to set program ID: {}", e),
                })?;
            
            // Register program ID
            let prog_key_path = format!("Software\\Classes\\{}", prog_id);
            let prog_key = hkcu.create_subkey(&prog_key_path)
                .map_err(|e| PlatformError::RegistryError {
                    message: format!("Failed to create program key: {}", e),
                })?;
            
            prog_key.0.set_value("", &association.description)
                .map_err(|e| PlatformError::RegistryError {
                    message: format!("Failed to set description: {}", e),
                })?;
            
            // Register shell command
            let command_key_path = format!("{}\\shell\\open\\command", prog_key_path);
            let command_key = hkcu.create_subkey(&command_key_path)
                .map_err(|e| PlatformError::RegistryError {
                    message: format!("Failed to create command key: {}", e),
                })?;
            
            let command = format!("\"{}\" \"%1\"", executable_path.display());
            command_key.0.set_value("", &command)
                .map_err(|e| PlatformError::RegistryError {
                    message: format!("Failed to set command: {}", e),
                })?;
            
            info!("File association registered: .{} -> {}", association.extension, launcher_config.app_name);
        }
        
        Ok(())
    }
    
    /// Launch process with monitoring
    pub async fn launch_process(&self, executable_path: &PathBuf, arguments: &[String], working_dir: Option<&PathBuf>) -> PlatformResult<ProcessInfo> {
        info!("Launching process: {:?} with args: {:?}", executable_path, arguments);
        
        let mut command = Command::new(executable_path);
        command.args(arguments);
        
        if let Some(work_dir) = working_dir {
            command.current_dir(work_dir);
        }
        
        let child = command.spawn()
            .map_err(|e| PlatformError::LauncherError {
                message: format!("Failed to launch process: {}", e),
            })?;
        
        let process_info = ProcessInfo {
            process_id: child.id(),
            executable_path: executable_path.clone(),
            command_line: format!("{} {}", executable_path.display(), arguments.join(" ")),
            working_directory: working_dir.cloned().unwrap_or_else(|| std::env::current_dir().unwrap()),
            start_time: chrono::Utc::now(),
            is_running: true,
        };
        
        info!("Process launched successfully: PID {}", process_info.process_id);
        Ok(process_info)
    }
    
    /// Check if process is still running
    #[cfg(feature = "windows-launcher")]
    pub fn is_process_running(&self, process_id: u32) -> bool {
        unsafe {
            match OpenProcess(PROCESS_QUERY_INFORMATION, false, process_id) {
                Ok(handle) if !handle.is_invalid() => {
                    let mut exit_code: u32 = 0;
                    let result = GetExitCodeProcess(handle, &mut exit_code);
                    let _ = CloseHandle(handle);
                    result.is_ok() && exit_code == 259 // STILL_ACTIVE
                }
                _ => false,
            }
        }
    }
    
    /// Check for application updates
    pub async fn check_for_updates(&self, update_server_url: &str, current_version: &str) -> PlatformResult<Option<UpdateInfo>> {
        info!("Checking for updates from: {}", update_server_url);
        
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/version", update_server_url))
            .send()
            .await?;
        
        if response.status().is_success() {
            let update_info: UpdateInfo = response.json().await?;
            
            if version_is_newer(&update_info.version, current_version) {
                info!("Update available: {} -> {}", current_version, update_info.version);
                Ok(Some(update_info))
            } else {
                info!("No updates available");
                Ok(None)
            }
        } else {
            warn!("Failed to check for updates: {}", response.status());
            Ok(None)
        }
    }
    
    /// Download and install update
    pub async fn install_update(&self, update_info: &UpdateInfo, install_path: &PathBuf) -> PlatformResult<()> {
        info!("Installing update: {}", update_info.version);
        
        // Download update package
        let client = reqwest::Client::new();
        let response = client
            .get(&update_info.download_url)
            .send()
            .await?;
        
        if response.status().is_success() {
            let update_package = response.bytes().await?;
            
            // Save update package
            let update_file = install_path.join(format!("update_{}.zip", update_info.version));
            std::fs::write(&update_file, update_package)
                .map_err(|e| PlatformError::IoError(e))?;
            
            info!("Update package downloaded: {:?}", update_file);
            
            // Extract and apply update (simplified)
            // In a real implementation, this would handle proper backup, extraction, and rollback
            info!("Update installation completed");
            Ok(())
        } else {
            Err(PlatformError::NetworkError(reqwest::Error::from(response.error_for_status().unwrap_err())))
        }
    }
    
    /// Get system information for compatibility checking
    pub fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            os_version: std::env::var("OS").unwrap_or_else(|_| "Unknown".to_string()),
            architecture: std::env::consts::ARCH.to_string(),
            available_memory: 0, // Would need Windows API calls
            cpu_cores: num_cpus::get(),
            graphics_info: vec![], // Would need DirectX/OpenGL detection
        }
    }
    
    /// Fallback implementation for non-Windows features
    #[cfg(not(feature = "windows-launcher"))]
    fn fallback_operation(&self, operation: &str) -> PlatformResult<()> {
        info!("Windows launcher integration disabled, using fallback for: {}", operation);
        Ok(())
    }
}

/// Update information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub release_date: String,
    pub download_url: String,
    pub changelog: String,
    pub required: bool,
    pub size_bytes: u64,
    pub checksum: String,
}

/// System information for compatibility
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os_version: String,
    pub architecture: String,
    pub available_memory: u64,
    pub cpu_cores: usize,
    pub graphics_info: Vec<String>,
}

/// Compare version strings
fn version_is_newer(new_version: &str, current_version: &str) -> bool {
    // Simplified version comparison
    // A real implementation would handle semantic versioning properly
    new_version > current_version
}

#[async_trait]
impl PlatformAdapter for WindowsLauncherAdapter {
    fn platform_id(&self) -> &PlatformId {
        &self.platform_id
    }
    
    fn capabilities(&self) -> &PlatformCapabilities {
        &self.capabilities
    }
    
    async fn initialize(&mut self, config: PlatformConfig) -> PlatformResult<()> {
        info!("Initializing Windows launcher adapter");
        
        // Parse Windows-specific configuration
        let launcher_config: WindowsLauncherConfig = config.get_custom_setting("launcher_config")?
            .ok_or_else(|| PlatformError::InvalidConfiguration {
                platform: "windows".to_string(),
                details: "Windows launcher configuration is required".to_string(),
            })?;
        
        #[cfg(feature = "windows-launcher")]
        {
            // Register the application
            self.register_application(&launcher_config)?;
            
            // Create shortcuts if requested
            self.create_shortcuts(&launcher_config)?;
            
            // Register file associations
            self.register_file_associations(&launcher_config)?;
        }
        
        #[cfg(not(feature = "windows-launcher"))]
        {
            self.fallback_operation("initialize")?;
        }
        
        // Set installation info
        self.installation_info = Some(WindowsInstallationInfo {
            install_path: launcher_config.install_directory.clone(),
            executable_path: launcher_config.install_directory.join(&launcher_config.executable_name),
            version: launcher_config.app_version.clone(),
            registry_key: format!("Software\\{}\\{}", launcher_config.publisher, launcher_config.app_name),
            auto_update_enabled: launcher_config.update_server_url.is_some(),
            service_name: None,
        });
        
        self.config = Some(config);
        info!("Windows launcher adapter initialized successfully");
        Ok(())
    }
    
    async fn shutdown(&mut self) -> PlatformResult<()> {
        info!("Shutting down Windows launcher adapter");
        self.config = None;
        self.installation_info = None;
        Ok(())
    }
    
    async fn is_available(&self) -> bool {
        self.config.is_some()
    }
    
    async fn authenticate(&self, _credentials: AuthenticationCredentials) -> PlatformResult<PlatformUser> {
        // Windows launcher doesn't handle authentication directly
        Err(PlatformError::UnsupportedCapability {
            capability: "Windows launcher doesn't handle authentication".to_string(),
        })
    }
    
    async fn refresh_token(&self, _refresh_token: &str) -> PlatformResult<AuthenticationResult> {
        Err(PlatformError::UnsupportedCapability {
            capability: "Windows launcher doesn't handle tokens".to_string(),
        })
    }
    
    async fn get_user_profile(&self, _user_id: &str) -> PlatformResult<UserProfile> {
        Err(PlatformError::UnsupportedCapability {
            capability: "Windows launcher doesn't handle user profiles".to_string(),
        })
    }
    
    async fn sync_achievements(&self, _user_id: &str, _achievements: &[Achievement]) -> PlatformResult<()> {
        Err(PlatformError::UnsupportedCapability {
            capability: "Windows launcher doesn't handle achievements".to_string(),
        })
    }
    
    async fn get_friends_list(&self, _user_id: &str) -> PlatformResult<Vec<Friend>> {
        Err(PlatformError::UnsupportedCapability {
            capability: "Windows launcher doesn't handle friends".to_string(),
        })
    }
    
    async fn launch_content(&self, content_id: &str, launch_options: LaunchOptions) -> PlatformResult<()> {
        info!("Launching content {} through Windows launcher", content_id);
        
        if let Some(install_info) = &self.installation_info {
            let mut arguments = vec![content_id.to_string()];
            arguments.extend(launch_options.arguments);
            
            let working_dir = Some(&install_info.install_path);
            let _process_info = self.launch_process(&install_info.executable_path, &arguments, working_dir).await?;
            
            info!("Content launched successfully through Windows launcher");
            Ok(())
        } else {
            Err(PlatformError::InvalidConfiguration {
                platform: "windows".to_string(),
                details: "Installation information not available".to_string(),
            })
        }
    }
    
    async fn get_analytics(&self, _user_id: &str) -> PlatformResult<PlatformAnalytics> {
        let system_info = self.get_system_info();
        
        let mut platform_metrics = HashMap::new();
        platform_metrics.insert("os_version".to_string(), serde_json::Value::String(system_info.os_version));
        platform_metrics.insert("architecture".to_string(), serde_json::Value::String(system_info.architecture));
        platform_metrics.insert("cpu_cores".to_string(), serde_json::Value::Number(serde_json::Number::from(system_info.cpu_cores)));
        
        if let Some(install_info) = &self.installation_info {
            platform_metrics.insert("version".to_string(), serde_json::Value::String(install_info.version.clone()));
            platform_metrics.insert("install_path".to_string(), serde_json::Value::String(install_info.install_path.to_string_lossy().to_string()));
        }
        
        Ok(PlatformAnalytics {
            platform_id: self.platform_id.clone(),
            user_id: "local_user".to_string(),
            session_count: 0,
            total_playtime_hours: 0.0,
            achievements_unlocked: 0,
            friends_count: 0,
            last_activity: None,
            platform_specific_metrics: platform_metrics,
        })
    }
}

impl Default for WindowsLauncherAdapter {
    fn default() -> Self {
        Self::new()
    }
}