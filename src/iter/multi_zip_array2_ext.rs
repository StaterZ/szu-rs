pub trait MultiZipArray2Ext<T, const N_X: usize, const N_Y: usize> {
	fn multi_zip_array2(self) -> [[T; N_Y]; N_X];
}

impl<T: Clone, const N_X: usize, const N_Y: usize> MultiZipArray2Ext<T, N_X, N_Y> for [[&T; N_X]; N_Y] {
	fn multi_zip_array2(self) -> [[T; N_Y]; N_X] {
		std::array::from_fn(|x|
			std::array::from_fn(|y|
				self[y][x].clone()))
	}
}
