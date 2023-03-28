pub mod config;
pub mod item;
pub mod logger;
pub mod storage;

pub fn get_dir() -> directories_next::ProjectDirs {
    directories_next::ProjectDirs::from("rs", "", "brevium")
        .expect("Cannot initialzie Brevium directories")
}
