//! This module implements logic for interacting with the `.ppm` image format.

use std::{io::Write, path::Path};

use crate::Color;

pub struct PPM {
    pub columns: u32,
    pub rows: u32,
    pub max_color: u32,
    pixels: Vec<Color>,
}

impl PPM {
    pub fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        // Write header
        write!(
            writer,
            "P3\n{} {}\n{}\n",
            self.columns, self.rows, self.max_color
        )?;

        for pixel in self.pixels.iter() {
            writeln!(writer, "{}", pixel)?;
        }
        Ok(())
    }

    pub fn write_to_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut fptr = std::fs::File::create(path)?;
        self.write(&mut fptr)
    }

    pub fn new(columns: u32, rows: u32, max_color: u32) -> Self {
        Self {
            columns,
            rows,
            max_color,
            pixels: vec![Color::default(); (columns * rows) as usize],
        }
    }
}
