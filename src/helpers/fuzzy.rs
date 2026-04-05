// ============ FUNCTIONS ============
pub fn fuzzy_score(needle: &str, haystack: &str) -> i32
{
	if needle.is_empty() {
		return 1;
	}
	if needle.len() > haystack.len() {
		return 0;
	}

	if needle.is_ascii() && haystack.is_ascii() {
		fuzzy_score_ascii(needle.as_bytes(), haystack.as_bytes())
	} else {
		fuzzy_score_unicode(needle, haystack)
	}
}


#[inline]
fn fuzzy_score_ascii(needle: &[u8], haystack: &[u8]) -> i32
{
	let mut ni = 0;
	let mut score = 0_i32;
	let mut consecutive = 0_i32;
	let mut prev_hi: usize = usize::MAX;

	let mut hi = 0;
	while hi < haystack.len() && ni < needle.len() {
		if haystack[hi] != needle[ni] {
			consecutive = 0;
			hi += 1;
			continue;
		}

		consecutive = if prev_hi.wrapping_add(1) == hi { consecutive + 1 } else { 0 };

		score += 1 + consecutive * 2;
		if hi == 0 {
			score += 5;
		} else if is_word_boundary_byte(haystack[hi - 1]) {
			score += 3;
		}

		prev_hi = hi;
		hi += 1;
		ni += 1;
	}

	if ni == needle.len() { score } else { 0 }
}


fn fuzzy_score_unicode(needle: &str, haystack: &str) -> i32
{
	let mut needle_chars = needle.chars().peekable();
	let mut score = 0_i32;
	let mut consecutive = 0_i32;
	let mut prev_byte_pos: usize = usize::MAX;
	let mut prev_char_end: usize = usize::MAX;

	for (byte_pos, haystack_char) in haystack.char_indices() {
		let Some(&needle_char) = needle_chars.peek() else { break };

		if haystack_char != needle_char {
			consecutive = 0;
			continue;
		}

		consecutive = if prev_char_end == byte_pos { consecutive + 1 } else { 0 };

		score += 1 + consecutive * 2;
		if byte_pos == 0 {
			score += 5;
		} else if prev_byte_pos != usize::MAX {
			let prev_char = haystack[prev_byte_pos..].chars().next().unwrap_or(' ');
			if is_word_boundary(prev_char) {
				score += 3;
			}
		}

		prev_byte_pos = byte_pos;
		prev_char_end = byte_pos + haystack_char.len_utf8();
		needle_chars.next();
	}

	if needle_chars.peek().is_some() { 0 } else { score }
}


#[inline(always)]
fn is_word_boundary_byte(b: u8) -> bool
{
	matches!(b, b' ' | b'-' | b'_' | b'.')
}


#[inline(always)]
fn is_word_boundary(c: char) -> bool
{
	matches!(c, ' ' | '-' | '_' | '.')
}
