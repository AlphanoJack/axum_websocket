// use serde::Deserialize;
// use dotenv::dotenv;
// use std::env;

// #[derive(Debug, Deserialize)]
// pub struct Config {
//     pub server_port: u16,
// }

// impl Config {
//     pub fn init_config() -> Self {
//         dotenv().ok();

//         let server_port = env::var("SERVERPORT")
//             .unwrap_or_else(|_| "8090".to_string())
//             .parse::<u16>()
//             .expect("SERVER_PORT must be a number");

//         Config {
//             server_port
//         }
//     }
// }
