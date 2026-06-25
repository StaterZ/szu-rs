pub use artifact::*;
pub use window_option::*;
pub use multi_zip::*;
pub use multi_zip_array::*;
pub use padded_array_chunks::*;
pub use bit_packer::*;
pub use rle::*;
pub use find_last_map_ext::*;
pub use bins_ext::*;
pub use multi_zip_array2_ext::*;
pub use split_by_bytes::*;
pub use peekable_iter::*;

mod artifact;
mod window_option;
mod multi_zip;
mod multi_zip_array;
mod padded_array_chunks;
mod bit_packer;
mod rle;
mod find_last_map_ext;
mod bins_ext;
mod multi_zip_array2_ext;
mod split_by_bytes;
mod peekable_iter;

pub trait IteratorExt: Iterator {
	fn min_by_key_while<K, F>(self, mut f: F) -> Option<Self::Item>
	where
		Self: Sized,
		K: Ord,
		F: FnMut(&Self::Item) -> Option<K>,
	{
		let mut min_item: Option<Self::Item> = None;
		let mut min_key: Option<K> = None;

		for item in self {
			let key = match f(&item) {
				Some(k) => k,
				None => return None,
			};

			if min_key.as_ref().map_or(true, |mk| key < *mk) {
				min_key = Some(key);
				min_item = Some(item);
			}
		}

		min_item
	}
}

impl<I: Iterator> IteratorExt for I {}
