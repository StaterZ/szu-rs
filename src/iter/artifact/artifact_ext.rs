
use super::Artifact;
pub trait ArtifactExt : Iterator {
	fn artifact<P, A>(self, pred: P) -> Artifact<Self, P, A>
	where
		Self: Sized,
		P: Fn(&Self::Item) -> A,
		A: Sized,
	{
		Artifact::new(self, pred)
	}
}

impl<I: Iterator> ArtifactExt for I {}
