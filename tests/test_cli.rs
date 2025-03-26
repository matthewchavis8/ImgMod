#[cfg(test)] 
mod tests {
    use assert_cmd::Command;
    use serial_test::serial;

    fn remove_test_chunks() {
        let mut cmd = Command::cargo_bin("pngMessages")
            .unwrap();
        cmd.args(&["remove", "./test_image.png", "maTt"]);
        cmd.assert().success();
    }
    
    #[test]
    #[serial]
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
        remove_test_chunks();
    }

    #[test]
    #[serial]
    fn test_encode() {

        let encode_cli = |()| {
            let mut cmd = Command::cargo_bin("pngMessages")
            .unwrap();
            cmd.args(&["encode", "./test_image.png", "maTt", "Hello Matt!"]);   
            cmd.assert().success();
        };
        
        let print_cli = |()| {
            let mut cmd = Command::cargo_bin("pngMessages")
            .unwrap();
            cmd.args(&["print", "./test_image.png"]);

            cmd.assert()
            .stdout(
  "File: ./test_image.png, Size: 66227
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
            cmd.assert().success();
        };
        encode_cli(());
        print_cli(());

        remove_test_chunks();
    }

    #[test]
    #[serial]
    fn test_decode() {
        let encode_cli = |()| {
            let mut cmd = Command::cargo_bin("pngMessages")
            .unwrap();
            cmd.args(&["encode", "./test_image.png", "maTt", "Hello Matt!"]);   
            cmd.assert().success();
        };

        let decode_cli = |()| {
            let mut cmd = Command::cargo_bin("pngMessages")
            .unwrap();
            cmd.args(&["decode", "./test_image.png", "maTt"]);   
            
            cmd.assert()
                .stdout("msg: Hello Matt!\n");
            cmd.assert().success();
        };
        
        encode_cli(());
        decode_cli(());

        remove_test_chunks();
    }
    
    #[test]
    #[serial]
    fn test_remove() {
        let encode_cli = |()| {
            let mut cmd = Command::cargo_bin("pngMessages")
            .unwrap();
            cmd.args(&["encode", "./test_image.png", "maTt", "Hello Matt!"]);   
            cmd.assert().success();
        };

        let remove_cli = |()| {
            let mut cmd = Command::cargo_bin("pngMessages")
            .unwrap();
            cmd.args(&["remove", "./test_image.png", "maTt"]);   
            cmd.assert().success();
        };

        let print_cli = |()| {
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
        };

        encode_cli(());
        remove_cli(());
        print_cli(());
        remove_test_chunks();
    }
}
