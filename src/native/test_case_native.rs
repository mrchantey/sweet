use crate::native::*;
use crate::test_case::*;
use anyhow::Result;
use forky::prelude::*;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

inventory::collect!(TestCaseNative);

#[derive(Clone)]
pub struct TestCaseNative {
	pub file: StringOrStaticStr,
	pub name: StringOrStaticStr,
	pub func: TestCaseNativeFunc,
	pub config: TestCaseConfig,
}

impl std::fmt::Debug for TestCaseNative {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> Result<(), std::fmt::Error> {
		#[derive(Debug)]
		#[allow(unused)]
		struct TestCaseNative<'a> {
			file: &'a StringOrStaticStr,
			name: &'a StringOrStaticStr,
			config: &'a TestCaseConfig,
		}

		let Self {
			file, name, config, ..
		} = self;

		// per Chayim Friedman’s suggestion
		fmt::Debug::fmt(&TestCaseNative { file, name, config }, f)
	}
}

#[derive(Debug, Clone)]
pub enum StringOrStaticStr {
	StaticStr(&'static str),
	String(String),
}

impl StringOrStaticStr {
	pub fn as_str(&self) -> &str {
		match self {
			StringOrStaticStr::StaticStr(s) => s,
			StringOrStaticStr::String(s) => s,
		}
	}
}


impl TestCase for TestCaseNative {
	/// Forward slashed path
	fn path(&self) -> PathBuf {
		Path::new(self.file.as_str()).to_forward_slash()
	}
	fn name(&self) -> &str { &self.name.as_str() }
	fn config(&self) -> &TestCaseConfig { &self.config }
	async fn run_func(&self) -> Result<()> {
		panic!("native test cases are run by the native suite");
		// let fut = (self.func)();
		// try_catch_panic(fut).await
	}
}
impl TestCaseNative {
	pub fn split_funcs<'a>(
		funcs: impl IntoIterator<Item = &'a Self>,
	) -> TestCaseNativeSplit<'a> {
		funcs.into_iter().fold(
			TestCaseNativeSplit::<'a>::default(),
			|mut split, t| {
				match &t.func {
					TestCaseNativeFunc::SyncString(f) => {
						split.syncs_str.push((t, f))
					}
					TestCaseNativeFunc::Sync(f) => split.syncs.push((t, f)),
					TestCaseNativeFunc::Series(f) => split.series.push((t, f)),
					TestCaseNativeFunc::Parallel(f) => {
						split.parallels.push((t, f))
					}
				}
				split
			},
		)
	}
}

#[derive(Debug, Default)]
pub struct TestCaseNativeSplit<'a> {
	/// non-async but possibly in parallel
	pub syncs_str: Vec<(&'a TestCaseNative, &'a TestCaseNativeFuncSyncString)>,
	pub syncs: Vec<(&'a TestCaseNative, &'a TestCaseNativeFuncSync)>,
	/// Async but on main thread
	pub series: Vec<(&'a TestCaseNative, &'a TestCaseNativeFuncSeries)>,
	pub parallels: Vec<(&'a TestCaseNative, &'a TestCaseNativeFuncParallel)>,
}
