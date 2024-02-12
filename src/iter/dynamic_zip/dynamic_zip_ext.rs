use itertools::Itertools;

use super::DynamicZip;

pub trait DynamicZipExt : Iterator {
	fn dynamic_zip<I, T>(self) -> DynamicZip<I, T> where
		Self: Sized + Iterator<Item = I>,
		I: Iterator<Item = T>,
	{
		DynamicZip::new(self.collect_vec())
	}
}

impl<I: Iterator> DynamicZipExt for I {}
