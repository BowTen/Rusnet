
#[derive(Clone, Copy, PartialEq)]
pub struct Segment{
	pub x: u32,
	pub y: u32
}

impl From<(u32, u32)> for Segment {
	fn from(value: (u32, u32)) -> Self {
		Segment{
			x: value.0,
			y: value.1
		}
	}
}
impl From<Segment> for (u32, u32) {
	fn from(value: Segment) -> Self {
		(value.x, value.y)
	}
}
