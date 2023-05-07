use std::env::home_dir;
use std::fs;
use std::path::Path;

fn main() {
    // Locate the user's home directory
    let mut dir_path;
    let home_dir: String = match env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .or_else(|_| env::var("HOMEDRIVE").and_then(|home_drive| env::var("HOMEPATH").map(|home_path| home_drive + &home_path)))
    {
        Ok(home) => home,
        Err(_) => {
            eprintln!("Failed to locate user's home directory");
            let dir_path = std::fs::Path::new(home_dir + "/tprs");
            match std::fs::create_dir_all(&dir_path) {
                Ok(()) => println!("Directory created successfully: {:?}", dir_path),
                Err(e) => eprintln!("Failed to create directory: {}", e),
            }
            return;
        }
    };


    let config_path = dir_path + "/config.yaml";


    
    println!("env_vars: {:?}", config_path);
    // v.for_each(|(k, v)| println!("{}: {}", k, v));
    let mut config_content = String::new();
    match File::open(&config_path) {
        Ok(mut file) => {
            if let Err(e) = file.read_to_string(&mut config_content) {
                eprintln!("Failed to read config file: {}", e);
                return;
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Config file not found: {}", e);
            } else {
                eprintln!("Failed to open config file: {}", e);
                return;
            }
        }
    }
    
}