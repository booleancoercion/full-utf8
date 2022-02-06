#![no_std]

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OverflowError;

/// The type representing a byte encoding. Since byte sequences
/// are at most 6 bytes long, it lives on the stack and requires no allocations.
pub type Codepoint = tinyvec::ArrayVec<[u8; 6]>;

/// Encodes the input as a UTF-8 byte sequence of length ranging from 1 to 6.
///
/// This function does no validation other than a size check, and as such
/// should not be used without additional checks to act as an
/// implementation of [RFC3629].
///
/// ## Errors
/// When the input has its 32nd (as in, most significant) bit set, an overflow error is returned.
///
/// ## Example
/// ```rust
/// # fn main() {
/// let not_equal = 0x2260; // U+2260 corresponds to `≠`
/// let output = utf8_rfc2279::encode_u32(not_equal).unwrap();
///
/// assert_eq!(&output[..], &[0xE2, 0x89, 0xA0]);
/// # }
/// ```
///
/// [RFC3629]: https://datatracker.ietf.org/doc/html/rfc3629
pub fn encode_u32(mut input: u32) -> Result<Codepoint, OverflowError> {
    let (length, header) = get_header(input)?;

    let mut output = Codepoint::new();
    output.set_len(length);

    for i in (1..length).rev() {
        output[i] = 0b1000_0000 | (input & 0b0011_1111) as u8;
        input >>= 6;
    }
    output[0] = header | input as u8; // by now, input fits in the remaining space

    // note that this works for ASCII inputs too,
    // because the loop is skipped and the header is 0
    Ok(output)
}

/// Helper function that returns the 'header' byte of the input's corresponding byte
/// sequence, along with the corresponding length (total number of bytes in the sequence).
///
/// The placeholder bytes in the header are replaced with zeros, which can be conveniently
/// changed using bitwise OR.
///
/// ## Errors
/// When the input has its 32nd (as in, most significant) bit set, an overflow error is returned.
///
/// ## Example
/// ```rust
/// # fn main() {
/// let not_equal = 0x2260; // U+2260 corresponds to `≠`
/// let (length, header) = utf8_rfc2279::get_header(not_equal).unwrap();
///
/// assert_eq!(length, 3);
/// assert_eq!(header, 0b11100000);
/// # }
/// ```
pub fn get_header(input: u32) -> Result<(usize, u8), OverflowError> {
    Ok(match input {
        0x00000000..=0x0000007F => (1, 0b00000000),
        0x00000080..=0x000007FF => (2, 0b11000000),
        0x00000800..=0x0000FFFF => (3, 0b11100000),
        0x00010000..=0x001FFFFF => (4, 0b11110000),
        0x00200000..=0x03FFFFFF => (5, 0b11111000),
        0x04000000..=0x7FFFFFFF => (6, 0b11111100),
        _ => return Err(OverflowError),
    })
}

#[cfg(test)]
mod tests {
    use crate::{encode_u32, get_header, Codepoint};

    #[test]
    fn header_works() {
        let tests = [
            (0xa9, 0b11000000),
            (0x2260, 0b11100000),
            (b'a' as u32, 0b00000000),
            (b'\0' as u32, 0b00000000),
            (0x001FCAFE, 0b11110000),
            (0x03FCAFEF, 0b11111000),
            (0x7DEADA55, 0b11111100),
        ];

        for (input, output) in tests {
            assert_eq!(get_header(input).unwrap().1, output)
        }
    }

    #[test]
    fn encoding_works() {
        #[rustfmt::skip]
        let tests = [
            (0xa9,          Codepoint::from_array_len([0xc2, 0xa9, 0x00, 0x00, 0x00, 0x00], 2)),
            (0x2260,        Codepoint::from_array_len([0xe2, 0x89, 0xa0, 0x00, 0x00, 0x00], 3)),
            (b'a' as u32,   Codepoint::from_array_len([b'a', 0x00, 0x00, 0x00, 0x00, 0x00], 1)),
            (b'\0' as u32,  Codepoint::from_array_len([0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 1)),
            (0x001FCAFE,    Codepoint::from_array_len([0xf7, 0xbc, 0xab, 0xbe, 0x00, 0x00], 4)),
            (0x03FCAFEF,    Codepoint::from_array_len([0xfb, 0xbf, 0x8a, 0xbf, 0xaf, 0x00], 5)),
            (0x7DEADA55,    Codepoint::from_array_len([0xfd, 0xbd, 0xba, 0xad, 0xa9, 0x95], 6)),
        ];

        for (input, output) in tests {
            assert_eq!(encode_u32(input).unwrap(), output)
        }
    }
}
