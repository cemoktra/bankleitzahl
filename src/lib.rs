mod data;

pub use data::Blz;


#[cfg(test)]
mod tests {
    #[test]
    fn from_blz() {
        let result = super::Blz::from_blz("10000000");
        assert_eq!(1, result.len());
        assert_eq!("Bundesbank", result[0].name());
        assert_eq!("Berlin", result[0].city());
        assert_eq!("10591", result[0].post_code());
        assert_eq!(Some("MARKDEF1100"), result[0].bic());
        assert!(!result[0].is_deleted());
        assert_eq!(None, result[0].new_blz());

        let result = super::Blz::from_blz("10090000");
        assert_eq!(19, result.len());
    }

    #[test]
    fn from_bic() {
        let result = super::Blz::from_bic("MARKDEF1100");
        assert_eq!(1, result.len());
        assert_eq!("Bundesbank", result[0].name());
        assert_eq!("Berlin", result[0].city());
        assert_eq!("10591", result[0].post_code());
        assert_eq!("10000000", result[0].blz());
        assert!(!result[0].is_deleted());
        assert_eq!(None, result[0].new_blz());
    }
}