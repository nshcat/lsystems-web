use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct LineSegment {
	pub begin: Vertex,
	pub end: Vertex,
	pub color: i32
}
