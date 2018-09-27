pub mod calcs;
pub mod database;
pub mod initials;
pub mod retrievals;

pub enum Command {
    InitializePaths,
    Retrieve,
}
