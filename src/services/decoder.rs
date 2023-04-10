use codec2::*;
use std::io::prelude::*;
use zerocopy::AsBytes;

pub fn decode_audio() -> Result<(), String> {
    let mut filein = std::fs::File::open(format!("./src/audio/encoded.mp3"))
        .expect("unknown file to be decoded");
    let mut fileout = std::fs::File::open(format!("./src/audio/decoded.mp3"))
        .expect("couldn't create file for decoded audio");
    let mut c2 = Codec2::new(Codec2Mode::MODE_3200);
    let mut samps = vec![0; c2.samples_per_frame()];
    let mut packed = vec![0; (c2.bits_per_frame() + 7) / 8];
    // Decodes the bytes and converts back to audio
    while filein.read_exact(&mut packed).is_ok() {
            c2.decode(&mut samps[..], &packed);
            fileout.write_all(samps.as_bytes())
                .expect("couldn't write to decoded file");
    }
    Ok(())
}
