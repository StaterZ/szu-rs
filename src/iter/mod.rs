pub use artifact::{Artifact, ArtifactExt};
pub use enumerate_artifact::{EnumerateArtifact, EnumerateArtifactExt};
pub use consumed::{Consumed, ConsumedExt};
pub use window_option::{WindowOption, WindowOptionExt};

mod artifact;
mod enumerate_artifact;
mod consumed;
mod window_option;
//mod streaming_enumerate;