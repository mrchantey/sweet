use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// A simple random source, by default retrieved from entropy.
///
/// ```rust
/// # use bevy::prelude::*;
/// # use sweet_bevy::prelude::*;
/// # use rand::Rng;
///
/// // defaults to from entropy
/// let mut source = RandomSource::default();
/// // or from a seed
/// let mut source = RandomSource::from_seed(7);
/// App::new()
/// 	.insert_resource(source)
/// 	.add_systems(Update,use_source);
///
///
/// fn use_source(mut source: ResMut<RandomSource>) {
/// 	println!("Random number: {}", source.random_range(1..1000));
/// }
/// ```
///https://bevyengine.org/examples/math/random-sampling/
#[derive(Resource, Deref, DerefMut)]
pub struct RandomSource(ChaCha8Rng);

impl RandomSource {
	pub fn from_seed(seed: u64) -> Self {
		let rng = ChaCha8Rng::seed_from_u64(seed);
		Self(rng)
	}
}

impl Default for RandomSource {
	fn default() -> Self {
		let rng = ChaCha8Rng::from_rng(&mut rand::rng());
		Self(rng)
	}
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use rand::Rng;
	use sweet_test::prelude::*;


	#[test]
	fn seed() {
		let mut source = RandomSource::from_seed(7);
		let val = source.random_range(10..100);
		expect(val).to_be(22);
	}

	#[test]
	fn entropy() {
		let mut source = RandomSource::default();
		let val = source.random_range(10..100);
		expect(val).to_be_greater_or_equal_to(10);
		expect(val).to_be_less_than(100);
	}
}
