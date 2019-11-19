mod encoding;

use hound;
use crate::encoding::*;
use std::path::PathBuf;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name="text_to_barcodeaudio")]
struct Opt {
    /// The text to encode
    #[structopt(short, long)]
    text: String,

    /// Outputfile
    #[structopt(short, long, parse(from_os_str), default_value = "out.wav")]
    output: PathBuf
}



fn main() {
    // get cli arguments
    let opt = Opt::from_args();
    // convert into a vec of uppercase chars
    let input_list: Vec<char> = opt.text.to_uppercase().chars().collect();

    // specify the wav file
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // crete the writer to write the wav file to
    let mut writer = hound::WavWriter::create(opt.output, spec).unwrap();

    for t in input_list
        .iter()
        // encode each character into the code_39 representation
        .map(|x| encode(x))
        .flatten()
        // generate the sound samples
        .map(|x| generate_samples(&x, spec.sample_rate as usize))
        .flatten()
        .collect::<Vec<i16>>()
    {
        // write to the output buffer
        writer.write_sample(t).unwrap();
    }
}
