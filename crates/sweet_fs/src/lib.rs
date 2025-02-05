pub mod fs;
pub mod process;
pub mod terminal;

pub mod prelude {
	pub use crate::fs::*;
	pub use crate::process::*;
	pub use crate::terminal;
	pub use notify::event::AccessKind;
	pub use notify::event::CreateKind;
	pub use notify::event::ModifyKind;
	pub use notify::event::RemoveKind;
	pub use notify::Event as FsEvent;
	pub use notify::EventKind;
	pub use notify_debouncer_full::DebouncedEvent;
}
