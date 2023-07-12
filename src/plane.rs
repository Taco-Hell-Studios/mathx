
use core::ops::Neg;
use crate::Vector3;
#[cfg(not(feature = "no_rays"))]
use crate::{Ray3, Math, interfaces::IRaycast, collision::{RaycastInfo, RaycastInfoBuilder}};

/// A struct that represents a 3D plane
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Plane {
	/// The normal perpendicular to the plane
	normal: Vector3,
	/// The distance from origin, up towards the normal where the plane lies
	distance: f32,
}

/// Constructors
impl Plane {
	/// Create a new 3D plane
	/// - **normal**: The normal perpendicular to the plane
	/// - **distance**: The distance from origin, up towards the normal where the plane lies
	/// 
	/// **Returns**: Returns a new 3D plane
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::new(Vector3::one(), 1.0);
	/// assert_eq!(0.57735026 * Vector3::one(), plane.normal());
	/// assert_eq!(1.0, plane.distance());
	/// ```
	pub fn new(normal: Vector3, distance: f32) -> Self {
		Plane {
			normal: normal.normalize(),
			distance,
		}
	}
	
	/// Creates a new 3D plane from a normal and a given point
	/// - **normal**: The normal perpendicular to the plane
	/// - **point**: The point on the plane
	/// 
	/// **Returns**: Returns a new 3D plane from a normal and a given point
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane, Math, assert_range};
	/// let plane = Plane::new_from_point(Vector3::one(), Vector3::new(-1.0, 0.5, 2.5));
	/// assert_eq!(0.57735026 * Vector3::one(), plane.normal());
	/// assert_range!(-1.1547005, plane.distance());
	/// ```
	pub fn new_from_point(normal: Vector3, point: Vector3) -> Self {
		let normal = normal.normalize();
		Plane {
			normal,
			distance: -(normal * point),
		}
	}
	
	/// Creates a new 3D plane from 3 separate points
	/// - **a**: The first point used to triangulate the plane
	/// - **b**: The second point used to triangulate the plane
	/// - **c**: The third point used to triangulate the plane
	/// 
	/// **Returns**: Returns a new 3D plane from 3 separate points
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane, Math, assert_range};
	/// let plane = Plane::new_triangulated(
	/// 	Vector3::new(0.0, 0.2, 0.4),
	/// 	Vector3::new(0.6, 0.8, 1.0),
	/// 	Vector3::new(0.3, 0.6, -0.9)
	/// );
	/// assert_eq!(Vector3::new(-0.7275328, 0.6847368, 0.04279606), plane.normal());
	/// assert_range!(-0.1540658, plane.distance());
	/// ```
	pub fn new_triangulated(a: Vector3, b: Vector3, c: Vector3) -> Self {
		let p = b - a;
		let q = c - a;
		let normal = p.cross(q).normalize();
		
		Plane {
			normal,
			distance: -(normal * a),
		}
	}
	
	/// Creates a plane that spans the X and Y axis
	/// 
	/// **Returns**: Returns a plane that spans the X and Y axis
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::xy_plane();
	/// assert!(plane.is_on_plane(Vector3::new(100.0, -100.0, 0.0)));
	/// ```
	pub fn xy_plane() -> Self { Plane::new(Vector3::forward(), 0.0) }
	
	/// Creates a plane that spans the X and Z axis
	/// 
	/// **Returns**: Returns a plane that spans the X and Z axis
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::xz_plane();
	/// assert!(plane.is_on_plane(Vector3::new(1.0, 0.0, 2.0)));
	/// ```
	pub fn xz_plane() -> Self { Plane::new(Vector3::up(), 0.0) }
	
	/// Creates a plane that spans the Y and Z axis
	/// 
	/// **Returns**: Returns a plane that spans the Y and Z axis
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::yz_plane();
	/// assert!(plane.is_on_plane(Vector3::new(0.0, -10.0, 10.0)));
	/// ```
	pub fn yz_plane() -> Self { Plane::new(Vector3::right(), 0.0) }
}

/// Properties
impl Plane {
	/// Gets the normal of the plane
	/// 
	/// **Returns**: Returns the normal of the plane
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::new(Vector3::one(), 1.0);
	/// assert_eq!(0.57735026 * Vector3::one(), plane.normal());
	/// ```
	pub fn normal(&self) -> Vector3 { self.normal }
	
	/// Sets the normal of the plane
	/// - **value**: The normal to set the plane
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let mut plane = Plane::new(Vector3::down(), 1.0);
	/// plane.set_normal(Vector3::one());
	/// assert_eq!(0.57735026 * Vector3::one(), plane.normal());
	/// ```
	pub fn set_normal(&mut self, value: Vector3) { self.normal = value.normalize(); }
	
	/// Gets the distance up the normal of the plane
	/// 
	/// **Returns**: Returns the distance up the normal of the plane
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::new(Vector3::down(), 1.0);
	/// assert_eq!(1.0, plane.distance());
	/// ```
	pub fn distance(&self) -> f32 { self.distance }
	
