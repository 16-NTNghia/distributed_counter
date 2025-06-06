use dotenv::{dotenv, var};



pub fn load_env(){
    dotenv().ok().expect("Failed to load env");
}

pub fn get_env(key: &str) -> String {
    var(key).expect("key not found").to_string()
}