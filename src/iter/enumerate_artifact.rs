use std::iter::Enumerate;
use super::{Artifact, ArtifactExt};

pub type EnumerateArtifact<I> = Artifact<
	Enumerate<I>,
	fn(&<Enumerate<I> as Iterator>::Item) -> usize,
	usize,
>;

pub trait EnumerateArtifactExt : Iterator {
	fn enumerate_artifact(self: Self) -> EnumerateArtifact<Self>
	where
		Self: Sized
	{
		self.enumerate().artifact(|(i, _)| *i)
	}
}

impl<I: Iterator> EnumerateArtifactExt for I {}
