#![feature(panic_payload_as_str)]
#![feature(thread_id_value)]
use rayon::prelude::*;
use std::cell::Cell;
use std::panic::PanicHookInfo;
use std::panic::{
	self,
};
use std::sync::Arc;
use std::thread;
use thread_local::ThreadLocal;



#[tokio::main]
async fn main() {
	let per_task_info = Arc::new(ThreadLocal::<Cell<String>>::new());

	let (send, recv) = flume::unbounded();
	let per_task_info2 = per_task_info.clone();
	panic::set_hook(Box::new(move |_panic_info: &PanicHookInfo| {
		let val = per_task_info2.get().expect("hook tls dropped");
		send.send(val.take()).unwrap();
	}));

	let recv2 = recv.clone();
	let recv_handle = tokio::spawn(async move {
		while let Ok(val) = recv2.recv_async().await {
			println!("{}", val);
		}
		println!("dropped");
	});

	// stop the abort
	std::panic::catch_unwind(|| {
		(0..60).into_par_iter().for_each_with(
			per_task_info.clone(),
			|tls, i| {
				println!("running {}", i);
				let thread_id = thread::current().id();
				let cell = tls.get_or(|| Cell::new(String::new()));
				cell.set(format!(
					"task {} ran on thread {}",
					i,
					thread_id.as_u64()
				));
				panic!("who cares");
			},
		);
	})
	.ok();
	let _hook = std::panic::take_hook();
	recv_handle.abort();
	println!("all finished");
}
