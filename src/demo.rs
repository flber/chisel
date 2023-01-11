use chisel::ecs::Component;

#[derive(Debug)]
pub struct PositionComp {
	pub x: i32,
	pub y: i32,
}
impl Component for PositionComp {
	fn print(&self) -> String {
		format!("{:#?}", self)
	}
}

#[derive(Debug)]
pub struct VelocityComp {
	pub dx: i32,
	pub dy: i32,
}
impl Component for VelocityComp {
	fn print(&self) -> String {
		format!("{:#?}", self)
	}
}
