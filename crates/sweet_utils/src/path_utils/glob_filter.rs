use clap::Parser;
use glob::PatternError;
use std::path::Path;


/// glob for watch patterns
#[derive(Default, Clone, Parser)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlobFilter {
	/// glob for watch patterns, leave empty to include all
	#[arg(long, value_parser = parse_glob_pattern)]
	#[cfg_attr(
		feature = "serde",
		serde(
			serialize_with = "serialize_patterns",
			deserialize_with = "deserialize_patterns"
		)
	)]
	pub include: Vec<glob::Pattern>,
	/// glob for ignore patterns
	#[arg(long, value_parser = parse_glob_pattern)]
	#[cfg_attr(
		feature = "serde",
		serde(
			serialize_with = "serialize_patterns",
			deserialize_with = "deserialize_patterns"
		)
	)]
	pub exclude: Vec<glob::Pattern>,
}

fn parse_glob_pattern(s: &str) -> Result<glob::Pattern, PatternError> {
	glob::Pattern::new(s)
}

#[cfg(feature = "serde")]
fn serialize_patterns<S>(
	patterns: &Vec<glob::Pattern>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	use serde::ser::SerializeSeq;

	let mut seq = serializer.serialize_seq(Some(patterns.len()))?;
	for pattern in patterns {
		seq.serialize_element(pattern.as_str())?;
	}
	seq.end()
}

#[cfg(feature = "serde")]
fn deserialize_patterns<'de, D>(
	deserializer: D,
) -> Result<Vec<glob::Pattern>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	use serde::Deserialize;

	let strs = Vec::<String>::deserialize(deserializer)?;
	strs.into_iter()
		.map(|s| glob::Pattern::new(&s).map_err(serde::de::Error::custom))
		.collect()
}


impl GlobFilter {
	pub fn set_include(mut self, watch: Vec<&str>) -> Self {
		self.include = watch
			.iter()
			.map(|w| glob::Pattern::new(w).unwrap())
			.collect();
		self
	}
	pub fn set_exclude(mut self, ignore: Vec<&str>) -> Self {
		self.exclude = ignore
			.iter()
			.map(|w| glob::Pattern::new(w).unwrap())
			.collect();
		self
	}

	pub fn with_include(mut self, watch: &str) -> Self {
		self.include.push(glob::Pattern::new(watch).unwrap());
		self
	}
	pub fn with_exclude(mut self, watch: &str) -> Self {
		self.exclude.push(glob::Pattern::new(watch).unwrap());
		self
	}
	/// Currently converts to string with forward slashes
	pub fn passes(&self, path: &Path) -> bool {
		// TODO this is presumptuous
		let path_str = path.to_string_lossy().replace('\\', "/");
		// let path = Path::new(&path_str);
		let pass_include =
			self.include.iter().any(|watch| watch.matches(&path_str))
				|| self.include.is_empty();
		let pass_exclude = self
			.exclude
			.iter()
			.all(|watch| watch.matches(&path_str) == false);
		pass_include && pass_exclude
	}
}


impl std::fmt::Debug for GlobFilter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("GlobFilter")
			.field(
				"include",
				&self
					.include
					.iter()
					.map(|p| p.to_string())
					.collect::<Vec<_>>(),
			)
			.field(
				"exclude",
				&self
					.exclude
					.iter()
					.map(|p| p.to_string())
					.collect::<Vec<_>>(),
			)
			.finish()
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use glob::Pattern;
	use std::path::Path;
	#[test]
	fn pattern() {
		let pat = Pattern::new("*target*").unwrap();
		assert!(!pat.matches("foo"));
		assert!(pat.matches("target"));
		assert!(pat.matches("foo/target/foo"));
		// let mut watcher = GlobFilter::default();
		// expect(watcher.exclude
	}
	#[test]
	fn passes() {
		let watcher = GlobFilter {
			include: vec![],
			exclude: vec![Pattern::new("*bar*").unwrap()],
			..Default::default()
		};
		assert!(watcher.passes(&Path::new("foo")));
		assert!(!watcher.passes(&Path::new("bar")));
		assert!(!watcher.passes(&Path::new("foo/bar/bazz")));

		let watcher = GlobFilter {
			include: vec![Pattern::new("*foo*").unwrap()],
			exclude: vec![Pattern::new("*bar*").unwrap()],
			..Default::default()
		};

		assert!(watcher.passes(&Path::new("bing/foo/bong")));
		// backslashes are normalized to forward slashes
		assert!(watcher.passes(&Path::new("bing\\foo\\bong")));
		assert!(!watcher.passes(&Path::new("froo")));
		assert!(!watcher.passes(&Path::new("bar")));


		let watcher = GlobFilter {
			include: vec![Pattern::new("foo/bar").unwrap()],
			..Default::default()
		};

		assert!(watcher.passes(&Path::new("foo/bar")));
		// backslashes are normalized to forward slashes
		assert!(watcher.passes(&Path::new("foo\\bar")));


		let pat = Pattern::new("**/*.rs").unwrap();
		assert_eq!(pat.as_str(), "**/*.rs");


		let watcher = GlobFilter::default()
			.with_include("**/*.rs")
			.with_exclude("*.git*")
			.with_exclude("*target*");

		assert!(watcher.passes(&Path::new("/foo/bar/bazz.rs")));
		assert!(!watcher.passes(&Path::new("/foo/target/bazz.rs")));

		let watcher = GlobFilter::default()
			.with_include("**/*.rs")
			.with_exclude("{.git,target,html}/**")
			.with_exclude("*codegen*");

		assert!(watcher.passes(&Path::new("src/lib.rs")));
		assert!(!watcher.passes(&Path::new("src/codegen/mockups.rs")));
	}
}
