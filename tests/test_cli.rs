#[cfg(test)] 
mod tests {
    use assert_cmd::Command;
    use serial_test::serial;

    struct TestCli;

    impl TestCli {
        pub fn remove_chunks() {
            let mut cmd = Command::cargo_bin("ImgMod")
                .unwrap();
            cmd.args(&["remove", "./images/test_image.png", "maTt"]);
            cmd.assert().success();
        }

        pub fn print_chunks() -> assert_cmd::assert::Assert {
            let mut cmd = Command::cargo_bin("ImgMod")
                .unwrap();
            cmd.args(&["print", "./images/test_image.png"]);
            cmd.assert()
        }

        pub fn encode() {
            let mut cmd = Command::cargo_bin("ImgMod")
            .unwrap();
            cmd.args(&["encode", "./images/test_image.png", "maTt", "Hello Matt!"]);   
            cmd.assert().success();
        }

        pub fn decode() {
            let mut cmd = Command::cargo_bin("ImgMod")
            .unwrap();
            cmd.args(&["decode", "./images/test_image.png", "maTt"]);   
            
            cmd.assert()
                .stdout("msg: Hello Matt!\n");
            cmd.assert().success();
        }
    }
    
    #[test]
    #[serial]
    fn test_print() {
        let assert = TestCli::print_chunks();
        assert.stdout(
  "File: ./images/test_image.png, Size: 66204
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
    }

    #[test]
    #[serial]
    fn test_encode() {

        let check_chunks_after_chunk_added = |()| {
            let assert = TestCli::print_chunks();
            assert.stdout(
  "File: ./images/test_image.png, Size: 66227
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
  chunk#10{ chunk_type: IEND, data_length: 0}
  chunk#11{ chunk_type: maTt, data_length: 11}\n"
            );
        };
        TestCli::encode();
        check_chunks_after_chunk_added(());
        TestCli::remove_chunks();
    }

    #[test]
    #[serial]
    fn test_decode() {
        TestCli::encode();
        TestCli::decode();
        TestCli::remove_chunks();
    }
    
    #[test]
    #[serial]
    fn test_remove() {
        TestCli::encode();
        TestCli::remove_chunks();
        TestCli::print_chunks();
        TestCli::remove_chunks();
    }
}
