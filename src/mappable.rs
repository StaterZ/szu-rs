pub trait Mappable<T> {
	type Target<U>;

	fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Self::Target<U>;
}

impl<T> Mappable<T> for Option<T> {
	type Target<U> = Option<U>;

	fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Self::Target<U> {
		self.map(f)
	}
}

impl<T, E> Mappable<T> for Result<T, E> {
	type Target<U> = Result<U, E>;

	fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Self::Target<U> {
		self.map(f)
	}
}
