pub mod ecs {
	use std::{
		cell::RefCell,
		rc::{Rc, Weak},
	};

	pub struct Entity {
		pub id: usize,
		pub components: Vec<RefCell<Comp>>,
	}

	impl Entity {
		pub fn new(id: usize, comps: Vec<Comp>) -> Rc<Entity> {
			let mut e = Entity {
				id,
				components: Vec::new(),
			};
			for comp in &comps {
				e.components.push(RefCell::new(Comp(Rc::clone(&comp.0))));
			}
			let rc_e = Rc::new(e);

			for comp in comps {
				*comp.0.parent.borrow_mut() = Rc::downgrade(&rc_e);
			}

			rc_e
		}

		pub fn add(&self, comp: Comp) {
			self.components.push(RefCell::new(Comp(Rc::clone(&comp.0))));
			let rc_e = Rc::new(self);
			*comp.0.parent.borrow_mut() = Rc::downgrade(&rc_e);
		}

		pub fn print(&self) -> String {
			let mut text = format!("Entity {}:\n", self.id);
			for component in &self.components {
				text = vec![text, format!("{}\n", component.borrow().print())].concat();
			}
			text
		}
	}

	pub struct Comp(Rc<ComponentWrapper>);
	impl Comp {
		pub fn new<T>(c: T) -> Comp
		where
			T: Component + 'static,
		{
			Comp(Rc::new(ComponentWrapper::new(Box::new(c))))
		}

		fn print(&self) -> String {
			self.0.print()
		}
	}

	pub struct ComponentWrapper {
		pub parent: RefCell<Weak<Entity>>,
		pub component: Box<dyn Component>,
	}

	impl ComponentWrapper {
		fn new(c: Box<dyn Component>) -> ComponentWrapper {
			ComponentWrapper {
				parent: RefCell::new(Weak::new()),
				component: c,
			}
		}

		fn print(&self) -> String {
			format!("{}", self.component.print())
		}
	}

	pub trait Component {
		fn print(&self) -> String;
	}
}

#[cfg(test)]
mod tests {}
