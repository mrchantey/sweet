use crate::prelude::terminal;
use anyhow::Result;
use clap::Parser;
use glob::PatternError;
use notify::event::CreateKind;
use notify::event::RemoveKind;
use notify::*;
use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::DebouncedEvent;
use std::num::ParseIntError;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use sweet_utils::prelude::*;

/// A file watcher with glob patterns
#[derive(Debug, Clone, Parser)]
pub struct FsWatcher {
	/// the path to watch
	#[arg(default_value = "./")]
	pub path: PathBuf,
	/// glob for watch patterns
	#[arg(long,value_parser = parse_glob_pattern)]
	pub include: Vec<glob::Pattern>,
	/// glob for ignore patterns
	#[arg(long,value_parser = parse_glob_pattern,default_value="*.git*,*target*")]
	pub exclude: Vec<glob::Pattern>,
	/// debounce time in milliseconds
	#[arg(short,long="debounce-millis",value_parser = parse_duration,default_value="50")]
	pub debounce: Duration,
}

pub fn parse_glob_pattern(s: &str) -> Result<glob::Pattern, PatternError> {
	glob::Pattern::new(s)
}
pub fn parse_duration(s: &str) -> Result<Duration, ParseIntError> {
	s.parse().map(Duration::from_millis)
}

impl Default for FsWatcher {
	fn default() -> Self { Self::parse_from(&[""]) }
}

impl FsWatcher {
	pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
		self.path = path.into();
		self
	}
	pub fn with_watches(mut self, watch: Vec<&str>) -> Self {
		self.include = watch
			.iter()
			.map(|w| glob::Pattern::new(w).unwrap())
			.collect();
		self
	}
	pub fn with_ignores(mut self, ignore: Vec<&str>) -> Self {
		self.exclude = ignore
			.iter()
			.map(|w| glob::Pattern::new(w).unwrap())
			.collect();
		self
	}

	pub fn with_watch(mut self, watch: &str) -> Self {
		self.include.push(glob::Pattern::new(watch).unwrap());
		self
	}
	pub fn with_ignore(mut self, watch: &str) -> Self {
		self.exclude.push(glob::Pattern::new(watch).unwrap());
		self
	}

	pub fn passes(&self, path: &Path) -> bool {
		let path = PathExt::to_forward_slash_str(path);
		self.passes_watch(&path) && self.passes_ignore(&path)
	}

	pub fn passes_watch(&self, path: &str) -> bool {
		self.include.iter().any(|watch| watch.matches(path))
			|| self.include.is_empty()
	}

	pub fn passes_ignore(&self, path: &str) -> bool {
		false == self.exclude.iter().any(|watch| watch.matches(path))
	}
	/// just print the events
	pub async fn watch_log(&self) -> Result<()> {
		self.watch_async(|e| {
			if e.has_mutate() {
				println!("{:?}", e);
			}
			Ok(())
		})
		.await?;
		Ok(())
	}
	pub fn print_globs(&self) {
		terminal::clear().unwrap();
		println!(
			"watching: {}\nignoring: {}\n",
			self.include
				.iter()
				.map(|w| w.as_str())
				.collect::<Vec<_>>()
				.join(","),
			self.exclude
				.iter()
				.map(|w| w.as_str())
				.collect::<Vec<_>>()
				.join(",")
		);
	}

	pub fn watch_blocking(
		&self,
		mut on_change: impl FnMut(WatchEvent) -> Result<()>,
	) -> Result<()> {
		let (tx, rx) = std::sync::mpsc::channel();
		let mut debouncer = new_debouncer(self.debounce, None, move |ev| {
			if let Err(err) = tx.send(ev) {
				eprintln!("{:?}", err);
			}
		})?;
		debouncer.watch(&self.path, RecursiveMode::Recursive)?;
		for ev in rx {
			let ev = WatchEvent::new(ev);
			if ev.any_path(|p| self.passes(p)) {
				on_change(ev)?;
			}
		}
		Ok(())
	}
	pub async fn watch_async(
		&self,
		mut on_change: impl FnMut(WatchEvent) -> Result<()>,
	) -> Result<()> {
		let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
		let mut debouncer = new_debouncer(self.debounce, None, move |ev| {
			if let Err(err) = tx.send(ev) {
				eprintln!("{:?}", err);
			}
		})?;
		debouncer.watch(&self.path, RecursiveMode::Recursive)?;
		while let Some(ev) = rx.recv().await {
			let ev = WatchEvent::new(ev);
			if ev.any_path(|p| self.passes(p)) {
				on_change(ev)?;
			}
		}
		Ok(())
	}
}

