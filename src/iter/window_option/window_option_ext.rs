use super::WindowOption;

pub trait WindowOptionExt<I: Iterator> {
	fn window_option(self) -> WindowOption<I>;
}

impl<I: Iterator> WindowOptionExt<I> for I {
	fn window_option(self) -> WindowOption<I> {
		WindowOption::new(self)
	}
}
