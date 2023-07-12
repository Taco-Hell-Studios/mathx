
use crate::{Vector3, Vector2};

/// A builder structure used to create a raycast info struct
#[derive(Debug)]
pub struct RaycastInfoBuilder {
	/// The point of contact
	point: Option<Vector3>,
	/// The normal from the contact
	normal: Option<Vector3>,
	/// The UV of the mesh from the contact
	uv: Option<Vector2>,
	/// The distance from the origin of the ray to the point of contact
	distance: Option<f32>,
	/// Set to true if the ray hit something
	is_hit: bool,
}

/// Constructors
impl RaycastInfoBuilder {
	/// Creates a new raycast information builder
	/// 
	/// **Returns**: Returns a new raycast information builder
	pub fn new() -> Self {
		RaycastInfoBuilder { is_hit: false, point: None, normal: None, uv: None, distance: None }
	}
}

/// Public Methods
impl RaycastInfoBuilder {
	/// Builds the raycast information
	/// 
	/// **Returns**: Returns a newly built raycast information
	pub fn build(self) -> RaycastInfo {
		RaycastInfo {
			is_hit: self.is_hit,
			point: self.point.unwrap_or(Vector3::zero()),
			normal: self.normal.unwrap_or(Vector3::zero()),
			uv: self.uv.unwrap_or(Vector2::zero()),
			distance: self.distance.unwrap_or(0.0),
		}
	}
	
	/// Sets the point of contact for the information
	/// - **value**: The point of contact to set into the information
	/// 
	/// **Returns**: Returns the builder to chain methods together
	pub fn set_point(mut self, value: Vector3) -> Self {
		self.point = Some(value);
		return self;
	}
	
	/// Sets the normal from the point of contact for the information
	/// - **value**: The normal from the point of contact to set into the information
	/// 
	/// **Returns**: Returns the builder to chain methods together
	pub fn set_normal(mut self, value: Vector3) -> Self {
		self.normal = Some(value);
		return self;
	}
	
	/// Sets the distance from the origin of the ray to the point of contact for the information
	/// - **value**: The distance from the origin of the ray to the point of contact to set into the information
	/// 
	/// **Returns**: Returns the builder to chain methods together
	pub fn set_distance(mut self, value: f32) -> Self {
		self.distance = Some(value);
		return self;
	}
	
	/// Sets if the raycast hit anything
	/// - **value**: Set to true to indicate that the raycast hit something
	/// 
	/// **Returns**: Returns the builder to chain methods together
	pub fn set_hit(mut self, value: bool) -> Self {
		self.is_hit = value;
		return self;
	}
}

/// A structure that holds information from a raycast
#[derive(Debug)]
pub struct RaycastInfo {
	/// The point of contact
	point: Vector3,
	/// The normal from the contact
	normal: Vector3,
	/// The UV of the mesh from the contact
	uv: Vector2,
	/// The distance from the origin of the ray to the point of contact
	distance: f32,
	/// Set to true if the ray hit something
	is_hit: bool,
}

/// Constructors
impl RaycastInfo {
	/// Gets an empty raycast information
	/// 
	/// **Returns**: Returns an empty raycast information
	pub fn empty() -> Self {
		RaycastInfo {
			is_hit: false,
			point: Vector3::zero(),
			normal: Vector3::zero(),
			uv: Vector2::zero(),
			distance: 0.0
		}
	}
}

/// Properties
impl RaycastInfo {
	/// Gets the point of contact
	/// 
	/// **Returns**: Returns the point of contact
	pub fn point(&self) -> Vector3 { self.point }
	
	/// Gets the normal from the contact
	/// 
	/// **Returns**: Returns the normal from the contact
	pub fn normal(&self) -> Vector3 { self.normal }
	
	/// Gets the UV of the mesh from the contact
	/// 
	/// **Returns**: Returns the UV of the mesh from the contact
	pub fn uv(&self) -> Vector2 { self.uv }
	
	/// Gets the distance from the origin of the ray to the point of contact
	/// 
	/// **Returns**: Returns the distance from the origin of the ray to the point of contact
	pub fn distance(&self) -> f32 { self.distance }
	
	/// Gets if the ray hit something
	/// 
	/// **Returns**: Returns true if the ray hit something
	pub fn is_hit(&self) -> bool { self.is_hit }
}
