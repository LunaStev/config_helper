# Config Helper
`Config Helper` is a simple and flexible library for handling configuration files in various formats, such as TOML, JSON, and YAML. It allows you to easily load and save configuration data to and from these formats and automatically deserializes them into Rust structs.

## Features
* Multi-format support: Handles TOML, JSON, and YAML configuration files.
* Easy deserialization: Convert configuration files into Rust structs.
* Serialization support: Convert Rust structs back into configuration files in the specified format.
* Flexible API: Simple functions for loading and saving configuration data. 
## Installation
To use `Config Helper` in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
config_helper = "0.1.0"
```
## Usage
### Define a Configuration Struct
First, define a struct that matches the configuration format you expect to load.

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub debug: bool,
}
```
### Loading Configuration from a File
Use the `load_config` function to load a configuration file. It automatically detects the file format (TOML, JSON, or YAML) based on the file extension.

```rust
use config_helper::{load_config, Config};

fn main() {
    let config: Config = load_config("config.toml").expect("Failed to load config");
    println!("{:?}", config);
}
```

### Saving Configuration to a File
Use the `save_config` function to save a Rust struct as a configuration file. The file format is determined by the file extension.

```rust
use config_helper::{save_config, Config};

fn main() {
    let new_config = Config {
    database_url: "http://localhost".to_string(),
    port: 8080,
    debug: true,
    };

    save_config("new_config.toml", &new_config).expect("Failed to save config");
}
```

## Supported Formats
* TOML: Common format for Rust configuration files.
* JSON: Widely used and human-readable format.
* YAML: Often used for more complex configuration data.

## Error Handling
The library returns a `Result` type for both loading and saving configurations. Errors may occur due to invalid file format, deserialization issues, or I/O errors. Make sure to handle these errors appropriately in your application.

## Contributing
Feel free to open issues or submit pull requests for any improvements or bug fixes. Contributions are always welcome!

1. Fork the repository.
2. Create a branch for your feature or bug fix.
3. Make your changes and test thoroughly.
4. Submit a pull request with a clear explanation of your changes.

## License
This project is licensed under the MPL-2.0 License - see the [LICENSE](LICENSE) file for details.