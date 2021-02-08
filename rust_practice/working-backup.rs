use std::io;
use std::io::prelude::*;

//////////////////////////////////////////////////////////////////////
// The Parse Tree Structure
/////////////////////////////////////////////////////////////////////

enum Type { List, ListString, Number, Pointer }

impl Clone for Type {
	fn clone (&self) -> Type {
		match *self {
			Type::List => Type::List,
			Type::ListString => Type::ListString,
			Type::Number => Type::Number,
			Type::Pointer => Type::Pointer
		}
	}
}

struct Node {
	value: String,
	nodetype: Type,
	children: Vec<Node>
}

impl Clone for Node {
	fn clone (&self) -> Node {
		return Node { value: self.value.clone(), nodetype: self.nodetype.clone(), children: self.children.clone() };
	}
}

fn validate_input (input: Vec<String>) -> Vec<Node> {
	let mut datatree: Vec<Node> = vec![];
	if input.len() == 0 { return datatree; }
	return datatree;
}

//////////////////////////////////////////////////////////////////////
// Lexical Functions
/////////////////////////////////////////////////////////////////////

// checks if 's' is a NUMBER
// NUMBER ::= '-' [0-9]+ | [0-9]+
fn check_numeric(s: &str) -> bool { s.parse::<i64>().is_ok() }

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
	if s.starts_with("@") { 
		let mut newsome = String::new();
		for n in 1..s.len() { newsome.push ( s.chars().nth(n).unwrap() ); }
		return check_string(&*newsome); 
	} else { return false; }
}

//////////////////////////////////////////////////////////////////////
// Query Functions
/////////////////////////////////////////////////////////////////////

// SUM: use recursive descent to go through all nodes and get the sum of numeric fields
fn recursive_sum (data: Vec<Node>) -> i64 {
	let mut sum: i64 = 0;
	for node in data {
		match node.nodetype  {
			Type::List => sum += recursive_sum(node.children),
			Type::Number => sum += node.value.parse::<i64>().unwrap(),
			_ => {}
		}
	}
	return sum;
}

// gather all of the names of lists in the tree
fn recursive_namecheck (data: Vec<Node>) -> Vec<String> {
	let mut names: Vec<String> = vec![];
	for node in data {
		match node.nodetype {
			Type::List => { 
				names.push(node.value); 
				for childstring in recursive_namecheck(node.children) {
					names.push(childstring);
				}
			},
			_ => {}
		}
	}
	return names;
}

// NAMECHECK: get names of list in tree and check for duplicates
fn namecheck (data: Vec<Node>) {
	let mut names: Vec<String> = recursive_namecheck(data);
	if names.len() < 2 { return; }
	names.sort_by(|a, b| a.cmp(b));
	let mut duplicates: Vec<String> = vec![];
	for n in 0..names.len()-1 { if names[n] == names[n+1] { duplicates.push(names[n].clone()); } }
	duplicates.dedup();
	for n in 0..duplicates.len() {
		print!("{}", duplicates[n]);
		if n != duplicates.len()-1 { print!(","); } else { print!("\n"); }
	}
}

// gather all of the names of the pointers in the tree
fn recursive_ptrcheck (data: Vec<Node>) -> Vec<String> {
	let mut names: Vec<String> = vec![];
	for node in data {
		match node.nodetype {
			Type::List => { 
				for childstring in recursive_ptrcheck(node.children) {
					names.push(childstring);
				}
			},
			Type::Pointer => names.push(node.value),
			_ => {}
		}
	}
	return names;
}

// PTRS: perform a pointer check to find dangling pointers
fn pointercheck (data: Vec<Node>) {
	// get all the names of the lists
	let mut names: Vec<String> = recursive_namecheck(data.to_vec());
	names.sort_by(|a, b| a.cmp(b));
	names.dedup();
	// get all the names of all the pointers
	let mut ptrs: Vec<String> = recursive_ptrcheck(data.to_vec());
	ptrs.sort_by(|a, b| a.cmp(b));
	ptrs.dedup();
	// look for dangling pointers
	let mut dangling: Vec<String> = vec![];
	for elem in ptrs {
		if names.binary_search(&elem).is_err() { dangling.push(elem); }
	}
	// print correct message
	if dangling.len() == 0 { println!("OK"); return; }
	for n in 0..dangling.len() {
		print!("{}", dangling[n]);
		if n != dangling.len()-1 { print!(","); } else { print!("\n"); }
	}
}

fn main () {

	// get all user input and store into a vector
	let stdin = io::stdin();
	let mut v : Vec<String> = vec![];
	for line in stdin.lock().lines() { v.push(line.unwrap().to_string()); }

	// parse the input (lexical)
	let datatree: Vec<Node> = validate_input(v);

	// perform queries (output)
	let treesum = recursive_sum(datatree.to_vec());
	println!("Sum is: {}", treesum);
	namecheck(datatree.to_vec());
	
}