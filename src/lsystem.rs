
mod line;
mod turtle;
mod engine;
mod grammar;
mod weighted;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use turtle::*;
use line::*;
use engine::*;
use js_sys::Float64Array;
use std::convert::TryInto;
use crate::lsystem::turtle::Polygon;


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position2D {
	x: i32,
	y: i32
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct DrawingParameters {
	start_position: Position2D,
	start_angle: f64,
	angle_delta: f64,
	step: f64,
	color_palette_size: u32,
	initial_line_width: f64,
	line_width_delta: f64
}

#[wasm_bindgen]
impl DrawingParameters {
	pub fn new() -> DrawingParameters {
		return DrawingParameters{
			start_position: Position2D{ x: 0, y: 0 },
			start_angle: 0.0,
			angle_delta: 45.0,
			step: 1.0,
			color_palette_size: 1,
			initial_line_width: 1.0,
			line_width_delta: 0.1
		}	
	}

	pub fn set_initial_line_width(&mut self, width: f64) {
		self.initial_line_width = width;
	}

	pub fn set_line_width_delta(&mut self, delta: f64) {
		self.line_width_delta = delta;
	}

	pub fn set_start_position(&mut self, x: i32, y: i32) {
		self.start_position = Position2D{ x: x, y: y };	
	}

	pub fn set_start_angle(&mut self, angle: f64) {
		self.start_angle = angle;	
	}

	pub fn set_start_angle_degrees(&mut self, angle: f64) {
		self.start_angle = angle.to_radians();	
	}
	
	pub fn set_angle_delta(&mut self, angle: f64) {
		self.angle_delta = angle;	
	}

	pub fn set_angle_delta_degrees(&mut self, angle: f64) {
		self.angle_delta = angle.to_radians();	
	}

	pub fn set_step(&mut self, step: f64) {
		self.step = step;
	}

	pub fn set_color_palette_size(&mut self, size: u32) {
		self.color_palette_size = size;	
	}
}


pub struct InterpretationMap {
	internal_map: HashMap<char, DrawOperation>
}

impl InterpretationMap {
	// Called from java script, set a new association for given character. This will remove all
    // other existing associations of the given character.
    fn associate(&mut self, character:char, operation: DrawOperation) {
		self.internal_map.remove(&character);
		self.internal_map.insert(character, operation);
	}

	fn retrieve(&self, character: char) -> DrawOperation {
		match self.internal_map.get(&character) {
			Some(operation) => return operation.clone(),
			None => panic!("Interpretation map does not contain definition for character {}", character)
		}
	}

	fn has_interpretation(&self, character: char) -> bool {
		return self.internal_map.contains_key(&character);
	}

	fn new() -> InterpretationMap {
		return InterpretationMap{ internal_map: HashMap::new() };
	}
}



pub struct LSystem {
	engine: IterationEngine,
	interpretations: InterpretationMap,
	parameters: DrawingParameters,	
	commands: Vec<DrawingModule>,  
	line_segments: Vec<LineSegment>, // List of line segments which are the current interpretation of the lsystem based on the current drawing parameters
	polygons: Vec<Polygon>
}

impl LSystem {
	pub fn set_drawing_parameters(&mut self, params: &DrawingParameters) {
		self.parameters = DrawingParameters::clone(params);
	}

	// Perform L-System iteration by applying ruleset to axiom string
	pub fn iterate(&mut self) {
		self.engine.iterate();

		let mut commands: Vec<DrawingModule> = Vec::new();
		
		for module in &self.engine.module_string {
			if(self.interpretations.has_interpretation(module.identifier)) {
				let operation = self.interpretations.retrieve(module.identifier);

				let command = match module.parameter_count() {
					0 => DrawingModule::new(operation),
					1 => DrawingModule::new_with_parameter(operation, module.parameter_values[0]),
					_ => panic!("Drawing operation can't have more than one parameters, but has {}", module.parameter_count())	
				};

				commands.push(command);
			}
		}

		self.commands = commands.clone();
	}

	pub fn interpret(&mut self) {
		let mut turtle = Turtle3D::new(self.parameters, self.engine.iteration_depth);

		turtle.execute_modules(&self.commands);

		self.line_segments = turtle.line_segments().clone();
		self.polygons = turtle.polygons().clone();
	}

	pub fn parse(&mut self, axiom: &str, rules: &str) {
		self.engine.module_string = grammar::lsystem_parser::module_string(axiom).unwrap_or(Vec::new());
		self.engine.rules = grammar::lsystem_parser::rule_list(rules).unwrap_or(Vec::new());
	}

	pub fn new() -> LSystem {
		LSystem {
			engine: IterationEngine::new(),
			interpretations: InterpretationMap::new(),
			parameters: DrawingParameters::new(),
			commands: Vec::new(),
			line_segments: Vec::new(),
			polygons: Vec::new()
		}
	}

	pub fn set_iteration_depth(&mut self, depth: u32) {
		self.engine.set_iteration_depth(depth);	
	}
}

// Object that will be used by the web app. Contains functions to access and modify data.
#[wasm_bindgen]
pub struct LSystemInterface {
	lsystem: LSystem,
	line_vertices: Vec<f64>,
	polygons: Vec<f64>
}

#[wasm_bindgen]
impl LSystemInterface {
	pub fn new() -> LSystemInterface {
		return LSystemInterface{ lsystem: LSystem::new(), line_vertices: Vec::new(), polygons: Vec::new() };
	}

	pub fn set_iterations(&mut self, iterations: u32) {
		self.lsystem.set_iteration_depth(iterations);	
	}

	pub fn set_draw_parameters(&mut self, params: DrawingParameters) {
		self.lsystem.set_drawing_parameters(&params);	
	}

	pub fn set_rules_and_axiom(&mut self, axiom: &str, rules: &str) {
		self.lsystem.parse(axiom, rules);	
	}

	pub fn set_interpretation(&mut self, character: char, operation: DrawOperation) {
		self.lsystem.interpretations.associate(character, operation);	
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

