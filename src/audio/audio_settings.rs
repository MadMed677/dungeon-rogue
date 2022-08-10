use ron::de::from_reader;
use ron::ser::to_writer;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Audio {
    pub state: bool,
    pub volume: i8,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Settings {
    pub audio: Audio,
}

impl Settings {
    fn get_default_config() -> Self {
        let default_settings_file =
            File::open("resources/settings.ron").expect("Failed opening file");

        from_reader(default_settings_file).expect("Unable to parse the file")
    }

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
}
