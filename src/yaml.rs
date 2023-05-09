
use std::path::PathBuf;
use home::home_dir;
use serde_yaml;
use std::collections::HashMap;
use prettyprint::PrettyPrinter;

#[derive(Debug)]
pub struct Config {
    path: PathBuf,
    yaml_string: String,
    yaml_data: HashMap<String, serde_yaml::Value>,
}


impl Config {
    pub fn new() -> Config {
        let mut config = Config {
            path: PathBuf::from(""),
            yaml_string: String::from(""),
            yaml_data: HashMap::new(),
        };
        config.load();
        config
    }
    fn load(&mut self) {
        match home_dir() {
            Some(path) => {
                println!("User's home directory: {:?}", path);
                self.path = PathBuf::from(PathBuf::from(path).join(".tp").join("profile2.yaml"));
                match std::fs::read_to_string(&self.path) {
                    Ok(config_string) => {
                        // println!("Config file found:\n{}", config_string);
                        self.yaml_string = config_string;
                        match serde_yaml::from_str(&self.yaml_string){
                            Ok(yaml_data) => {
                                println!("Yaml data: {:?}", yaml_data);
                                self.yaml_data = yaml_data;
                                self.print_yaml_data();
                            },
                            Err(e) => {
                                println!("Failed to parse yaml: {:?}", e);
                            }
                        }

                    },
                    Err(e) => {
                        println!("Config file not found, creating...");
                        let config_string = include_str!("template/default_profile.yaml");
                        if let Err(e) = std::fs::write(&self.path, config_string) {
                            panic!("Failed to create config file: {:?}", e);
                        }
                        self.yaml_string = String::from(config_string);
                    }
                }
            },
            None => { panic!("Failed to locate user's home directory, please use a custom config path");}
        }
    }
    
    pub fn print_yaml_data(&self) {
        println!("{:#?}", self.yaml_data);
    }
}

pub fn get_config() -> Config {
    Config::new()
}
// fn init() {
//     const DEFAULT_CONFIG: &'static str = include_str!("template/default_profile.yaml");
//     let yaml_file = "profile2.yaml";
//     let mut config_string = "";

//     if let Some(home_path) = home_dir() {
//         println!("User's home directory: {:?}", home_path);
//         let tp_dir = PathBuf::from(PathBuf::from(home_path).join(".tp"));
//         println!("TP directory: {:?}", &tp_dir);
//         if let Ok(config_string) = std::fs::read_to_string(&tp_dir.join(&yaml_file )) {
//             println!("Config file found: {}", config_string);
//         } else {
//             // Error handling
//             match std::fs::create_dir_all(&tp_dir) {
//                 Ok(()) => {
//                     println!("Config file not found, creating...");
//                     let config_string = DEFAULT_CONFIG;
//                     if let Err(e) = std::fs::write(PathBuf::from(&tp_dir).join(&yaml_file ), config_string) {
//                         panic!("Failed to create config file: {:?}", e);
//                     }
//                 }
//                 Err(e) => {
//                     println!("Failed to create TP directory: {:?}", e);
//                     // Handle the error here, either by propagating it up the call stack or returning an error value
//                 }
//             }
//         }
        
//         if let Err(e) = std::fs::create_dir_all(&tp_dir) {
//             panic!("Failed to create TP directory: {:?}", e);
//         }
        
//     } else {
//         panic!("Failed to locate user's home directory");
//     }
//     // let yaml_str = std::fs::read_to_string("path/to/file.yaml")?;
// }



