/// Basically a `FnOnce` trait, but not nightly and a little less awkward to implement.
pub trait Pipeline<In, Out = In> {
	/// Consume self and apply to the target
	fn apply(self, value: In) -> Out;
}

impl<F, In, Out> Pipeline<In, Out> for F
where
	F: FnOnce(In) -> Out,
{
	fn apply(self, value: In) -> Out { self(value) }
}


/// Utilities for method-chaining on any type.
/// Very similar in its goals to [`tap`](https://crates.io/crates/tap)
pub trait PipelineTarget: Sized {
	/// its like map but for any type
	fn xmap<O>(self, func: impl FnOnce(Self) -> O) -> O { func(self) }
	/// its like inpsect but for any type
	fn xtap(mut self, func: impl FnOnce(&mut Self)) -> Self {
		func(&mut self);
		self
	}
	fn xdebug(self) -> Self
	where
		Self: std::fmt::Debug,
	{
		println!("{:?}", self);
		self
	}
	fn xdisplay(self) -> Self
	where
		Self: std::fmt::Display,
	{
		println!("{}", self);
		self
	}
	fn xtap_mut(&mut self, func: impl FnOnce(&mut Self)) -> &mut Self {
		func(self);
		self
	}
	/// its like map but for our pipeline trait
	fn xpipe<P: Pipeline<Self, O>, O>(self, pipeline: P) -> O {
		pipeline.apply(self)
	}

	fn xref(&self) -> &Self { self }
	fn xok<E>(self) -> Result<Self, E>
	where
		Self: Sized,
	{
		Ok(self)
	}
}
impl<T: Sized> PipelineTarget for T {}
