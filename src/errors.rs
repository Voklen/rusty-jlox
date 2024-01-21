#[macro_export]
macro_rules! error {
    ($($message:tt)*) => {{
        let res = format!($($message)*);
        println!("Error: {res}");
    }}
}
