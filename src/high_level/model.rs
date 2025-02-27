// use crate::model::Model;

use crate::bounds::Bounds;
use crate::high_level::collider::{CapsuleCollider, Collider, ColliderType};
use crate::high_level::math_traits::{
	MatrixContainer, MatrixTrait, PosTrait, RotationTrait, ScaleTrait,
};
use crate::high_level::text::Text;
use crate::high_level::{quat_from_angles, Pos, Scale, WHITE};
use crate::input::Handed::Right;
use crate::lifecycle::{StereoKitContext, StereoKitDraw};
use crate::material::{Material, DEFAULT_ID_MATERIAL};
use crate::mesh::Mesh;
use crate::render::RenderLayer;
use crate::shader::Shader;
use crate::values::{vec3_to, Color128};
use crate::StereoKit;
use color_eyre::Result;
use glam::{DQuat, EulerRot, Mat4, Quat, Vec3};
use mint::{EulerAngles, Quaternion};
use std::cell::RefCell;
use std::ops::{Add, AddAssign, Deref, Mul};
use stereokit_sys::{matrix, render_layer_};

pub struct Model {
	pub model: crate::model::Model,
	pub matrix: MatrixContainer,
	pub tint: Color128,
	pub render_layer: RenderLayer,
	pub collider: Option<Collider>,
}

impl Model {
	pub fn from_mesh(sk: &impl StereoKitContext, mesh: &Mesh, material: &Material) -> Result<Self> {
		let model = crate::model::Model::from_mesh(sk, mesh, material)?;
		Ok(Self {
			model,
			matrix: MatrixContainer::new(
				Vec3::default(),
				Vec3::new(0f32, 0f32, 0f32),
				[1f32, 1f32, 1f32],
			),
			tint: WHITE,
			render_layer: RenderLayer::Layer0,
			collider: None,
		})
	}
	pub fn from_memory(
		sk: &impl StereoKitContext,
		file_name: &str,
		memory: &[u8],
		shader: Option<&Shader>,
	) -> Result<Self> {
		let model = crate::model::Model::from_mem(sk, file_name, memory, shader)?;
		Ok(Self {
			model,
			matrix: MatrixContainer::new(
				Vec3::default(),
				Vec3::new(0f32, 0f32, 0f32),
				[1f32, 1f32, 1f32],
			),
			tint: WHITE,
			render_layer: RenderLayer::Layer0,
			collider: None,
		})
	}

	pub fn draw(&self, draw: &StereoKitDraw) {
		self.model
			.draw(draw, self.get_matrix().into(), self.tint, self.render_layer)
	}

	pub fn contains(&self, sk: &impl StereoKitContext, point: Vec3) -> bool {
		let inverted_matrix = self.get_matrix().inverse();
		let new_point = inverted_matrix.transform_point3(point);
		self.get_bounds(sk).bounds_point_contains(new_point.into())
	}
	pub fn collider_intersects(&self, sk: &StereoKitDraw, collider: &Collider) -> bool {
		match collider {
			Collider::CapsuleCollider(c) => self.capsule_intersects(sk, c),
		}
	}
	pub fn capsule_intersects(
		&self,
		sk: &impl StereoKitContext,
		capsule_collider: &CapsuleCollider,
	) -> bool {
		let inverted_matrix = Mat4::from_rotation_translation(
			self.get_matrix().to_scale_rotation_translation().1,
			self.get_matrix().to_scale_rotation_translation().2,
		)
		.inverse();
		let mut pt1 = inverted_matrix.transform_point3(capsule_collider.point1);
		let mut pt2 = inverted_matrix.transform_point3(capsule_collider.point2);
		self.get_bounds(sk)
			.bounds_capsule_contains(pt1.into(), pt2.into(), capsule_collider.radius)
	}
	pub fn get_bounds(&self, sk: &impl StereoKitContext) -> Bounds {
		let mut b = self.model.get_bounds(sk);
		b.center = glam::Vec3::from(b.center).mul(self.get_scale_vec()).into();
		b.dimensions = glam::Vec3::from(b.dimensions)
			.mul(self.get_scale_vec())
			.into();
		b
	}
	pub fn set_collider(&mut self, sk: &impl StereoKitContext, collider: ColliderType) {
		self.collider = Some(Collider::CapsuleCollider(CapsuleCollider::from(sk, self)));
	}
	pub fn get_collider(&mut self, sk: &impl StereoKitContext) -> Option<Collider> {
		if self.collider.is_none() {
			return None;
		}
		self.set_collider(sk, self.collider.unwrap().get_type());
		Some(self.collider.unwrap().clone())
	}
}

