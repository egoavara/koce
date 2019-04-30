
pub fn hex_format
<R: AsRef<[u8]>>(raw:R, sep : &str) -> String{
    raw.as_ref().iter().fold(String::new(), |mut res, d|{
        res += format!("0x{:02X}{}", d, sep).as_str();
        res
    }).trim_end_matches(sep).to_string()
}


