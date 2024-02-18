#![feature(result_flattening)]

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
