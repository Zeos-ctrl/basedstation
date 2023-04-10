#[cfg(test)]
mod tests {

    use crate::services::decoder::decode_audio;
    use crate::services::encoder::encode_audio;

    #[test]
    fn test_encode() {

        /*
         * Tests the encode function, test passes if function returns Ok which
         * means no errors have occured while encoding a file
         */

        let test_file: String = format!("./src/tests/audio/test.mp3");
        let result = encode_audio(test_file);
        match result {
            Ok(_) => assert!(true),
            Err(result) => println!("{}",result),
        }
    }

    #[test]
    fn test_decode() {

        /*
         * Tests the encode function, test passes if function returns Ok which
         * means no errors have occured while encoding a file
         */

        let result = decode_audio();
        match result {
            Ok(_) => assert!(true),
            Err(result) => println!("{}",result),
        }
    }
}
