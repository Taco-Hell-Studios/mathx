
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum InterpolationType {
	NoLoop = 0,
	NoLoopBackwards = 1,
	FullLoop = 2,
	FullLoopBackwards = 3,
	YoyoLoop = 4,
	YoyoLoopBackwards = 5,
}
