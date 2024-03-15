use redis::{Client, Commands, Connection, FromRedisValue};
use std::result::Result::Ok;
use std::{fmt::Debug, str::from_utf8};

extern crate redis;

#[derive(Debug, Default)]
pub enum ValueKind {
    #[default]
    String,
    List,
    Set,
    ZSet,
    Hash,
    Stream,
}
impl FromRedisValue for ValueKind {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        if let redis::Value::Status(x) = v {
            match x.as_str() {
                "string" => Ok(ValueKind::String),
                "list" => Ok(ValueKind::List),
                "set" => Ok(ValueKind::Set),
                "zset" => Ok(ValueKind::ZSet),
                "hash" => Ok(ValueKind::Hash),
                "stream" => Ok(ValueKind::Stream),
                _ => Ok(ValueKind::String),
            }
        } else {
            Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "key typy err",
                format!("{v:?}"),
            )))
        }
    }
}

#[derive(Debug, Default)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
    pub value_kind: Option<ValueKind>,
    pub is_open: bool,
}
impl KeyValuePair {
    pub fn new(key: String) -> Self {
        KeyValuePair {
            key,
            ..Default::default()
        }
    }
    pub fn get_key_title(&self) -> String {
        if let Some(value_kind) = &self.value_kind {
            format!("{}({:?})", self.key, value_kind)
        } else {
            format!("{}", self.key)
        }
    }
}

#[derive(Default)]
pub struct RedisConn {
    pub url: String,
    pub open_kvs: Vec<KeyValuePair>,
    pub current_key_index: usize,
    client: Option<Client>,
    conn: Option<Connection>,
}

impl RedisConn {
    pub fn new(url: String) -> Self {
        RedisConn {
            url,
            ..Default::default()
        }
    }

    pub fn is_conn(&mut self) -> bool {
        match self.conn {
            Some(_) => true,
            None => false,
        }
    }
    pub fn conn(&mut self) -> redis::RedisResult<()> {
        self.client = Some(redis::Client::open(self.url.to_string())?);
        self.conn = Some(self.client.as_ref().expect("conn err").get_connection()?);
        Ok(())
    }
    pub fn delete_current_key(&mut self) {
        self.open_kvs.remove(self.current_key_index);
        self.switch_to_before_key();
    }
    pub fn get_keys(&mut self, pattern: &mut String) {
        let result_keys: Vec<String> = self
            .conn
            .as_mut()
            .expect("conn err")
            .keys(pattern.to_string())
            .expect("get key error");
        for key in result_keys {
            self.open_kvs.push(KeyValuePair::new(key.to_string()));
        }
    }
    pub fn switch_to_next_key(&mut self) {
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
    pub fn get_curret_key(&self) -> String {
        self.open_kvs[self.current_key_index].key.to_string()
    }
    pub fn fetch_curret_value(&mut self) {
        let key = self.get_curret_key();
        if self.open_kvs[self.current_key_index].value_kind.is_none() {
            self.open_kvs[self.current_key_index].value_kind = self
                .conn
                .as_mut()
                .expect("conn err")
                .key_type(&key)
                .expect("err");
        }
        if let Ok(value) = self.conn.as_mut().expect("conn err").get(&key) {
            self.open_kvs[self.current_key_index].value = value;
        } else {
            let vec_value: Vec<String> = self
                .conn
                .as_mut()
                .expect("conn err")
                .smembers(&key)
                .expect("err vec value");
            self.open_kvs[self.current_key_index].value = vec_value.join("\n");
        }
    }
    pub fn get_curret_value(&self) -> String {
        if self.open_kvs.is_empty() {
            return "".to_string();
        }
        self.open_kvs[self.current_key_index].value.to_string()
    }
}
