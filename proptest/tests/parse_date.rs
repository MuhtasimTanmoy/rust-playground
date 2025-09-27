use proptest::prelude::*;
use date_parser::parse_date;

proptest! {
    #[test]
    fn parse_valid_date_roundtrip(y in 1900u32..2100, m in 1u32..13, d in 1u32..32) {
        // Format into YYYY-MM-DD
        let s = format!("{:04}-{:02}-{:02}", y, m, d);

        // println!("Testing with date string: {}", s);

        // Parse back
        let parsed = parse_date(&s);

        // It must succeed and equal the original numbers
        prop_assert_eq!(parsed, Some((y, m, d)));
    }

     #[test]
    fn parse_valid_date_roundtripStr(s in "[0-9]{4}-[0-9]{2}-[0-9]{2}") {

        println!("Testing with date string: {}", s);

        // Parse back
        let parsed = parse_date(&s);

        match parsed {
            Some((y, m, d)) => {
                // If parse_date succeeds, it should match the string contents
                let reconstructed = format!("{:04}-{:02}-{:02}", y, m, d);
                prop_assert_eq!(s, reconstructed);
            }
            None => {
                // If parse_date fails, that's fine: the regex can still generate invalid dates
                // e.g. "2025-99-40" → structurally valid but semantically bogus
                prop_assert!(true);
            }
        }
    }

    #[test]
    fn reject_non_ascii(input in "\\PC*") {
        // If string is non-ascii or wrong length, parser must not crash
        let _ = parse_date(&input);
    }
}
