use super::Consumed;

pub trait ConsumedExt : Iterator {
	fn consumed(self) -> Consumed<Self>
	where
		Self: Sized
	{
		Consumed::new(self)
	}
}

impl<I: Iterator> ConsumedExt for I {}
