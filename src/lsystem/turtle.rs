use wasm_bindgen::prelude::*;
use nalgebra::*;
use crate::lsystem::DrawingParameters;
use crate::lsystem::line::LineSegment;
use crate::lsystem::Position2D;
use crate::lsystem::line::Vertex;
use std::num::*;
use std::cmp::*;

type Matrix3f = Matrix3<f64>;
type Vector3f = Vector3<f64>;
type Vector3fU = Unit<Vector3f>;

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
	ForwardContracting = 7,	// use length s^n, where n is the number of iterations and is the step size

	// 3D operations
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

#[derive(Clone, Copy, Debug)]
struct Vector2D {
	x: f64,
	y: f64
}

#[derive(Clone, Copy, Debug)]
struct Turtle2DState {
	position: Vector2D,
	direction: Vector2D
}

pub struct Turtle2D {
	draw_parameters: DrawingParameters,
	contracted_length: f64,  // s^n for ForwardContracted
	line_segments: Vec<LineSegment>,
	current_state: Turtle2DState,
	state_stack: Vec<Turtle2DState>,
	cache_cos: f64,
	cache_sin: f64
}

impl Turtle2D {
	pub fn new(draw_parameters: DrawingParameters, num_iterations: u32) -> Turtle2D {
		return Turtle2D {
			draw_parameters: draw_parameters,
			line_segments: Vec::new(),
			contracted_length: draw_parameters.step.powf(num_iterations as f64),
			state_stack: Vec::new(),
			current_state: Turtle2DState {
				position: Vector2D{ x: draw_parameters.start_position.x as f64, y: draw_parameters.start_position.y as f64 },
				direction: Vector2D{ x: draw_parameters.start_angle.cos(), y: draw_parameters.start_angle.sin() }
			},
			cache_cos: draw_parameters.angle_delta.cos(),
			cache_sin: draw_parameters.angle_delta.sin()
		}	
	}

	pub fn execute(&mut self, commands: &[DrawOperation]) {
		for command in commands {
			match command {
				DrawOperation::Forward => self.move_forward(self.draw_parameters.step, true),
				DrawOperation::ForwardNoDraw => self.move_forward(self.draw_parameters.step, false),
				DrawOperation::ForwardContracting => self.move_forward(self.contracted_length, true),
				DrawOperation::TurnLeft => self.turn_left(),
				DrawOperation::TurnRight => self.turn_right(),
				DrawOperation::Ignore => (),
				DrawOperation::SaveState => self.push_state(),
				DrawOperation::LoadState => self.pop_state(),
				_ => ()
			}
		}
	}

	pub fn line_segments(& self) -> &Vec<LineSegment> {
		return &self.line_segments;	
	}

	pub fn move_forward(&mut self, distance: f64, draw: bool) {
		let old_position = self.current_state.position;
		
		let dx = distance * self.current_state.direction.x;
		let dy = distance * -self.current_state.direction.y;

		let new_position = Vector2D {
			x: self.current_state.position.x + dx, 	
			y: self.current_state.position.y + dy
		};
		
		self.current_state.position = new_position;

		if(draw) {
			let begin = Vertex { x: old_position.x, y: old_position.y, z: 0.0 };
			let end = Vertex { x: new_position.x, y: new_position.y, z: 0.0 };

			self.line_segments.push(LineSegment{
				begin: begin, end: end, color: 0, width: 1.0
			});		
		}
	}

	pub fn turn_right(&mut self) {
		self.current_state.direction = Vector2D {
			x: (self.current_state.direction.x * self.cache_cos) - (self.current_state.direction.y * (-self.cache_sin)),
			y: (self.current_state.direction.x * (-self.cache_sin)) + self.current_state.direction.y * self.cache_cos
		};
	}

	pub fn turn_left(&mut self) {
		self.current_state.direction = Vector2D {
			x: self.current_state.direction.x * self.cache_cos - self.current_state.direction.y * self.cache_sin,
			y: self.current_state.direction.x * self.cache_sin + self.current_state.direction.y * self.cache_cos
		};
	}
	
