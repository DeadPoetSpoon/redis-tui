use crate::args::AppArgs;
use crate::redis::RedisConn;

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Home,
    About,
}

#[derive(Debug, Default)]
pub enum KeyMode {
    #[default]
    Command,
    Input,
}



#[derive(Default)]
pub struct App {
    /// quit flag
    pub should_quit: bool,
    /// draw once
    pub draw_once:bool,
    /// redis url
    pub redis_urls: Vec<String>,
    /// current_url_index
    pub current_url_index : usize,
    /// current screen
    pub current_screen: CurrentScreen,
    /// key mode
    pub key_mode: KeyMode,
    pub redis_conns: Vec<RedisConn>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(app_args: &AppArgs) -> Self {
        let mut redis_urls: Vec<String> = Vec::new();
        let mut redis_conns: Vec<RedisConn> = Vec::new();
        for url in &app_args.url {
            redis_urls.push(url.to_string());
            redis_conns.push(RedisConn::new(url.to_string()));
        }
        App {
            redis_urls,
            redis_conns,
            ..Default::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// get urls string
    pub fn get_redis_url_string(&self) -> String {
        self.redis_urls.join("\n")
    }
    pub fn get_current_redis_url(&self) -> String{
        self.redis_urls[self.current_url_index].to_string()
    }
    pub fn get_redis_urls_len(&self) -> usize{
        self.redis_urls.len()
    }
    pub fn get_current_redis_conn(&mut self)->&mut RedisConn{
        &mut self.redis_conns[self.current_url_index]
    }

}
