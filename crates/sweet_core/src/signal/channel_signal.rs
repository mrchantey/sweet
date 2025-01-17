#![allow(unused)]
use super::SignalContext;
use crate::tree::TreePosition;
use std::any::Any;
use std::sync::Arc;
use std::sync::RwLock;

type Message = (usize, Box<dyn Any + Send>);

struct ChannelSignal {
	send: flume::Sender<Message>,
	receive: flume::Receiver<Message>,
	effects: Vec<Box<dyn Fn() + Send + Sync>>,
}


static CTX: once_cell::sync::Lazy<Arc<RwLock<ChannelSignal>>> =
	once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(ChannelSignal::new())));

impl ChannelSignal {
	pub fn new() -> Self {
		let (send, receive) = flume::unbounded();
		Self {
			send,
			receive,
			effects: Vec::new(),
		}
	}

	pub fn set(self) {
		let (send, receive) =
			flume::unbounded::<(TreePosition, Box<dyn Any + Send>)>();

		// *CTX.write().unwrap() = self;
		SignalContext {
			create_signal: Box::new(|_, _| {
				panic!("signal context has not been set, please call SignalContext::set(my_cx)")
			}),
			create_effect: Box::new(|_, _| {
				panic!("signal context has not been set, please call SignalContext::set(my_cx)")
			}),
		}.set();
	}

	pub fn signal<T: Clone>(initial: T) -> (impl Fn() -> T, impl Fn(T)) {
		let val = Arc::new(RwLock::new(initial));
		let val2 = val.clone();
		let read = move || val.read().unwrap().clone();
		let write = move |new: T| {
			let mut val = val2.write().unwrap();
			*val = new;
		};
		(read, write)
	}
}

pub fn signal<T: Clone>(initial: T) -> (impl Fn() -> T, impl Fn(T)) {
	let val = Arc::new(RwLock::new(initial));
	let val2 = val.clone();
	let read = move || val.read().unwrap().clone();
	let write = move |new: T| {
		let mut val = val2.write().unwrap();
		*val = new;
	};
	(read, write)
}

pub fn effect<T: Clone>(func: impl Fn(T)) {}



#[cfg(test)]
mod test {
	use crate::prelude::*;
	use channel_signal::*;

	#[test]
	fn works() {
		let (read, write) = signal(0);
		expect(read()).to_be(0);
		write(1);
		expect(read()).to_be(1);
	}
}
