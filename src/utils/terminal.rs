use termios::*;

const STD_IN: i32 = 0;

// Some tricks to make the VM's terminal be interactive
pub fn start_session() -> Termios {
    let termios = Termios::from_fd(STD_IN).unwrap();

    // make a mutable copy of termios
    // that we will modify
    let mut new_termios = termios.clone();
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode

    tcsetattr(STD_IN, TCSANOW, &mut new_termios).unwrap();

    new_termios
}

pub fn end_session(termios: Termios) {
    // reset the stdin to
    // original termios data
    tcsetattr(STD_IN, TCSANOW, &termios).unwrap();
}