impl PosTrait for Model {
	fn get_pos_vec(&self) -> Vec3 {
		self.matrix.get_pos_vec()
	}

	fn set_pos_vec(&mut self, pos: impl Into<crate::values::MVec3>) {
		self.matrix.set_pos_vec(pos)
	}

	fn translate_vec(&mut self, translation: impl Into<crate::values::MVec3>) {
		self.matrix.translate_vec(translation)
	}
}

impl ScaleTrait for Model {
	fn get_scale_vec(&self) -> Vec3 {
		self.matrix.get_scale_vec()
	}

	fn set_scale_vec(&mut self, scale: impl Into<crate::values::MVec3>) {
		self.matrix.set_scale_vec(scale)
	}

	fn scale_vec(&mut self, scale: impl Into<crate::values::MVec3>) {
		self.matrix.scale_vec(scale)
	}
}

impl RotationTrait for Model {
	fn get_rotation_vec(&self) -> Vec3 {
		self.matrix.get_rotation_vec()
	}

	fn set_rotation_vec(&mut self, rotation: impl Into<crate::values::MVec3>) {
		self.matrix.set_rotation_vec(rotation)
	}

	fn rotate_vec(&mut self, rotation: impl Into<crate::values::MVec3>) {
		self.matrix.rotate_vec(rotation)
	}
}
impl MatrixTrait for Model {
	fn get_matrix(&self) -> Mat4 {
		self.matrix.get_matrix()
	}

	fn set_matrix(&mut self, matrix: Mat4) {
		self.matrix.set_matrix(matrix);
		// self.matrix.sync_matrix();
	}
}

#[test]
fn bound_test() {
	let sk = Settings::default().init().unwrap();
	let mesh = &Mesh::gen_cube(&sk, Vec3::new(1f32, 1f32, 1f32), 1).unwrap();
	let material = &Material::copy_from_id(&sk, DEFAULT_ID_MATERIAL).unwrap();
	let mut model = Model::from_mesh(&sk, mesh, material).unwrap();
	model.set_pos(1.1, 0.0, 0.0);
	sk.run(
		|sk, ctx| {
			model.draw(ctx);
			let palm_pos = sk.input_hand(Right).palm.position;
			if model.contains(sk, palm_pos.into()) {
				model.translate(1f32, 0f32, 0f32);
			}
		},
		|_| {},
	);
}

#[test]
fn model_test() {
	let sk = Settings::default().init().unwrap();
	let mesh = &Mesh::gen_cube(&sk, Vec3::new(1f32, 1f32, 1f32), 1).unwrap();
	let material = &Material::copy_from_id(&sk, DEFAULT_ID_MATERIAL).unwrap();
	let mut model = Model::from_mesh(&sk, mesh, material).unwrap();
	model.set_pos_vec([0.1, 0.0, 0.0]);
	let mut red_val = 1f32;
	sk.run(
		|sk, ctx| {
			model.draw(ctx);
			model.rotate(0.0f32, 1f32, 0f32);
			model.scale(0.001f32, 0f32, 0f32);
			model.translate(0f32, 0f32, 0.01f32);
			model.tint.set_red(red_val);
			red_val += 0.0005f32;
		},
		|_| {},
	);
}
