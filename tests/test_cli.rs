#[cfg(test)] 
mod tests {
    use std::path::PathBuf;

    use assert_cmd::Command;
    use serial_test::serial;
    use tempfile::tempdir;

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

        pub fn install_image_from_the_internet() {
            let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string();
        
            let mut cmd = Command::cargo_bin("ImgMod")
            .unwrap();
            cmd.args(&["manage", "download", &url, "test.png"]);
            cmd.assert().success();
        }
    }
    
    #[test]
    #[serial]
    fn test_print() {
        let assert = TestCli::print_chunks();
        assert.stdout(
  "File: ./images/test_image.png, Size: 231668
  chunk#0{ chunk_type: IHDR, data_length: 13}
  chunk#1{ chunk_type: pHYs, data_length: 9}
  chunk#2{ chunk_type: iTXt, data_length: 1891}
  chunk#3{ chunk_type: IDAT, data_length: 229687}
  chunk#4{ chunk_type: IEND, data_length: 0}\n"
        );
    }

    #[test]
    #[serial]
    fn test_encode() {

        let check_chunks_after_chunk_added = |()| {
            let assert = TestCli::print_chunks();
            assert.stdout(
  "File: ./images/test_image.png, Size: 231691
  chunk#0{ chunk_type: IHDR, data_length: 13}
  chunk#1{ chunk_type: pHYs, data_length: 9}
  chunk#2{ chunk_type: iTXt, data_length: 1891}
  chunk#3{ chunk_type: IDAT, data_length: 229687}
  chunk#4{ chunk_type: IEND, data_length: 0}
  chunk#5{ chunk_type: maTt, data_length: 11}\n"
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

    #[test]
    fn test_delete_file() {

        let dir = tempdir().expect("Error creating temporary directory");
        let file_path = dir.path().join("test.png");
    
        let mut cmd = Command::cargo_bin("ImgMod")
        .unwrap();
        cmd.args(&["manage", "delete", file_path.to_str().unwrap()]);
        cmd.assert().success();

        assert!(!file_path.exists());
    }

    #[test]
    #[serial]
    fn test_install_image_from_internet() {
        TestCli::install_image_from_the_internet();
        
        let mut cmd = Command::cargo_bin("ImgMod")
        .unwrap();
        cmd.args(&["manage", "delete", "images/test.png"]);
        cmd.assert().success();

        let check_path = PathBuf::from("images/test.png");
        assert!(!check_path.exists());
    }

    #[test]
    #[serial]
    fn test_convert_image() {
        TestCli::install_image_from_the_internet();
        
        let mut cmd = Command::cargo_bin("ImgMod")
        .unwrap();
        cmd.args(&["manage", "convert", "-j", "images/test.png"]);
        cmd.assert().success();

        let check_path = PathBuf::from("images/test.jpeg");
        assert!(check_path.exists());

        let mut cmd = Command::cargo_bin("ImgMod")
        .unwrap();
        cmd.args(&["manage", "delete", "images/test.jpeg"]);
        cmd.assert().success();

        assert!(!check_path.exists());

    }
}
