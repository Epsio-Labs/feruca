use std::cmp::Ordering;

use crate::weights::{primary, secondary, tertiary, variability};

pub fn compare_incremental(a_cea: &[u32], b_cea: &[u32], shifting: bool) -> Ordering {
    if shifting {
        if let Some(o) = compare_primary_shifting(a_cea, b_cea) {
            return o;
        }
    } else if let Some(o) = compare_primary(a_cea, b_cea) {
        return o;
    }

    if let Some(o) = compare_secondary(a_cea, b_cea) {
        return o;
    }

    if let Some(o) = compare_tertiary(a_cea, b_cea) {
        return o;
    }

    // If not shifting, stop here
    if !shifting {
        return Ordering::Equal;
    }

    if let Some(o) = compare_quaternary(a_cea, b_cea) {
        return o;
    }

    // If we got to this point, return Equal. The efficiency of processing and comparing sort keys
    // incrementally, for both strings at once, relies on the rarity of needing to continue all the
    // way through tertiary or quaternary weights. (Remember, there are two earlier fast paths for
    // equal strings -- one before normalization, one after.)
    Ordering::Equal
}


pub fn get_key(cea: &[u32], shifting: bool) -> Vec<u16> {
    let mut key = {
        if shifting {
            get_primary_shifting(cea)
        } else {
            get_primary(cea)
        }
    };

    key.extend(get_secondary(cea));
    key.extend(get_tertiary(cea));
    key.extend(get_quaternary(cea));
    key
}


fn compare_primary(a_cea: &[u32], b_cea: &[u32]) -> Option<Ordering> {
    let mut a_filter = a_cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .map(|w| primary(*w))
        .filter(|p| *p != 0);

    let mut b_filter = b_cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .map(|w| primary(*w))
        .filter(|p| *p != 0);

    loop {
        let a_p = a_filter.next().unwrap_or_default();
        let b_p = b_filter.next().unwrap_or_default();

        if a_p != b_p {
            return Some(a_p.cmp(&b_p));
        }

        if a_p == 0 {
            return None;
        }
    }
}


fn get_primary(cea: &[u32]) -> Vec<u16> {
    cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .map(|w| primary(*w))
        .filter(|p| *p != 0)
        .collect()
}



fn compare_primary_shifting(a_cea: &[u32], b_cea: &[u32]) -> Option<Ordering> {
    let mut a_filter = a_cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .filter(|w| !variability(**w))
        .map(|w| primary(*w))
        .filter(|p| *p != 0);

    let mut b_filter = b_cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .filter(|w| !variability(**w))
        .map(|w| primary(*w))
        .filter(|p| *p != 0);

    loop {
        let a_p = a_filter.next().unwrap_or_default();
        let b_p = b_filter.next().unwrap_or_default();

        if a_p != b_p {
            return Some(a_p.cmp(&b_p));
        }

        if a_p == 0 {
            return None;
        }
    }
}

fn get_primary_shifting(cea: &[u32]) -> Vec<u16> {
    cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .filter(|w| !variability(**w))
        .map(|w| primary(*w))
        .filter(|p| *p != 0)
        .collect()
}

fn compare_secondary(a_cea: &[u32], b_cea: &[u32]) -> Option<Ordering> {
    let mut a_filter = a_cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .map(|w| secondary(*w))
        .filter(|s| *s != 0);

    let mut b_filter = b_cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .map(|w| secondary(*w))
        .filter(|s| *s != 0);

    loop {
        let a_s = a_filter.next().unwrap_or_default();
        let b_s = b_filter.next().unwrap_or_default();

        if a_s != b_s {
            return Some(a_s.cmp(&b_s));
        }

        if a_s == 0 {
            return None;
        }
    }
}

fn get_secondary(cea: &[u32]) -> Vec<u16> {
    cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .map(|w| secondary(*w))
        .filter(|s| *s != 0)
        .collect()
}


fn compare_tertiary(a_cea: &[u32], b_cea: &[u32]) -> Option<Ordering> {
    let mut a_filter = a_cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .map(|w| tertiary(*w))
        .filter(|t| *t != 0);

    let mut b_filter = b_cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .map(|w| tertiary(*w))
        .filter(|t| *t != 0);

    loop {
        let a_t = a_filter.next().unwrap_or_default();
        let b_t = b_filter.next().unwrap_or_default();

        if a_t != b_t {
            return Some(a_t.cmp(&b_t));
        }

        if a_t == 0 {
            return None;
        }
    }
}

fn get_tertiary(cea: &[u32]) -> Vec<u16> {
    cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .map(|w| tertiary(*w))
        .filter(|t| *t != 0)
        .collect()
}


fn compare_quaternary(a_cea: &[u32], b_cea: &[u32]) -> Option<Ordering> {
    let mut a_filter = a_cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .filter(|w| variability(**w) || secondary(**w) != 0)
        .map(|w| primary(*w));

    let mut b_filter = b_cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .filter(|w| variability(**w) || secondary(**w) != 0)
        .map(|w| primary(*w));

    loop {
        let a_p = a_filter.next().unwrap_or_default();
        let b_p = b_filter.next().unwrap_or_default();

        if a_p != b_p {
            return Some(a_p.cmp(&b_p));
        }

        if a_p == 0 {
            return None;
        }
    }
}


fn get_quaternary(cea: &[u32]) -> Vec<u16> {
    cea
        .iter()
        .take_while(|x| **x < std::u32::MAX)
        .filter(|w| variability(**w) || secondary(**w) != 0)
        .map(|w| primary(*w))
        .collect()
}