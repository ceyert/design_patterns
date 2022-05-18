use std::sync::{Arc, Mutex};


#[derive(Debug)]
pub struct Config {
    db_connection_url: String,
}

pub fn db_connection() -> Arc<Mutex<Config>> {
    static mut  CONF: Option<Arc<Mutex<Config>>> = None;

    unsafe {
        CONF.get_or_insert_with(|| {
            println!("INIT");
            Arc::new(Mutex::new(Config { db_connection_url: "test config".to_string(), }))
        }).clone()
    }
}