use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Number of bits to build the waveform
   #[arg(short, long, default_value_t = 4)]
   quntbits: usize,
   
   /// Number of samples for one period of wave
   #[arg(short, long, default_value_t = 16 )]
   sample_per_wave: usize,
   
   /// Repeatition count of output
   #[arg(short, long, default_value_t = 1 )]
   repeat: usize,
}


use core::f32::consts::PI;

fn sin_gen(samples_per_wave: usize, quantbits: usize) -> impl Iterator<Item = usize> {
    let maxval: usize = (1 << quantbits) - 1;
    (0..samples_per_wave)
        .map(move |x| x as f32 / (samples_per_wave as f32) * 2.0 * PI)
        .map(|x| x.sin())
        .map(move |x| ((x + 1.0) / 2.0 * (maxval as f32)) as usize)
        .cycle()
}

fn usize_to_bits(value: usize, quantbits: usize) -> impl Iterator<Item = bool> {
    (0..value)
        .map(|_| true)
        .chain((value..(1 << quantbits)).map(|_| false))
        .into_iter()
}


fn print_bitstream(i: impl Iterator<Item = bool>) {
    i.for_each(|i| if i { print!("1") } else { print!("0") });
    println!();
}

fn main() {
    let args = Args::parse();

    sin_gen(args.sample_per_wave, args.quntbits).take(args.sample_per_wave * args.repeat).for_each(|x| {
        print!("{x:4}: ");
        print_bitstream(usize_to_bits(x, args.quntbits));
    })
}
