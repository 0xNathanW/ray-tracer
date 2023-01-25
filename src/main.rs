use clap::Parser;
use ray_tracer::OutputFormat;

#[derive(Parser)]
#[command(author = "NathanW", about = "A simple ray tracer.")]
pub struct Args {
    #[clap(short, long)]
    #[clap(value_enum, default_value_t)]
    pub format: OutputFormat,

    #[clap(short = 'n', long, default_value = "image")]
    pub image_name: String,

    #[clap(long, default_value = "1000")]
    pub width: u32,

    #[clap(long, default_value = "562")]
    pub height: u32,

    #[clap(long, default_value = "100")]
    #[clap(help = "")]
    pub samples: u32,

    #[clap(long, default_value = "50")]
    #[clap(help = "Maximum number of bounces per ray.")]
    pub max_depth: u32,
}

fn main() {
    println!("beep");
}



