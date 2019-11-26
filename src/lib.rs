mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Float64Array;
use std::convert::TryInto;
use wasm_bindgen::JsValue;
use std::string::*;
use crate::utils::*;

use lsystems_core::*;
use lsystems_core::drawing::{DrawingParameters};
use lsystems_core::drawing::types::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawOperation {
	Forward = 0,
	ForwardNoDraw = 1,
	TurnRight = 2,
	TurnLeft = 3,
	SaveState = 4,
	LoadState = 5,
	Ignore = 6,
	ForwardContracting = 7,
	PitchDown = 8,
	PitchUp = 9,
	RollLeft = 10,
	RollRight = 11,
	TurnAround = 12,
	BeginPolygon = 13,
	EndPolygon = 14,
	SubmitVertex = 15,
	IncrementColor = 16,
	DecrementColor = 17,
	IncrementLineWidth = 18,
	DecrementLineWidth = 19
}

#[wasm_bindgen]
pub struct DrawingParametersInterface {
	params: DrawingParameters
}

#[wasm_bindgen]
impl DrawingParametersInterface {
	pub fn new() -> DrawingParametersInterface {
		DrawingParametersInterface {
			params: DrawingParameters::new()		
		}
	}

	pub fn set_initial_line_width(&mut self, width: f64) {
		self.params.initial_line_width = width;
	}

	pub fn set_line_width_delta(&mut self, delta: f64) {
		self.params.line_width_delta = delta;
	}

	pub fn set_start_position(&mut self, x: f64, y: f64) {
		self.params.start_position = Vector2f::new(x, y);	
	}

	pub fn set_start_angle(&mut self, angle: f64) {
		self.params.start_angle = angle;	
	}

	pub fn set_start_angle_degrees(&mut self, angle: f64) {
		self.params.start_angle = angle.to_radians();	
	}
	
	pub fn set_angle_delta(&mut self, angle: f64) {
		self.params.angle_delta = angle;	
	}

	pub fn set_angle_delta_degrees(&mut self, angle: f64) {
		self.params.angle_delta = angle.to_radians();	
	}

	pub fn set_step(&mut self, step: f64) {
		self.params.step = step;
	}

	pub fn set_color_palette_size(&mut self, size: u32) {
		self.params.color_palette_size = size;	
	}
}

#[wasm_bindgen]
pub struct LSystemInterface {
	lsystem: LSystem,
	line_vertices: Vec<f64>,
	polygons: Vec<f64>
}

#[wasm_bindgen]
impl LSystemInterface {
	pub fn new() -> LSystemInterface {
		set_panic_hook();
		return LSystemInterface{ lsystem: LSystem::new(), line_vertices: Vec::new(), polygons: Vec::new() };
	}

	pub fn set_iterations(&mut self, iterations: u32) {
		self.lsystem.set_iteration_depth(iterations);	
	}

	pub fn set_draw_parameters(&mut self, params: DrawingParametersInterface) {
		self.lsystem.set_drawing_parameters(&params.params);	
	}

	pub fn set_rules_and_axiom(&mut self, axiom: &str, rules: &str) {
		self.lsystem.parse(axiom, rules);	
	}

	pub fn set_interpretation(&mut self, character: char, operation: DrawOperation) {
		self.lsystem.interpretations.associate(character, Self::convert_operation(operation));	
	}

	pub fn set_seed(&mut self, seed: u64) {
		self.lsystem.engine.set_seed(seed);	
	}

	pub fn iterate(&mut self) {
		self.lsystem.iterate();
	}

	pub fn interpret(&mut self) {
		self.lsystem.interpret();	
	}

	fn convert_operation(op: DrawOperation) -> lsystems_core::drawing::DrawOperation {
		match op {
			DrawOperation::Forward => lsystems_core::drawing::DrawOperation::Forward,
			DrawOperation::ForwardNoDraw => lsystems_core::drawing::DrawOperation::ForwardNoDraw,
			DrawOperation::TurnRight => lsystems_core::drawing::DrawOperation::TurnRight,
			DrawOperation::TurnLeft => lsystems_core::drawing::DrawOperation::TurnLeft,
			DrawOperation::SaveState => lsystems_core::drawing::DrawOperation::SaveState,
			DrawOperation::LoadState => lsystems_core::drawing::DrawOperation::LoadState,
			DrawOperation::Ignore => lsystems_core::drawing::DrawOperation::Ignore,
			DrawOperation::ForwardContracting => lsystems_core::drawing::DrawOperation::ForwardContracting,
			DrawOperation::PitchDown => lsystems_core::drawing::DrawOperation::PitchDown,
			DrawOperation::PitchUp => lsystems_core::drawing::DrawOperation::PitchUp,
			DrawOperation::RollLeft => lsystems_core::drawing::DrawOperation::RollLeft,
			DrawOperation::RollRight => lsystems_core::drawing::DrawOperation::RollRight,
			DrawOperation::TurnAround => lsystems_core::drawing::DrawOperation::TurnAround,
			DrawOperation::BeginPolygon => lsystems_core::drawing::DrawOperation::BeginPolygon,
			DrawOperation::EndPolygon => lsystems_core::drawing::DrawOperation::EndPolygon,
			DrawOperation::SubmitVertex => lsystems_core::drawing::DrawOperation::SubmitVertex,
			DrawOperation::IncrementColor => lsystems_core::drawing::DrawOperation::IncrementColor,
			DrawOperation::DecrementColor => lsystems_core::drawing::DrawOperation::DecrementColor,
			DrawOperation::IncrementLineWidth => lsystems_core::drawing::DrawOperation::IncrementLineWidth,
			DrawOperation::DecrementLineWidth => lsystems_core::drawing::DrawOperation::DecrementLineWidth
		}
	}

	pub fn retrieve_final_string(& self) -> String {
		let mut str = String::new();

		for module in &self.lsystem.commands {
			str = str + &format!("{}", module);
		}

		return str;
	}


	pub fn clear(&mut self) {
		self.lsystem = LSystem::new();
	}

	pub fn retrieve_lines(&mut self) -> *const f64 {
		let length = self.lsystem.line_segments.len() * 2 * 3;

		let mut data = Vec::new();

		for segment in &self.lsystem.line_segments {
			data.push(segment.color as f64);
			data.push(segment.width);
			data.push(segment.begin.x);
			data.push(segment.begin.y);
			data.push(segment.begin.z);
			data.push(segment.end.x);
			data.push(segment.end.y);
			data.push(segment.end.z);
		}

		self.line_vertices = data;
		return self.line_vertices.as_ptr();
	}

	pub fn retrieve_lines_length(&mut self) -> usize {
		return self.line_vertices.len();	
	}

	pub fn retrieve_polygons(&mut self) -> *const f64 {

		let mut data = Vec::new();

		for polygon in &self.lsystem.polygons {
			data.push(polygon.vertices.len() as f64);
			data.push(polygon.color as f64);
	
			for vertex in &polygon.vertices {
				data.push(vertex.x);
				data.push(vertex.y);
				data.push(vertex.z);
			}
		}

		self.polygons = data;
		return self.polygons.as_ptr();
	}

	pub fn retrieve_polygons_length(&mut self) -> usize {
		return self.polygons.len();	
	}
}
