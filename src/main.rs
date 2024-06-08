use anyhow::Context;
use clap::Parser;
use ray_tracer::OutputFormat;
use ray_tracer::render;
use ray_tracer::write_to_file;
use ray_tracer::parse_scene;

#[derive(Parser)]
#[command(author = "NathanW", about = "A simple ray tracer.")]
pub struct Args {
    #[clap(short, long)]
    #[clap(help = "Path to scene YAML file.")]
    pub scene: String,

    #[clap(short, long)]
    #[clap(value_enum, default_value_t)]
    pub format: OutputFormat,

    #[clap(short = 'n', long, default_value = "image")]
    pub image_name: String,

    #[clap(long, default_value = "1280")] // HD standard.
    pub width: u32,

    #[clap(long, default_value = "720")]
    pub height: u32,

    #[clap(long, default_value = "300")]
    #[clap(help = "")]
    pub samples: u32,

    #[clap(long, default_value = "100")]
    #[clap(help = "Maximum number of bounces per ray.")]
    pub max_depth: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let dimensions = (args.width, args.height);
    let (scene, camera) = parse_scene(&args.scene, dimensions).context("failed to parse scene")?;
    let image = render(scene, camera, dimensions, 100, 100);
    write_to_file(&args.image_name, image, OutputFormat::PNG, dimensions).context("failed to write to file")?;
    Ok(())
}



