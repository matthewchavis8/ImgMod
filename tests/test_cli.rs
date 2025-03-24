#[cfg(test)] 
mod tests {
    use assert_cmd::Command;
    
    #[test]
    fn test_print() {
        let mut cmd = Command::cargo_bin("pngMessages")
            .unwrap();
        cmd.args(&["print", "./test_image.png"]);
        cmd.assert()
            .stdout(
  "File: ./test_image.png, Size: 66204
  chunk#0{ chunk_type: IHDR, data_length: 13}
  chunk#1{ chunk_type: cHRM, data_length: 32}
  chunk#2{ chunk_type: PLTE, data_length: 765}
  chunk#3{ chunk_type: bKGD, data_length: 1}
  chunk#4{ chunk_type: tIME, data_length: 7}
  chunk#5{ chunk_type: IDAT, data_length: 32768}
  chunk#6{ chunk_type: IDAT, data_length: 32364}
  chunk#7{ chunk_type: tEXt, data_length: 37}
  chunk#8{ chunk_type: tEXt, data_length: 37}
  chunk#9{ chunk_type: tEXt, data_length: 40}
  chunk#10{ chunk_type: IEND, data_length: 0}\n"
        );
        cmd.assert().success();

    }

    #[test]
    fn test_encode() {}

    #[test]
    fn test_decode() {}
    
    #[test]
    fn test_remove() {}
}