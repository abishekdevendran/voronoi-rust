use std::time::Instant;
use clap::Parser;
use voronoi_rust::{generate_sites, build_kdtree_from_sites, generate_voronoi_pixels, save_pixels_to_image};

// Default constants (can be overridden by CLI args)
const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 800;
const DEFAULT_SITE_COUNT: usize = 50;
const DEFAULT_RANDOM_SEED: u64 = 42;
const DEFAULT_OUTPUT_FILE_PREFIX: &str = "output_voronoi_";
const DEFAULT_OUTPUT_FORMAT: &str = "png";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, default_value_t = DEFAULT_WIDTH)]
    width: usize,

    #[clap(long, default_value_t = DEFAULT_HEIGHT)]
    height: usize,

    #[clap(long, short = 'c', default_value_t = DEFAULT_SITE_COUNT)]
    sites: usize,

    #[clap(long, short = 's', default_value_t = DEFAULT_RANDOM_SEED)]
    seed: u64,

    #[clap(long, short = 'p', default_value = DEFAULT_OUTPUT_FILE_PREFIX)]
    prefix: String,

    #[clap(long, short = 'f', default_value = DEFAULT_OUTPUT_FORMAT)]
    format: String,
}

fn main() {
    let args = Args::parse();

    println!("Voronoi Diagram Generator - Setup");
    println!("Dimensions: {}x{}", args.width, args.height);
    println!("Number of sites: {}", args.sites);
    println!("Random seed: {}", args.seed);
    println!("Output prefix: {}", args.prefix);
    println!("Output format: {}", args.format);

    // 1. Generate Sites
    let sites_vec = generate_sites(args.sites, args.width, args.height, args.seed);

    // 2. Build KD-Tree
    // Note: The kdtree type might need adjustment for Kiddo v3.x
    // e.g. kiddo::ImmutableKdTree<f64, u32, 2, 32, u32>
    // For Kiddo v2.x or v3.x with `f64` feature, this should be fine:
    let kdtree = build_kdtree_from_sites(&sites_vec);


    println!("\n--- Running Optimized (Parallel) Version ---");
    let start_pixel_calc_par = Instant::now();

    // 3. Generate Voronoi Pixel Data
    let pixel_data = generate_voronoi_pixels(
        args.width,
        args.height,
        &sites_vec,
        &kdtree,
    );

    println!(
        "Parallel pixel calculation took: {:.2?}",
        start_pixel_calc_par.elapsed()
    );

    // 4. Save Image
    let output_filename = format!(
        "{}{}.{}",
        args.prefix,
        args.seed, // Using seed in filename as per original
        args.format
    );

    match save_pixels_to_image(
        &output_filename,
        args.width,
        args.height,
        &pixel_data,
    ) {
        Ok(_) => println!("Successfully saved Voronoi diagram to {}", output_filename),
        Err(e) => eprintln!("Error saving image: {}", e),
    }
}