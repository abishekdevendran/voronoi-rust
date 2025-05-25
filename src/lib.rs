use std::iter::repeat_with;
use image::{ImageBuffer, Rgb};
use kiddo::{ImmutableKdTree, SquaredEuclidean};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Site {
    pub point: Point,
    pub color: Color,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl Site {
    pub fn new(point: Point, color: Color) -> Self {
        Site { point, color }
    }
}

/// Saves the pixel data as an image file.
pub fn save_pixels_to_image(
    filename: &str,
    width: usize,
    height: usize,
    pixels: &[Vec<Color>],
) -> Result<(), image::ImageError> {
    let img_buffer = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let pixel_color = pixels[y as usize][x as usize];
        Rgb([pixel_color.r, pixel_color.g, pixel_color.b])
    });
    img_buffer.save(filename)
}

/// Generates a vector of random sites.
pub fn generate_sites(count: usize, width: usize, height: usize, seed: u64) -> Vec<Site> {
  fastrand::seed(seed);
  let mut rng = fastrand::Rng::new();
  repeat_with(|| {
      Site::new(
          Point::new(rng.f64() * width as f64, rng.f64() * height as f64),
          Color::new(rng.u8(..), rng.u8(..), rng.u8(..)),
      )
  })
  .take(count)
  .collect::<Vec<Site>>()
}

/// Builds an immutable k-d tree from the site points.
pub fn build_kdtree_from_sites(sites: &[Site]) -> ImmutableKdTree<f64, 2> {
  let points: Vec<[f64; 2]> = sites
      .iter()
      .map(|site| [site.point.x, site.point.y])
      .collect();
  (&*points).into()
}

/// Generates the pixel data for the Voronoi diagram.
pub fn generate_voronoi_pixels(
  width: usize,
  height: usize,
  sites: &[Site],
  tree: &ImmutableKdTree<f64, 2>, // Adjust type for Kiddo v3 if needed
) -> Vec<Vec<Color>> {
  (0..height)
      .into_par_iter()
      .map(|y_idx| {
          (0..width)
              .map(|x_idx| {
                  // For Kiddo v2.x or v3.x with `f64` feature
                  let nearest_result =
                      tree.nearest_one::<SquaredEuclidean>(&[x_idx as f64, y_idx as f64]);
                  // For Kiddo v3.x the return type of nearest_one might be (f64, u32) for (distance, index)
                  // let (_dist, site_index) = tree.nearest_one::<SquaredEuclidean>(&[x_idx as f64, y_idx as f64]);
                  // let site_index = site_index as usize;
                  
                  let site_index = nearest_result.item as usize; // Kiddo v2.x stores index in `item`
                                                               // For Kiddo v3.x, it would be `nearest_result.1 as usize`
                  sites[site_index].color
              })
              .collect::<Vec<Color>>()
      })
      .collect::<Vec<Vec<Color>>>()
}