//#![feature(str_words)]
// #![feature(str_char)]
// use std::io;

// checks if 's' is a NUMBER
// NUMBER ::= '-' [0-9]+ | [0-9]+
fn check_numeric(s: &str) -> bool {
	if s.len() == 0 { return false; }
	let mut matcher = 0;
	// check first char to see if it's '-' and make sure there's more chars
	if s.len() > 1 && s.chars().nth(0).unwrap() == '-' { matcher += 1; }
	for token in s.chars() {
		match token {
			'0'...'9' => matcher += 1,
			_ => {}
		}
	}
	matcher == s.len()
}

// checks if 's' is a STRING
// STRING ::= [a-zA-z][0-9a-zA-z]
fn check_string(s: &str) -> bool {
	if s.len() == 0 { return false; }
	// ensure that the first char is not a numeric digit
	match s.chars().nth(0).unwrap() { '0'...'9' => return false, _ => {} }
	let mut matcher = 0;
	for token in s.chars() {
		match token {
			'0'...'9' => matcher += 1,
			'a'...'z' => matcher += 1,
			'A'...'z' => matcher += 1,
			_ => {}
		}
	}
	matcher == s.len()
}

// checks if 's' is a PTR
// PTR :: = '@' STRING
fn check_pointer(s: &str) -> bool {
	// requires str_char feature, which we can't use
	// let (c, s1) = s.slice_shift_char().unwrap();
	// if c != '@' { return false; }
	if s.starts_with("@") { 
		let mut newsome = String::new();
		for n in 1..s.len() { newsome.push ( s.chars().nth(n).unwrap() ); }
		return check_string(&*newsome); 
	} else { return false; }
}

/*
we cannot do this because 's.word()' is not allowed in beta
fn break_by_word (s: &str) {
	// needs the str_words feature
	for word in s.words() {
		print!("{}|",word);
	}
}
*/

fn main () {
	let program = "+ + 54 * - /";
	println!("{}", program);

	if check_numeric(program) {
		println!("this is a number");
	} else {
		println!("this is not a number");
	}

	let numpro = "01";
	println!("{}", numpro);
	println!("Numeric? {}", check_numeric(numpro));

	// let astring = "abc123"; //works
	// let astring = ""; //works
	 let astring = "A"; // works
	// let astring = "The quick brown fox"; // works
	// let astring = "ThequickBrownf0xJumps0v3r5"; // works
	//let astring = "1abc";
	println!("{}", check_string(numpro));
	println!("{}", check_string(program));
	println!("{}", check_string(astring));

	println!("Pointer? {}", check_pointer("@k1K"));
	println!("Pointer? {}", check_pointer("@1k1K"));
	println!("Pointer? {}", check_pointer("@"));
	println!("Pointer? {}", check_pointer(""));
	println!("Pointer? {}", check_pointer("a@"));

}