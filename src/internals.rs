/// A macro that only prints out the given
/// expression if it is compiled in debug
/// mode
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($x:expr) => { dbg!($x) }
}


#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($x:expr) => { std::convert::identity(true) }
}