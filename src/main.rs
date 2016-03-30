#![feature(btree_range, collections_bound)]
extern crate sfml;

mod float_order;
mod handle_events;
mod person;
mod setup_ground;
mod setup_window;
mod test;

use float_order::Float;
use sfml::window::{ContextSettings, Key, event, window_style};
use sfml::graphics::{RenderWindow, Shape, RenderTarget, Color, Transformable, View};
use std::collections::Bound;
use person::Person;

fn main() {

	let (mut window, mut view) = setup_window::setup();
	let mut map = setup_ground::setup();

	let mut person = Person::new();

	while window.is_open() {
		handle_events::handle_events(&mut window, &mut view, &mut person);

		for element in &mut map {
			element.1.set_fill_color(&Color::new_rgb(127u8, 127u8, 127u8));
		}
		person.simulate();
		for element in map.range_mut(
				Bound::Included(&person.getX()), Bound::Included(&person.getX2())) {
			element.1.set_fill_color(&Color::new_rgb(255u8, 255u8, 0u8));
			person.collide(element.1);
		}

		window.clear(&Color::new_rgb(0, 0, 0));
		for (_, drawable) in &map {
			window.draw(drawable);
		}
		window.draw(&person);
		window.display()
	}
}

