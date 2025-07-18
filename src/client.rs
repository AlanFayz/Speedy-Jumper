use sapp_jsutils::JsObject;

use std::{collections::{BTreeMap}, sync::RwLock};
use once_cell::sync::Lazy;


unsafe extern "C" {
  fn _register_name(name: JsObject);
  fn _register_time(name: JsObject, time: f64);
}

pub static LEADERBOARD: Lazy<RwLock<BTreeMap<String, f32>>> = Lazy::new(|| RwLock::new(BTreeMap::new()));

#[unsafe(no_mangle)]
pub extern "C" fn _update_player(player: JsObject, score: f64) {
    let mut message = String::new();
    player.to_string(&mut message);

    if score < 0.0 {
        LEADERBOARD.write()
            .unwrap()
            .remove(&message);

        return;
    }

    LEADERBOARD.write()
        .unwrap()
        .insert(message, score as f32);
}

pub struct Client {
    client_name: String,
    leaderboard: BTreeMap<String, f32>
}

impl Client {
    pub fn empty() -> Client {
        Client {
            client_name: "empty".to_owned(), 
            leaderboard: BTreeMap::new()
        }
    }

    pub fn new(client_name: String) -> Result<Client, String> {
        if LEADERBOARD.write().unwrap().contains_key(&client_name) {
            return Err("name already present".to_owned());
        }

        let client = Client {
            client_name,
            leaderboard: LEADERBOARD.write().unwrap().clone()
        };

        client.register_name();
        
        return Ok(client);
    }
 
    pub fn register_name(&self) {
        if cfg!(target_arch = "wasm32") {
            unsafe {
                _register_name(JsObject::string(&self.client_name.as_str()));
            }
        } else {
            
        }
    }

    pub fn register_time(&self, time: f64) {
        if cfg!(target_arch = "wasm32") {
            unsafe {
                _register_time(JsObject::string(&self.client_name.as_str()), time);
            }
        } else {

        }
    }

    pub fn sync(&mut self) {
        self.leaderboard = LEADERBOARD.write().unwrap().clone();
    }

    pub fn get_leaderboard(&self) -> &BTreeMap<String, f32> {
        &self.leaderboard
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.register_time(-1.0);
    }
}
