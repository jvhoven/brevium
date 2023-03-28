use macros::item;
use maplit::hashmap;
use std::collections::HashMap;
use std::fmt;

use crate::storage::Saveable;

item!(TextItem<'a> {});

impl Saveable for TextItem<'_> {
    fn get_title(&self) -> &str {
        todo!()
    }

    fn get_tags(&self) -> HashMap<&str, &str> {
        hashmap! {
            "type" => "text",
        }
    }

    fn get_content(&self) -> &str {
        todo!()
    }
}
