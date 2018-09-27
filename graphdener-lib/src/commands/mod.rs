pub mod initials;
pub mod retrievals;
pub mod calcs;
pub mod database;

use rmp_rpc::Value;

pub enum Command
{
	InitializePaths,
	Retrieve
}

// fn process_message(msg: Command) {
//     match msg {
//         Command::InitializePaths => initials::import_paths(),
//         Command::Retrieve => change_color(r, g, b),
//     };
// }

