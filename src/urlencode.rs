use std::fmt::Write;

// This isn't awesome but it beats pulling in an entire crate.
// https://rosettacode.org/wiki/URL_encoding#Rust
pub fn urlencode(input: &str) -> String {
    const MAX_CHAR_VAL: u32 = std::char::MAX as u32;
    let mut buff = [0; 4];
    input
        .chars()
        .map(|ch| match ch as u32 {
            0..=47 | 58..=64 | 91..=96 | 123..=MAX_CHAR_VAL => {
                ch.encode_utf8(&mut buff);
                buff[0..ch.len_utf8()]
                    .iter()
                    .fold(String::new(), |mut output, byte| {
                        let _ = write!(output, "%{byte:X}");
                        output
                    })
            }
            _ => ch.to_string(),
        })
        .collect::<String>()
}
