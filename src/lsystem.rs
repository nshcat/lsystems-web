
mod line;
mod turtle;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use turtle::*;
use line::*;
use js_sys::Float64Array;
use std::convert::TryInto;

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
	step: f64
}

#[wasm_bindgen]
impl DrawingParameters {
	pub fn new() -> DrawingParameters {
		return DrawingParameters{
			start_position: Position2D{ x: 0, y: 0 },
			start_angle: 0.0,
			angle_delta: 45.0,
			step: 1.0
		}	
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

#[wasm_bindgen]
pub struct IterationRules {
	rule_map: HashMap<char, String>
}

impl IterationRules {
	pub fn set_rule(&mut self, character: char, replacement: String) {
		self.rule_map.remove(&character);
		self.rule_map.insert(character, replacement);	
	}

	fn retrieve_rule(&mut self, character: char) -> String {
		match self.rule_map.get(&character) {
			Some(right_side) => return right_side.clone(),
			None => panic!("Iteration map does not containt rule definition for character {}", character)
		}
	}

	fn has_rule(& self, character: char) -> bool {
		return self.rule_map.contains_key(&character);	
	}

	fn new() -> IterationRules {
		return IterationRules {
			rule_map: HashMap::new()	
		}	
	}
}


pub struct LSystem {
	axiom: String,
	iterations: u32,
	rules: IterationRules,
	interpretation_map: InterpretationMap,
	parameters: DrawingParameters,
	command_string: String,			// Drawing commands as source string. For debugging.
	commands: Vec<DrawOperation>,  // List of drawing commands that can be interpreted using drawing parameters
	line_segments: Vec<LineSegment> // List of line segments which are the current interpretation of the lsystem based on the current drawing parameters
}

impl LSystem {
	pub fn set_drawing_parameters(&mut self, params: &DrawingParameters) {
		self.parameters = DrawingParameters::clone(params);
	}

	pub fn interpret(&mut self) {
		let mut turtle = Turtle3D::new(self.parameters, self.iterations);

		turtle.execute(&self.commands);

		self.line_segments = turtle.line_segments().clone();
	}

	// Perform L-System iteration by applying ruleset to axiom string
	pub fn iterate(&mut self) {
		let mut iterated_str = self.axiom.to_owned();

		for ix in (0..self.iterations) {
			let mut new_str = String::from("").to_owned();

			for c in iterated_str.chars() {
				let replacement = if self.rules.has_rule(c) { self.rules.retrieve_rule(c) } else { c.to_string() };
				new_str = new_str + &replacement;
			}

			iterated_str = new_str;
		}

		let mut cmds = Vec::new();
		
		for c in iterated_str.chars() {
			if(self.interpretation_map.has_interpretation(c)) {
				let command = self.interpretation_map.retrieve(c);
				cmds.push(command);		
			}
		}

		self.commands = cmds;
		self.command_string = iterated_str;
	}

	pub fn new() -> LSystem {
		let mut x = LSystem {
			iterations: 5,
			rules: IterationRules::new(),
			axiom: String::from(""),
			command_string: String::from(""),
			interpretation_map: InterpretationMap::new(),
			parameters: DrawingParameters::new(),
			commands: Vec::new(),
			line_segments: Vec::new()
		};

		/*x.line_segments.push(
			LineSegment{ begin: Vertex{ x: 0.0, y:0.0, z:0.0 }, end: Vertex{ x: 1.0, y:1.0, z:1.0} }
		);*/

		return x;
	}
}

// Object that will be used by the web app. Contains functions to access and modify data.
#[wasm_bindgen]
pub struct LSystemInterface {
	lsystem: LSystem,
	line_vertices: Vec<f64>
}

#[wasm_bindgen]
impl LSystemInterface {
	pub fn new() -> LSystemInterface {
		return LSystemInterface{ lsystem: LSystem::new(), line_vertices: Vec::new() };
	}

	pub fn set_iterations(&mut self, iterations: u32) {
		self.lsystem.iterations = iterations;	
	}

	pub fn set_axiom(&mut self, axiom: String) {
		self.lsystem.axiom = axiom;
	}

	pub fn set_draw_parameters(&mut self, params: DrawingParameters) {
		self.lsystem.set_drawing_parameters(&params);	
	}

	pub fn set_rule(&mut self, character: char, rule: String) {
		self.lsystem.rules.set_rule(character, rule);
	}

	pub fn set_interpretation(&mut self, character: char, operation: DrawOperation) {
		self.lsystem.interpretation_map.associate(character, operation);	
	}

	pub fn iterate(&mut self) {
		self.lsystem.iterate();
	}

	pub fn interpret(&mut self) {
		self.lsystem.interpret();	
	}

	pub fn retrieve_command_string(& self) -> String {
		return self.lsystem.command_string.clone();
	}

	pub fn clear(&mut self) {
		self.lsystem = LSystem::new();
	}

	pub fn retrieve_lines(&mut self) -> *const f64 {
		let length = self.lsystem.line_segments.len() * 2 * 3;

		let mut data = Vec::new();

		for segment in &self.lsystem.line_segments {
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
}

