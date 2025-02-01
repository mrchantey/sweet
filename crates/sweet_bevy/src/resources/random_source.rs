use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

///https://bevyengine.org/examples/math/random-sampling/
#[derive(Resource, Deref, DerefMut)]
pub struct RandomSource(ChaCha8Rng);

impl Default for RandomSource {
	fn default() -> Self {
		let rng = ChaCha8Rng::from_rng(&mut rand::rng());
		// let rng = ChaCha8Rng::seed_from_u64(123412341234);
		Self(rng)
	}
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use rand::Rng;
	use sweet_test::prelude::*;

	#[test]
	fn works() {
		let mut source = RandomSource::default();
		let val = source.random_range(7..10);
		expect(val).to_be_greater_or_equal_to(7);
		expect(val).to_be_less_than(10);
	}
}
