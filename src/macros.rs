macro_rules! print_stdout {
    ($($arg:tt)*) => {
        print!($($arg)*);
        let _ = std::io::stdout().flush();
    };
}

pub(crate) use print_stdout;
