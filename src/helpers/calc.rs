// ============ CRATES ============
use crate::AppEntry;




// ============ FUNCTIONS ============
pub fn evaluate_as_calculator(expr: &str) -> Option<AppEntry>
{
	let value = try_evaluate(expr.trim())?;
	Some(AppEntry 
        {
		name: format!("= {}", value),
		generic_name: String::new(),
		exec: String::new(),
		comment: expr.trim().to_string(),
		icon: String::new(),
		icon_path: None,
		keywords: Vec::new(),
		terminal: false,
		name_lc: String::new(),
		generic_name_lc: String::new(),
		exec_lc: String::new(),
		comment_lc: String::new(),
		keywords_lc: Vec::new(),
	})
}


fn try_evaluate(expr: &str) -> Option<String>
{
	if !has_math_operators(expr) 
        {
		return None;
	}

	let mut namespace = fasteval::EmptyNamespace;
	let result = fasteval::ez_eval(expr, &mut namespace).ok()?;

	Some(format_result(result))
}


fn has_math_operators(expr: &str) -> bool
{
	let has_standard_operator = expr.contains(['+', '*', '/', '%', '^', '(', ')']);

	let has_binary_minus = 
        {
		let mut prev: Option<char> = None;
		let mut found = false;
		for ch in expr.chars().filter(|c| !c.is_whitespace()) 
                {
			if ch == '-' && let Some(p) = prev && (p.is_ascii_digit() || p == ')')
			{
				found = true;
				break;
			}
			prev = Some(ch);
		}
		found
	};

	has_standard_operator || has_binary_minus
}


fn format_result(result: f64) -> String
{
	if result.fract() == 0.0 && result.abs() < 1e15 
        {
		format!("{}", result as i64)
	} 
        else 
        {
		let formatted = format!("{:.10}", result);
		formatted.trim_end_matches('0').trim_end_matches('.').to_string()
	}
}
