use std::path::{PathBuf, Path}; 
use crate::backend::constants::{CONFIG_PATH, ENV_HOME_VAR};
use std::fs; 
use std::env::var; 

#[derive(Clone)]
pub struct Config {
   /// Number of notes number: ui64
   /// Vector of structs of notes. Vec!<notes>
   /// path to the config of the cli path: pathbuf
   /// if the config has been initialized or not init: bool 
   /// string of the file in config: text : string
   config_text: String,
   config_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

pub fn init_config() -> Result<Config, String> {
    let mut lazy_config = Config::new(); 
    match lazy_config.load_or_create_config_file() {
        Ok(_) => {
            Ok(lazy_config)
        },
        Err(e) => {
            Err(e)
        }
    }
}


impl Config {
    /// new fn that will be called when the terminal is opened
    /// It will be called regardless if the app has been started the first time or not. 
    /// if the once if false then it will create the file and the configuration file.  
    ///
    ///
    ///

    pub fn new() -> Self {
        Self {
            config_text: String::from(""), 
            config_path: PathBuf::from("")
        }
    }

    pub fn get_config_text(self) -> String {
        self.config_text
    }

    /// Check for a config file at CONFIG_PATH and if it does not exist then create that file
    /// otherwise load the file. If the file exists, then the application has been run atleast
    /// once. 
    fn load_or_create_config_file(&mut self) ->  Result<(), String> {
        // Get the environment variable home and appends it to the CONFIG_PATH
        var(ENV_HOME_VAR).
            map_err(|e| e.to_string()).
            map(|mut f| {
                f.push_str(CONFIG_PATH); 

                let path = Path::new(&f);


                path.exists().
                    then(|| {
                        self.config_path = PathBuf::from(path); 

                        if let Ok(text) =  fs::read_to_string(path).
                            map_err(|e| e.to_string()) {
                                self.config_text = text; 
                            }
                    }).
                is_none().
                    then(|| {
                        fs::create_dir_all(path.parent().unwrap()).
                            map_err(|e| e.to_string()).
                            map(|_| {
                                if fs::File::create(path).
                                    map_err(|e| e.to_string()).is_ok() {
                                        self.config_path = PathBuf::from(path); 
                                    }
                            })
                    });
            })
    }
}
