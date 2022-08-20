use bevy::prelude::*;
use ron::de::from_reader;
use ron::ser::to_writer;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sync_settings_with_fs)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    // Add Settings to the shared resources
    commands.insert_resource(Settings::load());
}

fn sync_settings_with_fs(settings: Res<Settings>) {
    if settings.is_changed() {
        // Should save new settings data
        settings.save();
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Audio {
    pub state: bool,
    pub volume: i8,
}

/// All user settings
///
/// Note: Works with file system
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Settings {
    pub audio: Audio,
}

impl Settings {
    /// Returns a default config from `settings.ron` file
    fn get_default_config() -> Self {
        let default_settings_file =
            File::open("resources/settings.ron").expect("Failed opening file");

        from_reader(default_settings_file).expect("Unable to parse the file")
    }

    /// Returns a user config
    /// Takes it from `resources/settings.ron` (default config) file or
    ///  from existing config specifically for local user session
    pub fn load() -> Self {
        match File::open("temporary/settings.ron") {
            Ok(current_settings) => {
                // If everything is fine we may create Settings config from it
                // Note: We should validate current settigs and default settings
                //  we may have different configs even if we have `current_settigs` file
                //  when we update something in default config as an example
                from_reader(current_settings).expect("Unable to load settings")
            }
            // There is no current settings yet, then we have to create it
            Err(_) => {
                // Let's create a directory for current settings
                fs::create_dir_all("temporary").expect("Unable to create a file");

                // Read default settings
                let default_settings = Self::get_default_config();

                // Save default_settings into current settings
                let current_settings_file =
                    File::create("temporary/settings.ron").expect("Cannot create a file");

                to_writer(&current_settings_file, &default_settings)
                    .expect("Unable to save to the file");

                default_settings
            }
        }
    }

    /// Saves mutated `settings` to the file system
    pub fn save(&self) {
        let current_settings_file =
            File::create("temporary/settings.ron").expect("Cannot open the file");

        to_writer(&current_settings_file, &self).expect("Must save settings");
    }
}
