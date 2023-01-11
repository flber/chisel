mod demo;
use chisel::ecs::{Comp, Entity};
use demo::*;

fn main() {
	let pos = Comp::new(PositionComp { x: 10, y: 15 });
	let vel = Comp::new(VelocityComp { dx: 2, dy: 5 });

	let e = Entity::new(0, vec![pos, vel]);
	println!("{}", e.print());
}