	/// Sets the distance up the normal of the plane
	/// - **value**: The value to set the distance for
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let mut plane = Plane::new(Vector3::down(), 1.0);
	/// plane.set_distance(2.0);
	/// assert_eq!(2.0, plane.distance());
	/// ```
	pub fn set_distance(&mut self, value: f32) { self.distance = value; }
	
	/// Flips the plane to the opposite direction
	/// 
	/// **Returns**: Returns the flipped plane
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::new(Vector3::one(), 1.0).flipped();
	/// assert_eq!(-0.57735026 * Vector3::one(), plane.normal());
	/// assert_eq!(-1.0, plane.distance());
	/// ```
	pub fn flipped(self) -> Self { Plane::new(-self.normal, -self.distance) }
}

/// Public Methods
impl Plane {
	/// Finds if the point is on the plane
	/// - **point**: The point to check with
	/// 
	/// **Returns**: Returns true if the point is on the plane
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::yz_plane();
	/// assert!(plane.is_on_plane(Vector3::new(0.0, -10.0, 10.0)));
	/// ```
	pub fn is_on_plane(&self, point: Vector3) -> bool { Math::approx((self.normal * point) + self.distance, 0.0) }
	
	/// Gets the closest point on the plane from the given point
	/// - **point**: The point to find the closest point on the plane with
	/// 
	/// **Returns**: Returns the closest point on the plane from the given point
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::new(Vector3::new(1.0, -2.0, 3.0), 3.0);
	/// let point = plane.closest_point(Vector3::one());
	/// assert_eq!(Vector3::new(0.05535913, 2.889282, -1.833922), point);
	/// ```
	pub fn closest_point(self, point: Vector3) -> Vector3 {
		point - self.normal * self.distance_to_point(point)
	}
	
	/// Gets the distance from the point to the plane
	/// - **point**: The point to find the distance from the plane
	/// 
	/// **Returns**: Returns the distance from the point to the plane
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane, Math, assert_range};
	/// let plane = Plane::new(Vector3::new(1.0, -2.0, 3.0), 3.0);
	/// let distance = plane.distance_to_point(Vector3::one());
	/// assert_range!(3.534523, distance)
	/// ```
	pub fn distance_to_point(self, point: Vector3) -> f32 { (self.normal * point) + self.distance }
	
	/// Finds if the point is on the positive side of the plane
	/// - **point**: The point to find the if it's on the positive side of the plane
	/// 
	/// **Returns**: Returns true if the point is on the positive side of the plane
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::new(Vector3::new(1.0, -2.0, 3.0), 3.0);
	/// assert!(plane.is_on_positive_side(Vector3::one()));
	/// ```
	pub fn is_on_positive_side(&self, point: Vector3) -> bool {
		self.distance_to_point(point) > 0.0
	}
	
	/// Finds if the two given points are on the same side of the plane
	/// - **a**: The first point to query with
	/// - **b**: The second point to query with
	/// 
	/// **Returns**: Returns true if both points are on the same side of the plane
	/// #### Examples
	/// ```
	/// # use mathx::{Vector3, Plane};
	/// let plane = Plane::new(Vector3::new(1.0, -2.0, 3.0), 3.0);
	/// let a = Vector3::one();
	/// let b = Vector3::right();
	/// let c = Vector3::new(-10.0, -20.0, -30.0);
	/// assert!(plane.is_on_same_side(a, b));
	/// assert!(!plane.is_on_same_side(a, c));
	/// ```
	pub fn is_on_same_side(&self, a: Vector3, b: Vector3) -> bool {
		self.is_on_positive_side(a) == self.is_on_positive_side(b)
	}
}

#[cfg(not(feature = "no_rays"))]
impl IRaycast for Plane {
	/// Raycasts with the given ray
	/// - **ray**: The ray to raycast with
	/// 
	/// **Returns**: Returns the information on the raycast
	fn raycast(&self, ray: Ray3) -> RaycastInfo {
		let diff = ray.direction().dot(self.normal);
		let dist = -(ray.origin().dot(self.normal) + self.distance);
		
		if Math::approx(diff, 0.0) {
			return RaycastInfo::empty();
		}
		
		let distance = dist / diff;
		
		return RaycastInfoBuilder::new()
			.set_hit(distance > 0.0)
			.set_distance(distance)
			.set_normal(self.normal)
			.set_point(ray.get_point(distance))
			.build();
	}
}

unsafe impl Send for Plane {}
unsafe impl Sync for Plane {}

impl Eq for Plane {}
impl PartialEq for Plane {
	fn eq(&self, other: &Self) -> bool {
		self.normal == other.normal
		&& self.distance == other.distance
	}
}


#[cfg(not(feature = "no_std"))]
impl std::fmt::Display for Plane {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("normal: {}, distance: {}", self.normal, self.distance))
	}
}


impl Neg for Plane {
	type Output = Plane;
	fn neg(self) -> Self::Output { self.flipped() }
}
