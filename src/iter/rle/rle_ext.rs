use super::Rle;

pub trait RleExt : Iterator<Item: Eq> {
	fn rle(self) -> Rle<Self>
	where
		Self: Sized
	{
		Rle::new(self.peekable())
	}
}

impl<I: Iterator<Item: Eq>> RleExt for I {}
