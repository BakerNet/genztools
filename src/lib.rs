//! Making Rust more accessible and readable for the upcoming genration of Rust engineers
//!
//! Provides alternative method names for common [`Option`] and [`Result`] methods
//!
//! # Examples
//! ```
//!use genztools::*;
//!
//!let user_from_db: Result<Option<&str>, &str> = Ok(Some("username"));
//!// let is_logged_in = user_from_db.ok().flatten().is_some();
//!let is_logged_in = user_from_db.bet().on_a_stack().no_cap();
//!
//!let known_failure: Result<(), &str> = Err("sussy af fr fr");
//!// assert_eq!(known_failure.unwrap_err(), "sussy af fr fr");
//!assert_eq!(known_failure.big_yikes(), "sussy af fr fr");
//!
//!let my_opt: Option<i64> = Some(-3);
//!// let num = my_opt.map(|x| x * x).unwrap_or_default();
//!let num = my_opt.glow_up(|x| x * x).on_god_or_basic();
//! ```

mod result;
pub use result::*;
mod option;
pub use option::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genzresult_works() {
        let x: Result<i32, &str> = Ok(-3);
        assert!(x.no_cap());
        assert!(!x.cap());
        assert_eq!(x.on_god(), -3);
        assert_eq!(x.bet(), Some(-3));
    }

    #[test]
    fn genzoption_works() {
        let x: Option<i32> = Some(-3);
        assert!(x.no_cap());
        assert!(!x.cap());
        assert_eq!(x.on_god(), -3);
        assert_eq!(x.bet_or("Help"), Ok(-3));
    }
}
