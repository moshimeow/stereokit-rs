use crate::high_level::quat_from_angles;
use crate::values::MVec3;
use glam::{EulerRot, Mat4, Quat};
use std::ops::{AddAssign, Deref, MulAssign};

pub trait PosTrait {
	fn get_pos(&self) -> (f32, f32, f32) {
		let pos = self.get_pos_vec();
		(pos.x, pos.y, pos.z)
	}
	fn get_pos_vec(&self) -> glam::Vec3;

	fn set_pos(&mut self, x: f32, y: f32, z: f32) {
		self.set_pos_vec([x, y, z]);
	}
	fn set_pos_vec(&mut self, pos: impl Into<MVec3>);

	fn translate(&mut self, x: f32, y: f32, z: f32) {
		self.translate_vec([x, y, z])
	}
	fn translate_vec(&mut self, translation: impl Into<MVec3>);
}

pub trait ScaleTrait {
	fn get_scale(&self) -> (f32, f32, f32) {
		let scale = self.get_scale_vec();
		(scale.x, scale.y, scale.z)
	}
	fn get_scale_vec(&self) -> glam::Vec3;

	fn set_scale(&mut self, x: f32, y: f32, z: f32) {
		self.set_scale_vec([x, y, z])
	}
	fn set_scale_vec(&mut self, scale: impl Into<MVec3>);

	fn scale(&mut self, x: f32, y: f32, z: f32) {
		self.scale_vec([x, y, z]);
	}
	fn scale_vec(&mut self, scale: impl Into<MVec3>);
}

pub trait RotationTrait {
	fn get_rotation(&self) -> (f32, f32, f32) {
		let rotation = self.get_rotation_vec();
		(rotation.x, rotation.y, rotation.z)
	}
	fn get_rotation_vec(&self) -> glam::Vec3;

	fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
		self.set_rotation_vec([x, y, z]);
	}
	fn set_rotation_vec(&mut self, rotation: impl Into<MVec3>);

	fn rotate(&mut self, x: f32, y: f32, z: f32) {
		self.rotate_vec([x, y, z]);
	}
	fn rotate_vec(&mut self, rotation: impl Into<MVec3>);
}

pub trait MatrixTrait {
	fn get_matrix(&self) -> Mat4;
	fn set_matrix(&mut self, matrix: Mat4);
}

pub struct MatrixContainer {
	pub mat4: Mat4,
	pub pos: glam::Vec3,
	pub rotation: glam::Vec3,
	pub scale: glam::Vec3,
}

impl MatrixContainer {
	pub fn sync_matrix(&mut self) {
		self.mat4 = Mat4::from_scale_rotation_translation(
			self.scale,
			quat_from_angles(self.rotation.x, self.rotation.y, self.rotation.z),
			self.pos,
		);
	}
	pub fn sync_fields(&mut self) {
		let (a, b, c) = self.mat4.to_scale_rotation_translation();
		self.scale = a;
		self.pos = c;
		let r = b.to_euler(EulerRot::XYZ);
		self.rotation = MVec3::from([r.0.to_degrees(), r.1.to_degrees(), r.2.to_degrees()]).into();
	}
	pub fn new(pos: impl Into<MVec3>, rotation: impl Into<MVec3>, scale: impl Into<MVec3>) -> Self {
		let mut matrix_container = MatrixContainer {
			mat4: Default::default(),
			pos: glam::Vec3::from(pos.into()),
			rotation: glam::Vec3::from(rotation.into()),
			scale: glam::Vec3::from(scale.into()),
		};
		matrix_container.sync_matrix();
		matrix_container
	}
}

impl MatrixTrait for MatrixContainer {
	fn get_matrix(&self) -> Mat4 {
		self.mat4.clone()
	}
	fn set_matrix(&mut self, matrix: Mat4) {
		self.mat4 = matrix;
		self.sync_fields();
	}
}

impl PosTrait for MatrixContainer {
	fn get_pos_vec(&self) -> glam::Vec3 {
		self.pos
	}

	fn set_pos_vec(&mut self, pos: impl Into<MVec3>) {
		self.pos = glam::Vec3::from(pos.into());
		self.sync_matrix();
	}

	fn translate_vec(&mut self, translation: impl Into<MVec3>) {
		self.pos.add_assign(glam::Vec3::from(translation.into()));
		self.sync_matrix();
	}
}

impl RotationTrait for MatrixContainer {
	fn get_rotation_vec(&self) -> glam::Vec3 {
		self.rotation
	}

	fn set_rotation_vec(&mut self, rotation: impl Into<MVec3>) {
		self.rotation = glam::Vec3::from(rotation.into());
		self.sync_matrix();
	}

	fn rotate_vec(&mut self, rotation: impl Into<MVec3>) {
		self.rotation.add_assign(glam::Vec3::from(rotation.into()));
		self.sync_matrix();
	}
}

impl ScaleTrait for MatrixContainer {
	fn get_scale_vec(&self) -> glam::Vec3 {
		self.scale
	}

	fn set_scale_vec(&mut self, scale: impl Into<MVec3>) {
		self.scale = glam::Vec3::from(scale.into());
		self.sync_matrix();
	}

	fn scale_vec(&mut self, scale: impl Into<MVec3>) {
		self.scale.add_assign(glam::Vec3::from(scale.into()));
		self.sync_matrix();
	}
}
