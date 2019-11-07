use core::ops::Index;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub struct Model {
    pub verts: Vec<Vector<f32>>,
    pub faces: Vec<Vector<i32>>,
}

impl Model {
    pub fn new(path: &str) -> std::io::Result<Model> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut verts: Vec<Vector<f32>> = Vec::new();
        let mut faces: Vec<Vector<i32>> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            // remove comments
            let line = line.split('#').next().unwrap();

            let mut words = line.split_whitespace();

            // parse line by statement
            match words.next() {
                Some("v") => {
                    // parse remaining words into f32 floats as vertexes
                    let args: Vec<f32> = words.map(|w| w.parse::<f32>().unwrap()).collect();
                    verts.push(Vector::new(args[0], args[1], args[2]))
                }
                Some("f") => {
                    // parse remaining words into i32 integer as faces (only take the first from each word)
                    let args: Vec<i32> = words
                        .map(|w| {
                            let first = w.split("/").next().unwrap();
                            first.parse::<i32>().unwrap()
                        })
                        .collect();
                    faces.push(Vector::new(args[0], args[1], args[2]))
                }
                _ => continue,
            }
        }

        Ok(Model { verts, faces })
    }
}

pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector { x, y, z }
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
        }
    }
}
