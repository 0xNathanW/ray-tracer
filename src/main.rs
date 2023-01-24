use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use clap::Parser;
use ray_tracer::camera::Camera;
use ray_tracer::object::Scene;
use ray_tracer::vec3::Vec3;
use ray_tracer::point3::Point3;
use ray_tracer::render::render;
use ray_tracer::output::write_to_file;
use ray_tracer::Args;


fn main() {

    let args = Args::parse();
    let scene = Arc::new(Scene::randomised_scene());
    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        args.width as f64 / args.height as f64,
        0.1,
        10.0
    );

    let progress_bar = ProgressBar::new(args.height as u64);
    progress_bar.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-")
    );

    let image = render(
        scene,
        camera,
        args.width,
        args.height,
        args.samples,
        args.max_depth,
        Some(progress_bar)
    );

    write_to_file(
        &args.image_name,
        image,
        args.format,
        (args.width as usize, args.height as usize),
    ).unwrap();

}



