use macros::item;
use maplit::hashmap;
use std::collections::HashMap;
use std::fmt;

use crate::storage::Saveable;

item!(CodeItem<'a> { language: &'static str });

impl Saveable for CodeItem<'_> {
    fn get_title(&self) -> &str {
        todo!()
    }

    fn get_content(&self) -> &str {
        todo!()
    }

    fn get_tags(&self) -> HashMap<&str, &str> {
        hashmap! {
            "type" => "code",
            "language" => self.language
        }
    }
}
