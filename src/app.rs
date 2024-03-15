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
    Input(InputKind),
}

#[derive(Debug, Default)]
pub enum InputKind {
    #[default]
    SearchPattern,
}

#[derive(Default)]
pub struct App {
    /// quit flag
    pub should_quit: bool,
    /// draw once
    pub draw_once: bool,
    /// redis url
    pub redis_urls: Vec<String>,
    /// current_url_index
    pub current_url_index: usize,
    /// current screen
    pub current_screen: CurrentScreen,
    /// key mode
    pub key_mode: KeyMode,
    pub input_key_pattern: String,
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
    pub fn get_current_redis_url(&self) -> String {
        self.redis_urls[self.current_url_index].to_string()
    }
    pub fn get_redis_urls_len(&self) -> usize {
        self.redis_urls.len()
    }
    pub fn get_current_redis_conn(&mut self) -> &mut RedisConn {
        &mut self.redis_conns[self.current_url_index]
    }
    pub fn get_current_keys(&mut self) {
        let conn = &mut self.redis_conns[self.current_url_index];
        // let pattern = String::from(self.input_key_pattern);
        conn.get_keys(&mut self.input_key_pattern);
    }
    pub fn switch_to_next_key(&mut self) {
        self.get_current_redis_conn().switch_to_next_key();
    }
    pub fn switch_to_before_key(&mut self) {
        self.get_current_redis_conn().switch_to_before_key();
    }
    pub fn conn_current(&mut self) -> redis::RedisResult<()> {
        self.get_current_redis_conn().conn()
    }
    pub fn fetch_curret_value(&mut self) {
        self.get_current_redis_conn().fetch_curret_value();
    }
    pub fn delete_current_key(&mut self) {
        self.get_current_redis_conn().delete_current_key();
    }
    pub fn switch_to_next_conn(&mut self) {
        if let Some(x) = self.current_url_index.checked_add(1) {
            if x < self.redis_urls.len() {
                self.current_url_index = x;
            } else {
                self.current_url_index = 0;
            }
        }
    }

    pub fn switch_to_before_conn(&mut self) {
        if let Some(x) = self.current_url_index.checked_sub(1) {
            self.current_url_index = x;
        } else {
            self.current_url_index = self.redis_urls.len() - 1;
        }
    }

    pub fn switch_to_next_screen(&mut self) {
        match self.current_screen {
            CurrentScreen::Home => self.current_screen = CurrentScreen::About,
            CurrentScreen::About => self.current_screen = CurrentScreen::Home,
        }
    }
}
