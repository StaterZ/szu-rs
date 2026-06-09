pub use artifact::{Artifact, ArtifactExt};
pub use window_option::{WindowOption, WindowOptionExt};
pub use multi_zip::{MultiZip, MultiZipExt};
pub use multi_zip_array::{MultiZipArray, MultiZipArrayExt};
pub use padded_array_chunks::{PaddedArrayChunks, PaddedArrayChunksExt};
pub use bit_packer::{BitPacker, BitPackerExt};
pub use rle::{Rle, RleExt};
pub use find_last_map_ext::FindLastMapExt;
pub use bins_ext::BinsExt;
pub use multi_zip_array2_ext::MultiZipArray2Ext;
pub use split_by_bytes::SplitByBytes;

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
