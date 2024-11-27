mod hardware;

fn main() {
    let mut vm = hardware::Vm::new();
    vm.launch();
}
