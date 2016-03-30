use float_order::Float;
use sfml::graphics::{Drawable, RectangleShape, RenderTarget, RenderStates, Transformable};

pub struct Person<'a> {
	person: RectangleShape<'a>,
	pub speed: (f32, f32)
}

impl<'a> Person<'a> {
	pub fn new() -> Person<'a> {
		let mut shape = RectangleShape::new().unwrap();
		shape.set_size2f(20.0, 20.0);
		Person {
			person: shape,
			speed: (0.0, 0.0)
		}
	}
	pub fn simulate(&mut self) {
		self.person.move2f(self.speed.0, self.speed.1);
		self.speed.1 += 0.001;
	}

	pub fn getX(&self) -> Float {
		Float(self.person.get_position().x - 20.0)
	}

	pub fn getX2(&self) -> Float {
		Float(self.person.get_position().x + 20.0)
	}

	pub fn collide(&mut self, shape: &RectangleShape<'a>) {
		if self.person.get_position().y > (400.0 - 20.0) {
			let pos_x = self.person.get_position().x;
			self.speed.1 = 0.0;
			self.person.set_position2f(pos_x, (400.0 - 20.0));
		}
	}
}

impl<'a> Drawable for Person<'a> {
	fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
		target.draw(&self.person);
	}
}
