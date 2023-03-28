#[macro_use]
extern crate tantivy;

use core::storage::Saveable;
use core::storage::Storage;

use tantivy::schema::*;
use tantivy::Index;

#[derive(Debug)]
pub struct TantivyStorage {}

impl TantivyStorage {
    pub fn new() -> Result<Self, String> {
        println!("Now using TantivyStorage");
        // let config = CONFIG.read().expect("Should have config");

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("content", TEXT);

        let dir = "/tmp/test";
        let schema = schema_builder.build();
        let index = Index::create_in_dir(dir, schema.clone()).expect("Could not initialzie index");
        let mut index_writer = index
            .writer(50_000_000)
            .expect("Could not initialize index writer");

        let title = schema.get_field("title").unwrap();
        let content = schema.get_field("content").unwrap();

        index_writer
            .add_document(doc!(
                title => "Of Mice and Men",
                content => "A few miles"
            ))
            .expect("Couldn't create document");

        index_writer
            .commit()
            .expect("Could not commit new document");

        Ok(Self {})
    }
}

impl Storage for TantivyStorage {
    fn add_item(&self, _item: &impl Saveable) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
