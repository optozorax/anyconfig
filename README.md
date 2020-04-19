# anyconfig

Parse this text:
```
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
```

to this structure:
```rust
vec![
	Config { 
		header: "grammar".to_string(),
		content: "S -> P;\nP -> \"a\" { okay = \"result\"; };\n  \n".to_string(),
	},
	Config { 
		header: "tests".to_string(),
		content: "\"a\" == { \"okay\": \"result\" }\nb == {}\n".to_string(),
	},
];
```

it just put arbitrary syntax after `[header]` in string.