	pub fn push_state(&mut self) {
		self.state_stack.push(self.current_state.clone())
	}

	pub fn pop_state(&mut self) {
		if(self.state_stack.len() > 0) {
			self.current_state = self.state_stack.last().unwrap().clone();
			self.state_stack.pop();
		}
	}
}


struct Turtle3DMatrixCache {
	turn_left: Matrix3f,
	turn_right: Matrix3f,
	pitch_up: Matrix3f,
	pitch_down: Matrix3f,
	roll_left: Matrix3f,
	roll_right: Matrix3f,
	turn_around: Matrix3f
}

impl Turtle3DMatrixCache {
	fn new(angle: f64) -> Turtle3DMatrixCache {
		return Turtle3DMatrixCache{
			turn_left: Self::rotU(angle),
			turn_right: Self::rotU(-angle),
			pitch_down: Self::rotL(angle),
			pitch_up: Self::rotL(-angle),
			roll_left: Self::rotH(angle),
			roll_right: Self::rotH(-angle),
			turn_around: Self::rotU(std::f64::consts::PI)
		};
	}

	fn rotU(angle: f64) -> Matrix3f {
		return Matrix3f::new(
			angle.cos(), angle.sin(), 0.0,
			-angle.sin(), angle.cos(), 0.0,
			0.0, 0.0, 1.0 					
		);
	}

	fn rotL(angle: f64) -> Matrix3f {
		return Matrix3f::new(
			angle.cos(), 0.0, -angle.sin(),
			0.0, 1.0, 0.0,
			angle.sin(), 0.0, angle.cos() 					
		);
	}

	fn rotH(angle: f64) -> Matrix3f {
		return Matrix3f::new(
			1.0, 0.0, 0.0,
			0.0, angle.cos(), -angle.sin(),
			0.0, angle.sin(), angle.cos() 					
		);
	}
}

#[derive(Clone, Debug)]
pub struct Polygon {
	pub vertices: Vec<Vector3f>,
	pub color: i32
}

#[derive(Clone, Copy, Debug)]
struct Turtle3DState {
	position: Vector3f,
	heading: Vector3fU,
	left: Vector3fU,
	up: Vector3fU,
	color_index: i32,
	line_width: f64
}

impl Turtle3DState {
	fn new(start_position: Vector3f, start_angle: f64, initial_line_width: f64) -> Turtle3DState {
		let up = Vector3f::z_axis();
		let heading = Vector3fU::new_unchecked(Vector3f::new(start_angle.cos(), start_angle.sin(), 0.0));
		let left = Vector3fU::new_normalize(up.cross(&heading));

		return Turtle3DState {
			up: up,
			heading: heading,
			left: left,
			position: start_position,
			color_index: 0,
			line_width: initial_line_width
		};
	}
}

pub struct Turtle3D {
	draw_parameters: DrawingParameters,
	contracted_length: f64,
	matrix_cache: Turtle3DMatrixCache,
	line_segments: Vec<LineSegment>,
	current_state: Turtle3DState,
	state_stack: Vec<Turtle3DState>,
	current_polygon: Vec<Vector3f>,
	polygons: Vec<Polygon>
}

impl Turtle3D {
	pub fn new(draw_parameters: DrawingParameters, num_iterations: u32) -> Turtle3D {
		return Turtle3D {
			draw_parameters: draw_parameters,
			line_segments: Vec::new(),
			contracted_length: draw_parameters.step.powf(num_iterations as f64),
			state_stack: Vec::new(),
			matrix_cache: Turtle3DMatrixCache::new(draw_parameters.angle_delta),
			current_polygon: Vec::new(),
			polygons: Vec::new(),
			current_state: Turtle3DState::new(
				Vector3f::new(draw_parameters.start_position.x as f64, draw_parameters.start_position.y as f64, 0.0),
				draw_parameters.start_angle,
				draw_parameters.initial_line_width
			)
		}	
	}

	fn is_polygon_active(&self) -> bool {
		return !self.current_polygon.is_empty();
	}

	fn submit_vertex(&mut self) {
		self.current_polygon.push(self.current_state.position.clone());
	}

