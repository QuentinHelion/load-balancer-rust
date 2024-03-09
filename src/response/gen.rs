pub fn generator(code: &str, ct: &str,content: &str) -> String{
    format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\n\r\n{}",
        code, ct, content
    )
}