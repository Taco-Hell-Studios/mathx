
use crate::{Ray3, collision::RaycastInfo};

pub trait IRaycast {
	/// Raycasts with the given ray
	/// - **ray**: The ray to raycast with
	/// 
	/// **Returns**: Returns the information on the raycast
	fn raycast(&self, ray: Ray3) -> RaycastInfo;
}
