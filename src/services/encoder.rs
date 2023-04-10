use codec2::*;
use std::io::prelude::*;
use zerocopy::AsBytes;

pub fn encode_audio(filename: String) -> Result<(),String>{
    let encoded_audio = format!("./src/audio/encoded.mp3");
    let mut filein = std::fs::File::open(filename)
        .expect("unknown file to be encoded");
    let mut fileout = std::fs::File::open(encoded_audio)
        .expect("couldn't create file for encoded audio");
    let mut c2 = Codec2::new(Codec2Mode::MODE_3200);
    let mut samps = vec![0; c2.samples_per_frame()];
    let mut packed = vec![0; (c2.bits_per_frame() + 7) / 8];
    // Encodes the buffer and writes to file
    while filein.read_exact(samps.as_bytes_mut()).is_ok() {
            c2.encode(&mut packed, &samps[..]);
            fileout.write_all(&packed)
                .expect("couldn't write encoded buffer to file");
    }
    Ok(())
}
