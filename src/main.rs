#![feature(btree_range, collections_bound)]
extern crate sfml;

mod float_order;
mod handle_events;
mod setup_ground;
mod setup_window;
mod test;

use float_order::Float;
use sfml::window::{ContextSettings, Key, event, window_style};
use sfml::graphics::{RenderWindow, Shape, RenderTarget, Color, Transformable, View};
use std::collections::Bound;

fn main() {

	let (mut window, mut view) = setup_window::setup();
	let mut map = setup_ground::setup();
	// map.get_mut(&Float(60.0)).unwrap().set_fill_color(&Color::new_rgb(
	//	0u8, 0u8, 0u8));

	// map.range_mut(
	//	 Bound::Included(&Float(50.0)), Bound::Included(&Float(120.0))
	// );

	while window.is_open() {
		handle_events::handle_events(&mut window, &mut view);
		window.clear(&Color::new_rgb(0, 0, 0));
		for (_, drawable) in &map {
			window.draw(drawable);
		}
		window.display()
	}
}

