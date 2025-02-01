use bevy::prelude::*;
use rand::Rng;
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


/// save the `use rand::Rng` shenannigans
impl RandomSource {
	/// see [Rng::random]
	pub fn random<T>(&mut self) -> T
	where
		rand::distr::StandardUniform: rand::prelude::Distribution<T>,
	{
		self.0.random()
	}

	/// see [Rng::random_iter]
	pub fn random_iter<T>(
		self,
	) -> rand::distr::Iter<rand::distr::StandardUniform, ChaCha8Rng, T>
	where
		Self: Sized,
		rand::distr::StandardUniform: rand::prelude::Distribution<T>,
	{
		self.0.random_iter()
	}

	/// see [Rng::random_range]
	pub fn random_range<T, R>(&mut self, range: R) -> T
	where
		T: rand::distr::uniform::SampleUniform,
		R: rand::distr::uniform::SampleRange<T>,
	{
		self.0.random_range(range)
	}

	/// see [Rng::random_bool]
	pub fn random_bool(&mut self, p: f64) -> bool { self.0.random_bool(p) }

	/// see [Rng::random_ratio]
	pub fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
		self.0.random_ratio(numerator, denominator)
	}

	/// see [Rng::sample]
	pub fn sample<T, D: rand::prelude::Distribution<T>>(
		&mut self,
		distr: D,
	) -> T {
		self.0.sample(distr)
	}

	/// see [Rng::sample_iter]
	pub fn sample_iter<T, D>(
		self,
		distr: D,
	) -> rand::distr::Iter<D, ChaCha8Rng, T>
	where
		D: rand::prelude::Distribution<T>,
		Self: Sized,
	{
		self.0.sample_iter(distr)
	}

	/// see [Rng::fill]
	pub fn fill<T: rand::Fill + ?Sized>(&mut self, dest: &mut T) {
		self.0.fill(dest)
	}
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
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
