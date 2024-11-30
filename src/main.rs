use clap::Parser;
use utils::{
    cli::Cli,
    terminal::{end_session, start_session},
};

mod hardware;
mod utils;

fn main() {
    let termios = start_session();

    let Cli { image_path } = Cli::parse();

    let mut vm = hardware::Vm::new();
    vm.load_image_from_file(image_path);
    vm.launch();

    end_session(termios);
}
