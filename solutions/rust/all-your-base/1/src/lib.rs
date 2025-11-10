#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    for i in number {
        if *i >= from_base {
            return Err(Error::InvalidDigit(*i));
        }
    }
    let decimal = base_to_decimal(number, from_base);
    Ok (
        if decimal == 0 {
            vec!(0)
        } else {
            decimal_to_base(decimal, to_base)
        }
    )
}

fn base_to_decimal(nums: &[u32], base: u32) -> u32 {
    let mut d = 0;
    for i in nums {
        d *= base;
        d += *i;
    }
    d
}

fn decimal_to_base(decimal: u32, base: u32) -> Vec<u32> {
    let mut v = vec!();
    let mut decimal = decimal;
    while decimal > 0 {
        v.push(decimal%base);
        decimal = decimal/base;
    }
    v.reverse();
    v
}
