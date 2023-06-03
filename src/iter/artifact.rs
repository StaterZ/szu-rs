use std::iter::Enumerate;

pub struct Artifact<I, P, A>
where
	I: Iterator,
	P: Fn(&I::Item) -> A,
{
	iter: I,
	pred: P,
	artifact: Option<A>,
}

impl<I, P, A> Artifact<I, P, A>
where
	I: Iterator,
	P: Fn(&I::Item) -> A,
{
	pub fn new(iter: I, pred: P) -> Self {
		Self {
			iter,
			artifact: None,
			pred,
		}
	}

	pub fn get_artifact(&self) -> &Option<A> {
		&self.artifact
	}
}

impl<I, P, A> Iterator for Artifact<I, P, A>
where
	I: Iterator,
	P: Fn(&I::Item) -> A,
{
	type Item = I::Item;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		self.iter.next().map(|item| {
			self.artifact = Some((self.pred)(&item));
			item
		})
	}
}

impl<I, P, A> Clone for Artifact<I, P, A>
where
	I: Clone + Iterator,
	P: Clone + Fn(&I::Item) -> A,
	A: Clone,
{
	fn clone(&self) -> Self {
		Self { 
			iter: self.iter.clone(),
			pred: self.pred.clone(),
			artifact: self.artifact.clone(),
		}
	}
}

pub type EnumerateArtifact<I> = Artifact<
	Enumerate<I>,
	fn(&<Enumerate<I> as Iterator>::Item) -> usize,
	usize,
>;

pub trait ArtifactExt : Iterator {
	fn artifact<P, A>(self, p: P) -> Artifact<Self, P, A>
	where
		Self: Sized,
		P: Fn(&Self::Item) -> A,
		A: Sized,
	{
		Artifact::new(self, p)
	}

	fn enumerate_artifact(self) -> EnumerateArtifact<Self>
	where
		Self: Sized
	{
		self.enumerate().artifact(|(i, _)| *i)
	}
}

impl<I: Iterator> ArtifactExt for I {}
