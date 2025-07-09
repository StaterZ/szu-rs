use std::ops::Deref;

pub type OptOwnStr<'s> = OptOwn<'s, String, str>;

pub enum OptOwn<'s, O: Deref<Target = B>, B: ?Sized> {
	Owned(O),
	Ref(&'s B),
}

impl<'s, O: Deref<Target = B>, B: ?Sized> Deref for OptOwn<'s, O, B>{
	type Target = B;

	fn deref(&self) -> &Self::Target {
		match self {
			OptOwn::Ref(r) => r,
			OptOwn::Owned(o) => o.deref(),
		}
	}
}

impl<'s, O: Deref<Target = B>, B: ?Sized> From<&'s B> for OptOwn<'s, O, B> {
	fn from(value: &'s B) -> Self {
		Self::Ref(value)
	}
}

impl<'s> From<String> for OptOwnStr<'s> {
	fn from(value: String) -> Self {
		Self::Owned(value)
	}
}

impl<'s, B: ?Sized> From<Box<B>> for OptOwn<'s, Box<B>, B> {
	fn from(value: Box<B>) -> Self {
		Self::Owned(value)
	}
}
