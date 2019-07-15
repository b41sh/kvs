extern crate serde;
extern crate serde_json;
extern crate serde_repr;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Write, BufReader, BufRead};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Cmd {
    SET = 1,
    RM = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    cmd: Cmd,
    key: String,
    val: String,
}

#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new KvStore.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        //self.map.insert(key, value);
        let log = Log { 
            cmd: Cmd::SET,
            key: key,
            val: value
        };
        let ser_log = serde_json::to_string(&log).unwrap();

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("/tmp/kvs")
            .unwrap();

        if let Err(e) = writeln!(file, "{}", ser_log) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    pub fn get(&mut self, key: String) -> Option<String> {

        let file = File::open("/tmp/kvs").unwrap();
        let reader = BufReader::new(file);

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            // Show the line and its number.

            let log_data: Log = serde_json::from_str(&line).unwrap();

            let log_cmd = log_data.cmd;
            let log_key = log_data.key;
            let log_val = log_data.val;
            if log_cmd == Cmd::SET {
                self.map.insert(log_key, log_val);
            } else if log_cmd == Cmd::RM {
                self.map.remove(&log_key);
            }
        }

        //self.map.get(key)
        self.map.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        let log = Log { 
            cmd: Cmd::RM,
            key: key,
            val: "None".to_string(),
        };
        let ser_log = serde_json::to_string(&log).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("/tmp/kvs")
            .unwrap();

        if let Err(e) = writeln!(file, "{}", ser_log) {
            eprintln!("Couldn't write to file: {}", e);
        }
        //self.map.remove(&key);
    }
}
