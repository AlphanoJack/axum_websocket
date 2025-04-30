#[derive(Debug, Deserialize)]
pub struct App{
    pub config: Config,
}

impl App {
    pub fn newApp(config: Config) -> Self {
        // load config
        let init_config = Config::init_config();

        // set Router
    }
}
