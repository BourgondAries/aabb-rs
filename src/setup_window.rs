use sfml::window::{ContextSettings, VideoMode, window_style};
use sfml::graphics::{RenderWindow, RenderTarget, View};

pub fn setup() -> (RenderWindow, View)  {
	struct Size(u32, u32);
	let size = Size(800, 600);

	let window = match RenderWindow::new(VideoMode::new_init(size.0, size.1, 32),
		"ln(n) collision detection",
		window_style::CLOSE,
		&ContextSettings::default()
	) {
		Some(window) => window,
		None => panic!("Cannot create a new Render Window.")
	};
	let view = window.get_view();

	(window, view)
}