	fn begin_polygon(&mut self) {
		// Nothing to do
	}

	fn end_polygon(&mut self) {
		self.polygons.push(
			Polygon {
				vertices: self.current_polygon.clone(),
				color: self.current_state.color_index		
			}
		);

		self.current_polygon.clear();
	}

	fn apply_rotation(&mut self, matrix: Matrix3f) {
		let composite = Matrix3f::from_columns(&[
			self.current_state.heading.into_inner(),
			self.current_state.left.into_inner(),
			self.current_state.up.into_inner()
		]);

		let new_composite = composite * matrix;

		self.current_state.heading = Vector3fU::new_normalize(new_composite.column(0).into());
		self.current_state.left = Vector3fU::new_normalize(new_composite.column(1).into());
		self.current_state.up = Vector3fU::new_normalize(new_composite.column(2).into());
	}

	pub fn modify_line_width(&mut self, delta: f64) {
		self.current_state.line_width = (self.current_state.line_width + delta).max(0.0);	
	}

	pub fn execute(&mut self, commands: &[DrawOperation]) {
		for command in commands {
			match command {
				DrawOperation::Forward => self.move_forward(self.draw_parameters.step, true),
				DrawOperation::ForwardNoDraw => self.move_forward(self.draw_parameters.step, false),
				DrawOperation::ForwardContracting => self.move_forward(self.contracted_length, true),
				DrawOperation::Ignore => (),
				DrawOperation::SaveState => self.push_state(),
				DrawOperation::LoadState => self.pop_state(),
			
				DrawOperation::TurnLeft => self.apply_rotation(self.matrix_cache.turn_left),
				DrawOperation::TurnRight => self.apply_rotation(self.matrix_cache.turn_right),
				DrawOperation::PitchDown => self.apply_rotation(self.matrix_cache.pitch_down),
				DrawOperation::PitchUp => self.apply_rotation(self.matrix_cache.pitch_up),
				DrawOperation::RollLeft => self.apply_rotation(self.matrix_cache.roll_left),
				DrawOperation::RollRight => self.apply_rotation(self.matrix_cache.roll_right),
				DrawOperation::TurnAround => self.apply_rotation(self.matrix_cache.turn_around),

				DrawOperation::BeginPolygon => self.begin_polygon(),
				DrawOperation::EndPolygon => self.end_polygon(),
				DrawOperation::SubmitVertex => self.submit_vertex(),
				
				DrawOperation::IncrementColor => self.modify_color_index(1),
				DrawOperation::DecrementColor => self.modify_color_index(1),

				DrawOperation::IncrementLineWidth => self.modify_line_width(self.draw_parameters.line_width_delta),
				DrawOperation::DecrementLineWidth => self.modify_line_width(-self.draw_parameters.line_width_delta)
			}
		}
	}

	fn modify_color_index(&mut self, value: i32) {
		self.current_state.color_index = clamp(self.current_state.color_index + value, 0, (self.draw_parameters.color_palette_size - 1) as i32);
	}

	pub fn line_segments(& self) -> &Vec<LineSegment> {
		return &self.line_segments;	
	}

	pub fn polygons(& self) -> &Vec<Polygon> {
		return &self.polygons;	
	}

	pub fn move_forward(&mut self, distance: f64, draw: bool) {
		let old_position = self.current_state.position;
		
		let mv = self.current_state.heading.into_inner() * distance;

		self.current_state.position = old_position + mv;

		if(draw) {
			let begin = Vertex { x: old_position.x, y: old_position.y, z: old_position.z };
			let end = Vertex { x: self.current_state.position.x, y: self.current_state.position.y, z: self.current_state.position.z };

			self.line_segments.push(LineSegment{
				begin: begin,
				end: end,
				color: self.current_state.color_index,
				width: self.current_state.line_width
			});		
		}
	}

	pub fn push_state(&mut self) {
		self.state_stack.push(self.current_state.clone())
	}

	pub fn pop_state(&mut self) {
		if(self.state_stack.len() > 0) {
			self.current_state = self.state_stack.last().unwrap().clone();
			self.state_stack.pop();
		}
	}
}

