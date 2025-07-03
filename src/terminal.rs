pub fn printerr(msg: &str) {
    eprintln!("\x1b[91m\x1b[1merror\x1b[0m: {msg}")
}

pub fn printwarn(msg: &str) {
    eprintln!("\x1b[93m\x1b[warn\x1b[0m: {msg}")
}
