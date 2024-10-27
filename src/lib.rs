use std::{char, collections::HashMap};

pub trait ChooseYourLanguage {
    fn new(state: bool, message: String, delimiter: char) -> Self;

    fn get_state(&self) -> &bool;
    fn get_message(&self) -> &str;
    fn get_keys(&self) -> &Vec<char>;
    fn get_dict(&self) -> &HashMap<char, String>;
    fn get_delimiter(&self) -> &char;

    fn set_state(&mut self, new_state: bool);
    fn set_message(&mut self, new_message: String);
    fn set_keys(&mut self, new_keys: Vec<char>);
    fn set_dict(&mut self, new_dict: HashMap<char, String>);
    fn set_delimiter(&mut self, new_delimiter: char);


    fn make_dict<T>(&mut self, dict_value: T) where T: IntoIterator<Item = String>;
    fn crypt(&self) -> String;
    fn decrypt(&self) -> String;
}

pub struct Message {
    state: bool,
    message: String,
    keys: Vec<char>,
    dict: HashMap<char, String>,
    delimiter: char
}

impl ChooseYourLanguage for Message {

    fn new(state: bool, message: String, delimiter: char) -> Self {
        Message {
            state,
            message,
            keys: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
            dict: HashMap::new(),
            delimiter
        }
    }

    fn get_state(&self) -> &bool {
        &self.state
    }

    fn get_message(&self) -> &str {
        &self.message
    }

    fn get_keys(&self) -> &Vec<char> {
        &self.keys
    }

    fn get_dict(&self) -> &HashMap<char, String> {
        &self.dict
    }

    fn get_delimiter(&self) -> &char {
        &self.delimiter
    }

    fn set_state(&mut self, new_state: bool) {
        self.state = new_state;
    }

    fn set_message(&mut self, new_message: String) {
        self.message = new_message;
    }

    fn set_dict(&mut self, new_dict: HashMap<char, String>) {
        self.dict = new_dict;
    }

    fn set_keys(&mut self, new_keys: Vec<char>) {
        self.keys = new_keys;
    }

    fn set_delimiter(&mut self, new_delimiter: char) {
        self.delimiter = new_delimiter;
    }

    fn make_dict<T>(&mut self, dict_value: T) // Can use default dictionnary or create your own
    where T: IntoIterator<Item = String> {
        self.dict = self.keys.clone().into_iter().zip(dict_value.into_iter()).collect();
    }

    fn crypt(&self) -> String {
        if self.state == false {
            let mut encrypted_message:String = String::new();
            for i in self.message.chars() { 
                if self.dict.contains_key(&i) {
                    if let Some(value) = self.dict.get(&i) {
                        encrypted_message += value;
                    }
                }
                else {
                    encrypted_message.push(i);
                }
                encrypted_message.push(self.delimiter);
            }
            return encrypted_message
        }
        return String::from("ERROR: Verify if your message is not already encrypted!")
    }

    fn decrypt(&self) -> String {
        if self.state == true {
            let mut decrypted_message:String = String::new();
    
            let segments: Vec<&str> = self.message.split(self.delimiter).collect();
            for segment in segments {
                if let Some(&key) = self.dict.iter().find_map(|(k, v)| { if v == segment { Some(k) } else { None } }) {
                    decrypted_message.push(key);
                }
                else {
                    decrypted_message.push(' ');
                }
            }
            return decrypted_message
        }
        return String::from("ERROR: Verify if your message is not already crypted!")
    }
}