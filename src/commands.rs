use crate::client_state::ClientState;
use crate::memory::{MemoryDb, Value};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CommandsEval {
    pub database: Arc<Mutex<MemoryDb>>,
}

impl CommandsEval {
    pub fn eval(&self, command: &str, client_state: &mut ClientState) -> String {
        if command.chars().last().unwrap() != ';' {
            return "Invalid command".to_string();
        }

        let command = &command[..command.len() - 1];
        let command = command.trim();
        let command = command.split("\n").collect::<Vec<&str>>();

        match command[0] {
            "AUTH" => self.auth(&command, client_state),
            _ => {
                if client_state.auth == false {
                    return "Not authenticated".to_string();
                }
                self.eval_command(command)
            }
        }
    }

    fn auth(&self, command: &Vec<&str>, client_state: &mut ClientState) -> String {
        if command.len() != 2 {
            return "Invalid command".to_string();
        }

        if command[1] == "password" {
            client_state.auth = true;
            return "OK".to_string();
        }

        "Invalid password".to_string()
    }

    fn eval_command(&self, command: Vec<&str>) -> String {
        match command[0] {
            "GET" => self.get(&command),
            "SET" => self.set(&command),
            "INCREMENT" => self.increment(&command),
            "DECREMENT" => self.decrement(&command),
            "RENAME" => self.rename(&command),
            "COPY" => self.copy(&command),
            "DEL" => self.del(&command),
            "EXISTS" => self.exists(&command),
            "TTL" => self.ttl(&command),
            "EXPIRE" => self.expire(&command),
            "EXPIRETIME" => self.expiretime(&command),
            "PERSIST" => self.persist(&command),
            _ => "Invalid command".to_string(),
        }
    }

    fn get(&self, command: &Vec<&str>) -> String {
        if command.len() != 2 {
            return "Invalid command".to_string();
        }

        let db = self.database.lock().unwrap();
        let value = db.get(command[1]);

        if value.is_none() {
            return "Key does not exist".to_string();
        }

        value.unwrap().to_string()
    }

    fn set(&self, command: &Vec<&str>) -> String {
        if command.len() != 4 {
            return "Invalid command".to_string();
        }

        let t = command[1].to_string();

        let value = match t.as_str() {
            "INT" => Value::Int(command[3].parse::<i32>().unwrap()),
            "FLOAT" => Value::Float(command[3].parse::<f32>().unwrap()),
            "STRING" => Value::Str(command[3].to_string()),
            "BOOL" => Value::Bool(command[3].parse::<bool>().unwrap()),
            _ => return "Invalid type".to_string(),
        };

        self.database
            .lock()
            .unwrap()
            .set(command[2].to_string(), value);

        "OK".to_string()
    }

    fn increment(&self, command: &Vec<&str>) -> String {
        if command.len() != 3 {
            return "Invalid command".to_string();
        }

        let mut db = self.database.lock().unwrap();

        let value = db.get(command[1]).unwrap();

        let inc_val: f32 = command[2].parse().unwrap();

        let new_value = match value {
            Value::Int(i) => Value::Int(i + inc_val as i32),
            Value::Float(f) => Value::Float(f + &inc_val),
            _ => return "Invalid type".to_string(),
        };

        db.set(command[1].to_string(), new_value);

        "OK".to_string()
    }

    fn decrement(&self, command: &Vec<&str>) -> String {
        if command.len() != 3 {
            return "Invalid command".to_string();
        }

        let mut db = self.database.lock().unwrap();

        let value = db.get(command[1]).unwrap();

        let inc_val: f32 = command[2].parse().unwrap();

        let new_value = match value {
            Value::Int(i) => Value::Int(i - inc_val as i32),
            Value::Float(f) => Value::Float(f - &inc_val),
            _ => return "Invalid type".to_string(),
        };

        db.set(command[1].to_string(), new_value);

        "OK".to_string()
    }

    fn ttl(&self, command: &Vec<&str>) -> String {
        if command.len() != 2 {
            return "Invalid command".to_string();
        }

        let mut db = self.database.lock().unwrap();

        let mut ttl = db.get_ttl(command[1]);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        if ttl == -2 || ttl == -1 {
            return ttl.to_string();
        }

        ttl = ttl - now;

        if ttl < 0 {
            db.remove(command[1]);
            return "-2".to_string();
        }

        ttl.to_string()
    }

    fn expire(&self, command: &Vec<&str>) -> String {
        if command.len() != 3 {
            return "Invalid command".to_string();
        }

        let mut time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        time += command[2].parse::<i64>().unwrap();

        let mut db = self.database.lock().unwrap();

        let exists = db.get_ttl(command[1]) != -2;

        if !exists {
            return "Key does not exist".to_string();
        }

        db.set_ttl(command[1].to_string(), time);

        "OK".to_string()
    }

    fn expiretime(&self, command: &Vec<&str>) -> String {
        if command.len() != 3 {
            return "Invalid command".to_string();
        }

        let mut db = self.database.lock().unwrap();

        let ttl = db.get_ttl(command[1]);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        if ttl == -2 || ttl == -1 {
            return ttl.to_string();
        }

        if (ttl - now) < 0 {
            db.remove(command[1]);
            return "-2".to_string();
        }

        ttl.to_string()
    }

    fn rename(&self, command: &Vec<&str>) -> String {
        if command.len() != 3 {
            return "Invalid command".to_string();
        }

        self.database.lock().unwrap().rename(command[1], command[2]);

        "OK".to_string()
    }

    fn copy(&self, command: &Vec<&str>) -> String {
        if command.len() != 3 {
            return "Invalid command".to_string();
        }

        self.database.lock().unwrap().copy(command[1], command[2]);

        "OK".to_string()
    }

    fn del(&self, command: &Vec<&str>) -> String {
        let mut db = self.database.lock().unwrap();

        for key in command.iter().skip(1) {
            db.remove(key);
        }

        "OK".to_string()
    }

    fn exists(&self, command: &Vec<&str>) -> String {
        let db = self.database.lock().unwrap();

        let mut count = 0;

        for key in command.iter().skip(1) {
            if db.get_ttl(key) != -2 {
                count += 1;
            }
        }

        count.to_string()
    }

    fn persist(&self, command: &Vec<&str>) -> String {
        if command.len() != 2 {
            return "Invalid command".to_string();
        }

        let mut db = self.database.lock().unwrap();

        let exists = db.get_ttl(command[1]) != -2;

        if !exists {
            return "Key does not exist".to_string();
        }

        db.set_ttl(command[1].to_string(), -1);

        "OK".to_string()
    }
}
