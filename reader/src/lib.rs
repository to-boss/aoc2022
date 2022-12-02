use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_hello_world() {
        let content = read_file("test.txt");
        assert_eq!("hello\r\nworld", content.unwrap());
    }
}
