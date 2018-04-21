extern crate hound;
extern crate clap; use clap::{App, Arg};
use std::path::{Path, PathBuf}; use std::borrow::Cow;
use std::ffi::OsStr;

fn main()
{
    let matches = App::new("CompilationWavFormer").version("1.0")
        .author("S.Percentage/JourneyCat <Syn.Tri.Naga@gmail.com>").about("Normalizes wav file and generates 16-bit 44.1kHz")
        .arg(Arg::with_name("output").short("o").long("out").value_name("FILE").help("Specifies an output filename").takes_value(true))
        .arg(Arg::with_name("input").required(true).index(1))
        .arg(Arg::with_name("peak").short("p").long("peak").help("Specifies the target peak(in dB)").takes_value(true).required(false))
        .get_matches();
    let infile = Path::new(matches.value_of("input").unwrap());
    let outfile = matches.value_of("output").map(Path::new).map(Cow::from)
        .unwrap_or_else(|| PathBuf::from(format!("{}_16_44100.wav", infile.file_stem().unwrap_or(OsStr::new("")).to_string_lossy())).into());
    let target_peak = matches.value_of("peak").map(|x| x.parse::<f32>().expect("peak option requires floating number"))
        .unwrap_or(0.0);
    let target_mag = f32::powf(10.0, target_peak / 20.0);
    
    let reader = hound::WavReader::open(infile).expect("Couldn't open input file");
    let spec = hound::WavSpec
    {
        sample_rate: 44100, bits_per_sample: 16, sample_format: hound::SampleFormat::Int,
        .. reader.spec()
    };
    let mut writer = hound::WavWriter::create(outfile, spec).expect("Couldn't open output file");
    let wv: Vec<f32> = reader.into_samples().collect::<Result<_, _>>().unwrap();
    let peak = wv.iter().fold(0.0, |a, &x| f32::abs(x).max(a));
    for w in wv.iter().map(|&x| target_mag * x / peak) { writer.write_sample((w * 32767.0) as i16).unwrap(); }
    writer.finalize().unwrap();
}
