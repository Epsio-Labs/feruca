use feruca::{Collator, Locale, Tailoring};
use std::cmp::Ordering;


#[test]
fn test_sort_keys() {
        let str_1 = "Vision-oriented dedicated paradigm";
        let str_2 = "Visionary explicit infrastructure";
        let mut c = Collator::default();

        assert!(c.sort_key(str_1) > c.sort_key(str_2));
    }