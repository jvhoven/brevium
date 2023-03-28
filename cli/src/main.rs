use clap::{Parser, Subcommand};
use core::config::CONFIG;
use core::item::text::TextItem;
use core::storage::Storage;
use maplit::hashmap;
use storage_fs::FileStorage;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // #[clap(about = "Sync to a remote Brevium library")]
    // Sync,
    #[clap(about = "Configure a Brevium library or update current settings")]
    Setup,
    #[clap(about = "Add a new item to the Brevium library")]
    Add,
    // #[clap(about = "Open an item in the Brevium library")]
    // Open,
}

fn main() {
    let config = CONFIG.read().expect("Should have config");
    let storage = FileStorage::new().unwrap();
    let cli = Cli::parse();

    match &cli.command {
        // Commands::Sync => {
        //     match program.storage.sync() {
        //         Ok(_) => info!("Successfully synced"),
        //         Err(e) => error!("Failed to sync: {}", e),
        //     };
        // }
        // Commands::Setup => match setup() {
        //     Ok(_) => println!("Updated config with new values"),
        //     Err(e) => error!("{}", e),
        // },
        Commands::Add => match add(&storage) {
            Ok(_) => println!("Added new entry"),
            Err(e) => panic!("{}", e),
        },
        _ => todo!()
        // Commands::Open => match open(program.storage.clone()) {
        //     Ok(_) => println!("Opened a new entry"),
        //     Err(e) => error!("{}", e),
        // },
    }
}

fn add(storage: &impl Storage) -> Result<(), Box<dyn std::error::Error>> {
    let source = TextItem {
        name: "Test",
        tags: hashmap! {
            "something" => "something"
        },
    };

    storage.add_item(&source)
}
