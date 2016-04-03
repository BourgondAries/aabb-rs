#![feature(btree_range, collections_bound)]
extern crate fps_counter;
extern crate sfml;

mod float_order;
mod handle_events;
mod once_in;
mod person;
mod setup_ground;
mod setup_window;
mod test;

use float_order::Float;
use once_in::OnceIn;
use sfml::window::{ContextSettings, Key, event, window_style};
use sfml::graphics::{RenderWindow, Shape, RenderTarget, Color, Transformable, View};
use std::collections::Bound;
use person::Person;

fn main() {
	let (mut window, mut view) = setup_window::setup();
	let mut map = setup_ground::setup();
	let mut once_in = OnceIn::new(20);

	println!("Created a 1-DTree containing: {} elements", map.len());

	let mut person = Person::new();
	let mut fpscnt = fps_counter::FPSCounter::new();

	while window.is_open() {
		{
			let fps = fpscnt.tick();
			if once_in.count() {
				println!("fps: {}, pos: {}", fps, person.getX().0);
			}
		}
		handle_events::handle_events(&mut window, &mut view, &mut person);

		for element in map.range_mut(
				Bound::Included(
					&Float(person.person.get_position().x - 800.0)),
				Bound::Included(
					&Float(person.person.get_position().x + 800.0))) {
			element.1.set_fill_color(&Color::new_rgb(127u8, 127u8, 127u8));
		}
		person.simulate();
		for element in map.range_mut(
				Bound::Included(&person.getX()), Bound::Included(&person.getX2())) {
			element.1.set_fill_color(&Color::new_rgb(255u8, 255u8, 0u8));
			person.collide(element.1);
		}

		view.set_center2f(person.person.get_position().x,
			person.person.get_position().y);
		window.set_view(&view);

		window.clear(&Color::new_rgb(0, 0, 0));
		for element in map.range(
			Bound::Included(
				&Float(person.person.get_position().x - 800.0)),
			Bound::Included(
				&Float(person.person.get_position().x + 800.0))) {
			window.draw(element.1);
		}
		window.draw(&person);
		window.display();
	}
}

