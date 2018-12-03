use failure::Error;
use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;
use std::str::FromStr;

/// Extract numbers from string.
///
/// # Examples
///
/// ```
/// # use failure::Error;
/// # fn main() -> Result<(), Error> {
/// use aoc2018::input::get_numbers;
/// assert_eq!(get_numbers::<i32>("<1, 3*-4>")?, vec![1, 3, -4]);
/// # Ok(())
/// # }
/// ```
pub fn get_numbers<N: Integer + FromStr>(input: &str) -> Result<Vec<N>, Error> {
    lazy_static! {
        static ref Number: Regex = Regex::new(r"-?\d+").unwrap();
    }

    Number
        .find_iter(input)
        .map(|number| number.as_str().parse())
        .collect::<Result<Vec<N>, _>>()
        .map_err(|_| failure::err_msg(format!("Could not convert numbers in {}", input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() -> Result<(), Error> {
        assert_eq!(get_numbers::<i32>("1")?, vec![1]);
        assert_eq!(get_numbers::<i32>("-1")?, vec![-1]);
        assert_eq!(get_numbers::<i32>("23")?, vec![23]);
        assert_eq!(get_numbers::<i32>("-23")?, vec![-23]);
        assert_eq!(get_numbers::<i32>("0")?, vec![0]);

        Ok(())
    }

    #[test]
    fn test_space_separated_numbers() -> Result<(), Error> {
        assert_eq!(get_numbers::<i32>("  1 2 3  ")?, vec![1, 2, 3]);
        assert_eq!(get_numbers::<i32>("-1 32 7")?, vec![-1, 32, 7]);

        Ok(())
    }

    #[test]
    fn test_many_mixed_characters() -> Result<(), Error> {
        assert_eq!(
            get_numbers::<i32>(
                "sdaf1-asdfäö©@£$∞§|[]≈2sdafÖköoi
         3dsifguyhijksöldfmb,"
            )?,
            vec![1, 2, 3]
        );
        assert_eq!(get_numbers::<i32>("--1,--32,--7")?, vec![-1, -32, -7]);

        Ok(())
    }

    #[test]
    fn test_failure() {
        assert!(get_numbers::<u8>("1000").is_err());
        assert!(get_numbers::<usize>("-1").is_err());
    }
}
