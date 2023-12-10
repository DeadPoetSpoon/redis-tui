use crate::args::AppArgs;

#[derive(Debug, Default)]
pub struct App {
    /// quit flag
    pub should_quit: bool,
    /// redis url
    pub redis_urls: Vec<String>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(app_args: &AppArgs) -> Self {
        let mut urls: Vec<String> = Vec::new();
        for url in &app_args.url {
            urls.push(url.to_string());
        }
        App {
            should_quit: false,
            redis_urls: urls,
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

}
