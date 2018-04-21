extern crate hound;
extern crate clap; use clap::{App, Arg};
use std::path::{Path, PathBuf}; use std::borrow::Cow;
use std::ffi::OsStr;
use std::time::Instant;

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
    let target_peak = matches.value_of("peak").map(|x| x.parse::<f32>().expect("peak option requires floating number(default = -6.0dB)"))
        .unwrap_or(-6.0);
    let target_mag = f32::powf(10.0, target_peak / 20.0);
    
    println!("Processing: {} ---> {}", infile.display(), outfile.display());
    let reader = hound::WavReader::open(infile).expect("Couldn't open input file");
    let spec = hound::WavSpec
    {
        sample_rate: 44100, bits_per_sample: 16, sample_format: hound::SampleFormat::Int,
        .. reader.spec()
    };
    let mut writer = hound::WavWriter::create(outfile, spec).expect("Couldn't open output file");
    let wv: Vec<f32> = reader.into_samples().collect::<Result<_, _>>().unwrap();
    let peak = wv.iter().fold(0.0, |a, &x| f32::abs(x).max(a));
    println!("peak: {} dB", 20.0 * peak.log10());
    let (start, mut last_print) = (Instant::now(), Instant::now());
    for (i, w) in wv.iter().map(|&x| target_mag * x / peak).enumerate()
    {
        if last_print.elapsed() >= std::time::Duration::from_secs(1)
        {
            println!("- Writing... {} sample", i); last_print = Instant::now();
        }
        writer.write_sample((w * 32767.0) as i16).unwrap();
    }
    writer.finalize().unwrap();
    let e = start.elapsed();
    println!("Process Completed! elapsed {} ms", (e.subsec_nanos() as u64 / 1_000_000) + e.as_secs() * 1000);
}
