// ============ IMPORTS ============
use std::collections::HashMap;
use memchr::memmem;
use rayon::prelude::*;




// ============ CRATES ============
use crate::helpers::calc::evaluate_as_calculator;
use crate::helpers::frecency::FrecencyStore;
use crate::helpers::fuzzy::fuzzy_score;
use crate::ron::LauncherConfig;
use crate::AppEntry;




// ============ ENUM/STRUCT, ETC ============
struct ScoredEntry
{
	index:      usize,
	exact_tier: u8,
	fuzzy:      i32,
	frecency:   f64,
}




// ============ FUNCTIONS ============
pub fn filter_entries(entries: &[AppEntry], query: &str, config: &LauncherConfig, frecency: &FrecencyStore) -> Vec<AppEntry>
{
	let behaviour = &config.behaviour;

	if query.is_empty()
	{
		if !behaviour.show_on_empty_query
		{
			return Vec::new();
		}
		let all = sort_by_frecency(entries, frecency);
		return cap_empty_results(all, config);
	}

	if query.chars().count() < behaviour.min_query_length
	{
		return Vec::new();
	}

	let mut results = score_and_sort(entries, query, config, frecency);

	if config.behaviour.calc_enabled
		&& let Some(calc_entry) = evaluate_as_calculator(query)
	{
		results.insert(0, calc_entry);
	}

	results
}


fn cap_empty_results(mut entries: Vec<AppEntry>, config: &LauncherConfig) -> Vec<AppEntry>
{
	let cap = if config.behaviour.max_empty_results > 0
	{
		config.behaviour.max_empty_results
	}
	else
	{
		config.window.max_results
	};
	entries.truncate(cap);
	entries
}


fn sort_by_frecency(entries: &[AppEntry], frecency: &FrecencyStore) -> Vec<AppEntry>
{
	let scores = frecency.score_snapshot();
	let mut indices: Vec<usize> = (0..entries.len()).collect();
	indices.sort_unstable_by(|&a, &b| 
        {
		let sa = scores.get(entries[a].exec.as_str()).copied().unwrap_or(0.0);
		let sb = scores.get(entries[b].exec.as_str()).copied().unwrap_or(0.0);
		sb.partial_cmp(&sa)
			.unwrap_or(std::cmp::Ordering::Equal)
			.then_with(|| entries[a].name_lc.cmp(&entries[b].name_lc))
	});
	indices.into_iter().map(|i| entries[i].clone()).collect()
}


fn score_and_sort(entries: &[AppEntry], query: &str, config: &LauncherConfig, frecency: &FrecencyStore) -> Vec<AppEntry>
{
	let normalized_query = if config.behaviour.case_sensitive 
        {
		query.to_string()
	} 
        else 
        {
		query.to_lowercase()
	};

	let finder = memmem::Finder::new(normalized_query.as_bytes());
	let scores: HashMap<&str, f64> = frecency.score_snapshot();

	let mut scored: Vec<ScoredEntry> = entries
		.par_iter()
		.enumerate()
		.filter_map(|(i, entry)| 
                {
			let frec = scores.get(entry.exec.as_str()).copied().unwrap_or(0.0);
			score_entry(i, entry, &normalized_query, &finder, config, frec)
		})
		.collect();

	scored.sort_unstable_by(|a, b| 
        {
		a.exact_tier
			.cmp(&b.exact_tier)
			.then_with(|| b.fuzzy.cmp(&a.fuzzy))
			.then_with(|| b.frecency.partial_cmp(&a.frecency).unwrap_or(std::cmp::Ordering::Equal))
			.then_with(|| entries[a.index].name_lc.cmp(&entries[b.index].name_lc))
	});

	scored.into_iter().map(|s| entries[s.index].clone()).collect()
}


#[inline]
fn simd_contains(finder: &memmem::Finder, haystack: &str) -> bool
{
	finder.find(haystack.as_bytes()).is_some()
}


#[inline]
fn score_entry(index: usize, entry: &AppEntry, normalized_query: &str, finder: &memmem::Finder, config: &LauncherConfig, frecency: f64) -> Option<ScoredEntry>
{
	let behaviour = &config.behaviour;
	let name    = if behaviour.case_sensitive { entry.name.as_str()    } else { &entry.name_lc    };
	let comment = if behaviour.case_sensitive { entry.comment.as_str() } else { &entry.comment_lc };
	let exec    = if behaviour.search_exec 
        {
		if behaviour.case_sensitive { entry.exec.as_str() } else { &entry.exec_lc }
	} 
        else 
        {
		""
	};

	let keywords: &[String] = if behaviour.case_sensitive 
        {
		&entry.keywords
	} 
        else 
        {
		&entry.keywords_lc
	};

	let name_exact    = behaviour.search_name     && simd_contains(finder, name);
	let comment_exact = behaviour.search_comment  && simd_contains(finder, comment);
	let exec_exact    = behaviour.search_exec     && simd_contains(finder, exec);
	let keyword_exact = behaviour.search_keywords && keywords.iter().any(|k| simd_contains(finder, k));
	let any_exact = name_exact || comment_exact || exec_exact || keyword_exact;
	let name_fuzzy = if behaviour.search_name { fuzzy_score(normalized_query, name) } else { 0 };

	let best_fuzzy = if any_exact 
        {
		name_fuzzy
	} 
        else 
        {
		let comment_fuzzy = if behaviour.search_comment { fuzzy_score(normalized_query, comment) } else { 0 };
		let exec_fuzzy    = if behaviour.search_exec    { fuzzy_score(normalized_query, exec)    } else { 0 };
		let keyword_fuzzy = if behaviour.search_keywords 
                {
			keywords.iter().map(|k| fuzzy_score(normalized_query, k)).max().unwrap_or(0)
		} 
                else 
                {
			0
		};
		name_fuzzy.max(comment_fuzzy).max(exec_fuzzy).max(keyword_fuzzy)
	};

	if !any_exact && best_fuzzy == 0 
        {
		return None;
	}

	let exact_tier = if any_exact 
        {
		classify_exact_tier(name, normalized_query, name_exact, keyword_exact, exec_exact, comment_exact)
	}
        else 
        {
		6
	};

	Some(ScoredEntry { index, exact_tier, fuzzy: best_fuzzy, frecency })
}


pub fn classify_exact_tier(name: &str, query: &str, name_exact: bool, keyword_exact: bool, exec_exact: bool, comment_exact: bool) -> u8
{
	if name.starts_with(query) { 0 }
	else if name_exact         { 1 }
	else if keyword_exact      { 2 }
	else if exec_exact         { 3 }
	else if comment_exact      { 4 }
	else                       { 5 }
}
