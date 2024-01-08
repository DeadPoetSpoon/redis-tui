use redis::{Client, Connection, Commands};

extern crate redis;

#[derive(Debug, Default)]
pub struct KeyValuePair{
    pub key: String,
    pub value: String,
    pub is_open: bool,
    
}
impl KeyValuePair {
    pub fn new(key:String)->Self{
        KeyValuePair{
            key,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct RedisConn{
    pub url: String,
    pub open_kvs: Vec<KeyValuePair>,
    pub current_key_index: usize, 
    client:Option<Client>,
    conn:Option<Connection>,
}

impl RedisConn {
    pub fn new(url:String)->Self{
        RedisConn{
            url,
            ..Default::default()
        }
    }
    pub fn conn(&mut self) -> redis::RedisResult<()> {
        self.client = Some(redis::Client::open(self.url.to_string())?);
        self.conn = Some(self.client.as_ref().expect("conn err").get_connection()?);
        Ok(())
    }
    pub fn get_keys(&mut self,pattern:String){
        let result_keys:Vec<String> = self.conn.as_mut().expect("conn err").keys(pattern.to_string()).expect("get key error");
        for key in result_keys{
            self.open_kvs.push(KeyValuePair::new(key.to_string()));
        }
    }
    pub fn switch_to_next_key(& mut self) {
        if let Some(res) = self.current_key_index.checked_add(1) {
            if res < self.open_kvs.len() {
                self.current_key_index = res
            }
        }
    }
    pub fn switch_to_before_key(&mut self) {
        if let Some(res) = self.current_key_index.checked_sub(1) {
            self.current_key_index = res
        }
    }
    pub fn get_curret_key(&self)->String{
        self.open_kvs[self.current_key_index].key.to_string()
    }
    pub fn fetch_curret_value(&mut self) {
        let key = self.get_curret_key();
        self.open_kvs[self.current_key_index].value = self.conn.as_mut().expect("conn err").get(key).expect("get value err");
    }
    pub fn get_curret_value(&self)->String{
        if self.open_kvs.is_empty() {
            return "".to_string();
        }
        self.open_kvs[self.current_key_index].value.to_string()
    }
}