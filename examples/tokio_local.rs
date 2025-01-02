#![feature(panic_payload_as_str)]
use futures::FutureExt;
use std::panic;
use std::panic::PanicHookInfo;

tokio::task_local! {
	static TASK_INFO: String;
}

#[tokio::main]
async fn main() {
	let (send, recv) = flume::unbounded();

	let send2 = send.clone();
	panic::set_hook(Box::new(move |_panic_info: &PanicHookInfo| {
		let send = send2.clone();
		if let Some(task_info) = TASK_INFO.try_with(|info| info.clone()).ok() {
			send.send(task_info).unwrap_or_default();
		}
	}));

	let recv_handle = tokio::spawn(async move {
		while let Ok(val) = recv.recv_async().await {
			println!("Received panic info: {}", val);
		}
		println!("Receiver dropped");
	});

	let handles: Vec<_> = (0..100)
		.map(|i| {
			async move {
				TASK_INFO
					.scope(format!("task {}", i), async {
						println!("Running task {}", i);
						panic!("blabla");
					})
					.await
			}
			.catch_unwind()
		})
		.collect();

	let _results = futures::future::join_all(handles).await;

	// Wait for all tasks to complete or panic
	// for handle in handles {
	// 	let _ = handle.await;
	// }

	// Cleanup
	let _hook = panic::take_hook();
	recv_handle.abort();
	println!("All tasks finished");
}
