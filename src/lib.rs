use std::sync::atomic::AtomicBool;

static mut FLAG: AtomicBool = AtomicBool::new(true);

/// the flag is by default `true`
///
/// # Example
/// ```
/// // if you need to set it to depend from a flag
/// let from_cli = false;
/// iro::set_flag(from_cli);
///
/// assert!(!iro::get_flag())
/// ```
pub fn set_flag(flag: bool) {
    unsafe { FLAG = AtomicBool::from(flag) }
}

/// used for debuging [`set_flag`]
pub fn get_flag() -> bool {
    unsafe { FLAG.load(std::sync::atomic::Ordering::Relaxed) }
}

/// # Example
/// ```
///let (w_1, w_2) = (3, 2);
///        assert_eq!(
///            iro::iformat!(
///                "Round <y>{}</> <b>p_1</> has <y>{}</> wins <r>p_2</> <y>{}</> wins",
///                w_1,
///                w_2,
///                w_1 + w_2 + 1
///            ),
///            "Round \u{1b}[33m3\u{1b}[39m \u{1b}[34mp_1\u{1b}[39m has \u{1b}[33m2\u{1b}[39m wins \u{1b}[31mp_2\u{1b}[39m \u{1b}[33m6\u{1b}[39m wins"
///        )
/// ```
#[macro_export]
macro_rules! iformat {
    ($f:literal, $($arg:expr),*) => {
        if $crate::get_flag() {
            color_print::cformat!($f, $($arg),* )
        } else {
            format!(color_print::untagged!($f), $($arg),*)
        }
    };
}

/// see the `iformat` macro for usage
#[macro_export]
macro_rules! iprint {
    ($f:literal, $($arg:expr),*) => {
        print!("{}", iro_format!($f, $($arg),*))
    };
}

/// see the `iformat` macro for usage
#[macro_export]
macro_rules! iprintln {
    ($f:literal, $($arg:expr),*) => {
        println!("{}", iro_format!($f, $($arg),*))
    };
}
