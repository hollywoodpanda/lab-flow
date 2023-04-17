#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("💬 {}", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("❌ {}", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        println!("✅ {}", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! working {
    ($($arg:tt)*) => {
        println!("👷 {}", format_args!($($arg)*))
    };
}