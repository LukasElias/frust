extern crate glow;

#[derive(Default)]
pub struct Vector3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Vector3 {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self {
			x,
			y,
			z
		}
	}
}

pub struct Vertex {
	pub position: Vector3,
}

pub struct Geometry {
	pub vertices: Vec<Vertex>,
	pub indeces: Vec<u64>,
}

pub struct GameObject {
	pub geometry: Geometry,
	pub position: Vector3,
	pub program: glow::Program,
}

pub struct Texture {

}

pub struct Scene {
	pub game_objects: Vec<GameObject>,
}

impl Scene {
	pub fn new() -> Self {
		Self {
			game_objects: vec![]
		}
	}
}