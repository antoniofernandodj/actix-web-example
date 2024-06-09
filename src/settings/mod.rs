use std::{env, path::PathBuf};
use dotenv;

const DEV_FILE: &str = "src/settings/.dev.env";
const PROD_FILE: &str = "src/settings/.prod.env";
const STAGING_FILE: &str = "src/settings/.staging.env";


pub fn init() {

    let mut environment: String = "DEV".to_owned();
    if let Ok(env) = env::var("ENV") {
        environment = env.to_owned()
    }

    match environment.as_str() {
        "PROD"      => load_prod(),
        "STAGING"   => load_staging(),
        _           => load_dev()
    }
}

fn load_dev() {
    let result: Result<PathBuf, dotenv::Error> = dotenv::from_filename(DEV_FILE);
    if let Err(_) = result {
        panic!("Cannot load dev file");
    } else {
        println!("Loaded dev config")
    }
}

fn load_prod() {
    let result: Result<PathBuf, dotenv::Error> = dotenv::from_filename(PROD_FILE);
    if let Err(_) = result {
        panic!("Cannot load prod file");
    } else {
        println!("Loaded prod config")
    }
}

fn load_staging() {
    let result: Result<PathBuf, dotenv::Error> = dotenv::from_filename(STAGING_FILE);
    if let Err(_) = result {
        panic!("Cannot load staging file");
    } else {
        println!("Loaded staging config")
    }
}
