#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Config {
	pub header: String,
	pub content: String,
}

pub use self::anyconfig::file as parse;

peg::parser!( grammar anyconfig() for str {
	pub rule file() -> Vec<Config>
		= comment_or_empty_line()* c:config_part()* { c }

	rule config_part() -> Config
		= h:header() s:not_header()* {
			Config {
				header: h.to_string(), 
				content: s.into_iter().filter_map(|x| x).collect(),
			}
		}

	rule comment_or_empty_line() 
		= comment()
		/ _ __

	rule not_header() -> Option<&'input str>
		= comment() { None }
		/ s:anyline() { Some(s) }

	rule header() -> &'input str = "[" i:ident() "]" _ __ { i }
	rule anyline() -> &'input str = !header() s:$(char_except_newline()* __) { s }
	rule comment() = _ "#" char_except_newline()* __

	rule ident() -> &'input str = n:$(ident_head() ident_tail()) { n }
	rule ident_head() = ['a'..='z' | 'A'..='Z' | '_']
	rule ident_tail() = ['a'..='z' | 'A'..='Z' | '_' | '0'..='9']*

	rule char_except_newline() = !__ [_]
	rule _()   = quiet!{[' ' | '\t']*}
	rule __()  = quiet!{"\n"} / quiet!{"\r\n"}
	rule ___() = quiet!{[' ' | '\t' | '\n']*}
});

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		let test = vec![
			Config { 
				header: "grammar".to_string(),
				content: "S -> P;\nP -> \"a\" { okay = \"result\"; };\n  \n".to_string(),
			},
			Config { 
				header: "tests".to_string(),
				content: "\"a\" == { \"okay\": \"result\" }\nb == {}\n".to_string(),
			},
		];
		assert_eq!(parse(indoc::indoc!(r#"
			# This file provided
			# as is

			# Ok?
				 
			      
			# ok.
			[grammar]
			S -> P;
			# inside comment
			P -> "a" { okay = "result"; };
			  
			[tests]
			"a" == { "okay": "result" }
			b == {}
			"#)
		), Ok(test))
    }
}
