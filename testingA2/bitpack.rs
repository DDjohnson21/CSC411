use std::convert::TryInto;

/// Returns true iff the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    if width == 0 { return false; } // Edge case: no space to fit any number
    let max_positive: i64 = (1 << (width - 1)) - 1;
    let min_negative: i64 = -1 << (width - 1);
    n >= min_negative && n <= max_positive
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An unsigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    width != 0 && n < (1u64 << width)
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// Returns `None` if `width+lsb>64` or `width` is 0.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> Option<i64> {
    if width == 0 || width > 64 || lsb + width > 64 {
        return None;
    }
    
    // Calculate the mask for the value; special case when width is 64
    let mask = if width < 64 { (1 << width) - 1 } else { u64::MAX };
    
    // Extract the value
    let value = (word >> lsb) & mask;
    
    // If the width is less than 64, we need to sign-extend manually
    if width < 64 {
        // Determine if the sign bit is set
        let sign_bit = 1 << (width - 1);
        
        if value & sign_bit != 0 {
            // Sign bit is set; extend the sign to the full i64 width
            // Shift left to clear all bits except the sign, then arithmetic shift right
            // to extend the sign, taking advantage of i64's sign extension
            return Some(((value | (!mask)) as i64).wrapping_shl(64 - width as u32).wrapping_shr(64 - width as u32));
        }
    }
    
    // No sign extension needed, or value is positive
    Some(value as i64)
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// Returns `None` if `width+lsb>=64`
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> Option<u64> {
    if lsb + width >= 64 || width == 0 {
        None
    } else {
        let mask = if width == 64 { !0 } else { (1 << width) - 1 };
        Some((word >> lsb) & mask)
    }

}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if !fitsu(value, width) || lsb + width > 64 {
        None
    } else {
        let mask = ((1 << width) - 1) << lsb;
        Some((word & !mask) | ((value << lsb) & mask))
    }
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if !fitss(value, width) || lsb + width > 64 {
        None
    } else {
        let mask = ((1 << width) - 1) << lsb;
        let value_masked = (value as u64) & ((1 << width) - 1);
        Some((word & !mask) | ((value_masked << lsb) & mask))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
