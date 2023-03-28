use crate::item::code::CodeItem;
use crate::item::text::TextItem;
use core::str::FromStr;

#[derive(Debug)]
pub enum ItemTypes<'a> {
    Text(TextItem<'a>),
    Code(CodeItem<'a>),
}

impl FromStr for ItemTypes<'_> {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim() {
            "text" => Ok(ItemTypes::Text(Default::default())),
            "code" => Ok(ItemTypes::Code(Default::default())),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_entry_from_str() {
        assert_eq!("text".parse::<ItemTypes>().is_ok(), true);
    }

    #[test]
    fn should_not_parse_unknown_entry() {
        assert_eq!("unknown".parse::<ItemTypes>().is_err(), true);
    }
}
