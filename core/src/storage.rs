use std::collections::HashMap;

pub trait Storage {
    fn add_item(&self, item: &impl Saveable) -> Result<(), Box<dyn std::error::Error>>;
    // fn list(&self) -> Result<Vec<T>, E>;
}

pub trait Saveable {
    fn get_title(&self) -> &str;
    fn get_content(&self) -> &str;
    fn get_tags(&self) -> HashMap<&str, &str>;
}
