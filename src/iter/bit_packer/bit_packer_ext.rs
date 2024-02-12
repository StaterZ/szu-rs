use super::BitPacker;

pub trait BitPackerExt : Iterator<Item = bool> {
	fn pack_bits(self) -> BitPacker<Self>
	where
		Self: Sized,
	{
		BitPacker::new(self)
	}
}

impl<I: Iterator<Item = bool>> BitPackerExt for I {}
