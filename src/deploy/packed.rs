#[path = "deploy.rs"]
pub(crate) mod deploy;

use std::collections::HashMap;

pub struct Packed {
    pub project_config: String
}

impl Packed {
    // Example: https://github.com/mehcode/config-rs/tree/master/examples/simple
    fn init_config(&self) {
        println!("Initiating config!");
        let mut config = config::Config::default();
        config.merge(config::File::with_name("Settings")).unwrap();
        println!("{:?}",
                 config.try_into::<HashMap<String, String>>().unwrap());
    }

    fn download(&self) {
        println!("downloading!");
    }

    fn unpack(&self) {
        println!("unpacking!");
    }

    fn move_to_dir(&self) {
        println!("moving to directory!");
    }

    fn switch_env_stage(&self) {
        println!("Switch stage .env!")
    }

    fn switch_stage_symlink(&self) -> bool {
        println!("switching symlink!");
        return true;
    }

    fn switch_env_production(&self) {
        println!("Switch production .env!")
    }

    fn switch_production_symlink(&self) -> bool {
        println!("Switching production symlink!");
        return true;
    }
}

impl crate::packed::deploy::Deploy for Packed {
    fn init(&self) {
        self.init_config();
    }

    fn deploy(&self) -> bool {
        self.download();
        self.unpack();
        self.move_to_dir();
        self.switch_env_stage();
        self.switch_stage_symlink()
    }

    fn release(&self) -> bool {
        self.switch_env_production();
        self.switch_production_symlink()
    }
}