/// Wrapper for debounced events,
/// queries are match
#[derive(Debug)]
pub struct WatchEvent {
	pub events: Vec<DebouncedEvent>,
	pub errors: Vec<Error>,
}
impl WatchEvent {
	pub fn new(events: DebounceEventResult) -> Self {
		match events {
			Ok(events) => Self {
				events,
				errors: Vec::new(),
			},
			Err(errors) => Self {
				events: Vec::new(),
				errors,
			},
		}
	}

	pub fn any_path(&self, func: impl FnMut(&PathBuf) -> bool) -> bool {
		self.events.iter().map(|e| &e.paths).flatten().any(func)
	}
	pub fn find_path<O>(
		&self,
		func: impl FnMut(&PathBuf) -> Option<O>,
	) -> Option<O> {
		self.events
			.iter()
			.map(|e| &e.paths)
			.flatten()
			.find_map(func)
	}
	/// is_create || is_modify || is_remove
	pub fn has_mutate(&self) -> bool {
		self.has_create() || self.has_modify() || self.has_remove()
	}
	pub fn mutated(&self) -> Vec<&DebouncedEvent> {
		self.events
			.iter()
			.filter_map(|e| {
				if e.kind.is_create()
					|| e.kind.is_modify()
					|| e.kind.is_remove()
				{
					Some(e)
				} else {
					None
				}
			})
			.collect()
	}

	pub fn mutated_pretty(&self) -> Option<String> {
		let str = self
			.mutated()
			.iter()
			.map(|e| {
				format!(
					"{:?}: {}",
					e.kind,
					e.paths
						.iter()
						.map(|p| p.to_string_lossy())
						.collect::<Vec<_>>()
						.join(",")
				)
			})
			.collect::<Vec<_>>()
			.join("\n");
		if str.is_empty() {
			None
		} else {
			Some(str)
		}
	}

	pub fn has_access(&self) -> bool {
		self.events.iter().any(|e| e.kind.is_access())
	}
	pub fn has_create(&self) -> bool {
		self.events.iter().any(|e| e.kind.is_create())
	}
	pub fn has_create_file(&self) -> bool {
		self.events.iter().any(|e| {
			matches!(e.event.kind, EventKind::Create(CreateKind::File))
		})
	}
	pub fn has_create_dir(&self) -> bool {
		self.events.iter().any(|e| {
			matches!(e.event.kind, EventKind::Create(CreateKind::Folder))
		})
	}
	pub fn has_modify(&self) -> bool {
		self.events.iter().any(|e| e.kind.is_modify())
	}
	pub fn has_remove(&self) -> bool {
		self.events.iter().any(|e| e.kind.is_remove())
	}
	pub fn has_remove_file(&self) -> bool {
		self.events.iter().any(|e| {
			matches!(e.event.kind, EventKind::Remove(RemoveKind::File))
		})
	}
	pub fn has_remove_dir(&self) -> bool {
		self.events.iter().any(|e| {
			matches!(e.event.kind, EventKind::Remove(RemoveKind::Folder))
		})
	}
	pub fn has_other(&self) -> bool {
		self.events.iter().any(|e| e.kind.is_other())
	}
}
