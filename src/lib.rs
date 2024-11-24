use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

/// Represents the configuration structure for the application.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_url: String,  // URL for the database connection
    pub port: u16,             // Port number the server will listen on
    pub debug: bool,           // Flag indicating if the application is in debug mode
}

/// Loads configuration data from a file and deserializes it into the specified type `T`.
///
/// This function supports `.toml`, `.json`, and `.yaml` file formats. It reads the file contents,
/// determines the file type based on the file extension, and deserializes the content into a
/// `Config` struct (or another specified type `T`).
///
/// # Arguments
/// * `path` - A string slice that holds the file path of the configuration file.
///
/// # Returns
/// A result that contains the deserialized configuration data on success or an error on failure.
pub fn load_config<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, Box<dyn Error>> {
    // Read the contents of the file into a string
    let file_contents = fs::read_to_string(path)?;

    // Deserialize based on the file extension
    if path.ends_with(".toml") {
        // Deserialize as TOML format
        let config: T = toml::de::from_str(&file_contents)?;
        Ok(config)
    } else if path.ends_with(".json") {
        // Deserialize as JSON format
        let config: T = serde_json::from_str(&file_contents)?;
        Ok(config)
    } else if path.ends_with(".yaml") {
        // Deserialize as YAML format
        let config: T = serde_yaml::from_str(&file_contents)?;
        Ok(config)
    } else {
        // Return an error if the file format is unsupported
        Err("Unsupported file format".into())
    }
}

/// Serializes the provided configuration data and saves it to a file.
///
/// This function supports `.toml`, `.json`, and `.yaml` file formats. It serializes the provided
/// configuration into the appropriate format and writes it to the specified file.
///
/// # Arguments
/// * `path` - A string slice that holds the file path where the configuration will be saved.
/// * `config` - A reference to the configuration data that needs to be serialized and saved.
///
/// # Returns
/// A result indicating success or failure of the operation.
pub fn save_config<T: Serialize>(path: &str, config: &T) -> Result<(), Box<dyn Error>> {
    // Serialize the configuration based on the file extension
    let serialized = if path.ends_with(".toml") {
        // Serialize as TOML format
        toml::to_string(config)?
    } else if path.ends_with(".json") {
        // Serialize as JSON format
        serde_json::to_string(config)?
    } else if path.ends_with(".yaml") {
        // Serialize as YAML format
        serde_yaml::to_string(config)?
    } else {
        // Return an error if the file format is unsupported
        return Err("Unsupported file format".into());
    };

    // Write the serialized data to the file
    fs::write(path, serialized)?;
    Ok(())
}
