// ============ IMPORTS ============
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};




// ============ STATIC'S/CONST'S ============
const MAX_LAUNCHES_PER_APP: usize = 50;
const FRECENCY_STORE_PATH: &str   = ".cache/icelauncher/frecency.json";
/// Launches older than this many days are dropped from each app's history.
const MAX_LAUNCH_AGE_DAYS: u64    = 90;
/// Apps with zero surviving launches after pruning are removed entirely.
/// Also caps total number of tracked apps to this limit (lowest-scored removed first).
const MAX_TRACKED_APPS: usize     = 500;




// ============ ENUM/STRUCT, ETC ============
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AppRecord
{
	pub launches: Vec<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FrecencyStore
{
	pub apps: HashMap<String, AppRecord>,
}




// ============ IMPL'S ============
impl AppRecord
{
	pub fn record_launch(&mut self)
	{
		self.launches.insert(0, now_secs());
		self.launches.truncate(MAX_LAUNCHES_PER_APP);
	}

	pub fn score_at(&self, now: u64) -> f64
	{
		self.launches.iter().map(|&t| recency_weight(now, t)).sum()
	}

	pub fn score(&self) -> f64
	{
		self.score_at(now_secs())
	}

	pub fn launch_count(&self) -> usize
	{
		self.launches.len()
	}
}

impl FrecencyStore
{
	pub fn load() -> Self
	{
		let path = store_path();
		let text = fs::read_to_string(&path).unwrap_or_default();
		let mut store: Self = serde_json::from_str(&text).unwrap_or_default();
		store.prune();
		store
	}

	pub fn save(&self)
	{
		let path = store_path();
		if let Some(parent) = path.parent() {
			let _ = fs::create_dir_all(parent);
		}
		if let Ok(json) = serde_json::to_string_pretty(self) {
			let _ = fs::write(&path, json);
		}
	}

	/// Record a launch in memory AND immediately persist to disk.
	///
	/// Prefer `record_in_memory` + a background `save` (as done in
	/// `record_and_launch`) for non-blocking paths.  This method exists for
	/// call-sites that need a synchronous, single-step record+save; it calls
	/// `record_in_memory` exactly once before saving so no launch is
	/// double-counted.
	pub fn save_record(&mut self, exec: &str)
	{
		self.record_in_memory(exec);
		self.save();
	}

	pub fn record_in_memory(&mut self, exec: &str)
	{
		self.apps.entry(exec.to_string()).or_default().record_launch();
	}

	pub fn score(&self, exec: &str) -> f64
	{
		self.apps.get(exec).map(|r| r.score()).unwrap_or(0.0)
	}

	pub fn score_snapshot(&self) -> HashMap<&str, f64>
	{
		let now = now_secs();
		self.apps.iter().map(|(k, v)| (k.as_str(), v.score_at(now))).collect()
	}

	pub fn launch_count(&self, exec: &str) -> usize
	{
		self.apps.get(exec).map(|r| r.launch_count()).unwrap_or(0)
	}

	pub fn top_n(&self, n: usize) -> Vec<String>
	{
		let now = now_secs();
		let mut scored: Vec<(&String, f64)> =
			self.apps.iter().map(|(k, v)| (k, v.score_at(now))).collect();
		scored.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
		scored.into_iter().take(n).map(|(k, _)| k.clone()).collect()
	}

	/// Remove stale launch timestamps and evict low-value / excess app entries.
	///
	/// - Any individual launch timestamp older than `MAX_LAUNCH_AGE_DAYS` is dropped.
	/// - Apps left with zero launches are removed entirely.
	/// - If more than `MAX_TRACKED_APPS` apps remain, the lowest-scored ones are evicted.
	pub fn prune(&mut self)
	{
		let cutoff_secs = now_secs().saturating_sub(MAX_LAUNCH_AGE_DAYS * 86_400);

		// Drop old timestamps from every record.
		for record in self.apps.values_mut()
		{
			record.launches.retain(|&t| t >= cutoff_secs);
		}

		// Remove apps with no remaining launches.
		self.apps.retain(|_, record| !record.launches.is_empty());

		// If still over the cap, evict the lowest-scored apps.
		if self.apps.len() > MAX_TRACKED_APPS
		{
			let now = now_secs();
			let mut scored: Vec<(String, f64)> = self.apps
				.iter()
				.map(|(k, v)| (k.clone(), v.score_at(now)))
				.collect();
			// Sort ascending by score — weakest first.
			scored.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
			let to_remove = scored.len() - MAX_TRACKED_APPS;
			for (key, _) in scored.into_iter().take(to_remove)
			{
				self.apps.remove(&key);
			}
		}
	}
}




// ============ FUNCTIONS ============
#[inline]
fn recency_weight(now_secs: u64, launch_secs: u64) -> f64
{
	let age_secs = now_secs.saturating_sub(launch_secs) as f64;
	let age_days = (age_secs / 86_400.0).max(0.01);
	1.0 / age_days
}


fn now_secs() -> u64
{
	SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}


fn store_path() -> PathBuf
{
    home::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(FRECENCY_STORE_PATH)
}
