use crate::tree::TreePosition;
use std::any::Any;
use std::sync::Arc;
use std::sync::RwLock;

pub mod channel_signal;

/// Sweet does not have a reactive system,
/// and instead provides an interface for integrations with reactive libraries.
///
/// ## Reference
/// - [solid - building a reactive library from scratch](https://dev.to/ryansolid/building-a-reactive-library-from-scratch-1i0p)
/// - [leptos - reactive graph](https://book.leptos.dev/appendix_reactive_graph.html)
/// - [reactively - blog post](https://dev.to/milomg/super-charging-fine-grained-reactive-performance-47ph)

// pub trait SignalContext {
// 	fn register_setter(func: Box<dyn Fn(&mut Self)>);

// 	fn register_getter(func: Box<dyn Fn(&mut Self)>);
// }

/// A function that registers a signal.
/// Input values are
/// - the position in the tree
/// - the initial value
type CreateSignal = Box<dyn Send + Sync + Fn(TreePosition, Box<dyn Any>)>;
type CreateEffect = Box<dyn Send + Sync + Fn(TreePosition, Box<dyn Fn()>)>;


pub type SignalGetter = Box<dyn Fn() -> Box<dyn Any> + Send + Sync>;
pub type SignalSetter = Box<dyn Fn(Box<dyn Any>) + Send + Sync>;

pub struct SignalContext {
	pub create_signal: CreateSignal,
	pub create_effect: CreateEffect,
}

impl Default for SignalContext {
	fn default() -> Self {
		SignalContext {
			create_signal: Box::new(|_, _| {
				panic!("signal context has not been set, please call SignalContext::set(my_cx)")
			}),
			create_effect: Box::new(|_, _| {
				panic!("signal context has not been set, please call SignalContext::set(my_cx)")
			}),
		}
	}
}

impl SignalContext {
	/// store this context in the global context, replacing the previous one
	pub fn set(self) { *CTX.write().unwrap() = self; }
}


static CTX: once_cell::sync::Lazy<Arc<RwLock<SignalContext>>> =
	once_cell::sync::Lazy::new(|| {
		Arc::new(RwLock::new(SignalContext::default()))
	});
