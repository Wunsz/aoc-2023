use std::fmt::Debug;
use std::str::FromStr;

pub fn read_list_of_numbers<T: FromStr>(string: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug {
    return string.split_whitespace().map(FromStr::from_str).map(Result::unwrap).collect();
}
