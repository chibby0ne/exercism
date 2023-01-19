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
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
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
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    if from_base == to_base {
        return Ok(number.to_vec());
    }

    // Convert value to decimal
    let mut val_in_dec = 0;
    for (index, &digit) in number.iter().rev().enumerate() {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        val_in_dec += digit * from_base.pow(index as u32)
    }

    // Convert value in decimal to to_base digits inserting them to the front (index 0) of the vector
    let mut ans: Vec<u32> = Vec::new();
    while val_in_dec >= to_base {
        let remainder = val_in_dec % to_base;
        ans.insert(0, remainder);
        val_in_dec /= to_base;
    }
    // Remember to insert the quotient (since it is the technically last remainder of the integer
    // division)
    ans.insert(0, val_in_dec);

    Ok(ans)
}
