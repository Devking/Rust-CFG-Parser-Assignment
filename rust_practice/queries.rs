use std::io;
use std::io::prelude::*;

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

	datatree.push( Node { value: "a".to_string(), nodetype: Type::List, children: vec![ Node { value: "3".to_string(), nodetype: Type::Number, children: vec![ ] }, Node { value: "d".to_string(), nodetype: Type::List, children: vec![ Node { value: "apple".to_string(), nodetype: Type::ListString, children: vec![] } ] } ] } );
	datatree.push( Node { value: "1".to_string(), nodetype: Type::Number, children: vec![] } );

	datatree.push( Node { value: "c".to_string(), nodetype: Type::List, children: vec![ Node { value: "d".to_string(), nodetype: Type::List, children: vec![ Node { value: "a".to_string(), nodetype: Type::ListString, children: vec![] } ] } ] } );

	datatree.push( Node { value: "b".to_string(), nodetype: Type::List, children: vec![ Node { value: "c".to_string(), nodetype: Type::List, children: vec![ Node { value: "a".to_string(), nodetype: Type::ListString, children: vec![] } ] } ] } );

	datatree.push( Node { value: "c".to_string(), nodetype: Type::List, children: vec![ Node { value: "30".to_string(), nodetype: Type::Number, children: vec![] } ] } );

	// NOTE: DO NOT STORE THE @ WITH POINTERS
	datatree.push( Node { value: "p".to_string(), nodetype: Type::Pointer, children: vec![] } );
	datatree.push( Node { value: "a".to_string(), nodetype: Type::Pointer, children: vec![] } );
	datatree.push( Node { value: "b".to_string(), nodetype: Type::Pointer, children: vec![] } );
	datatree.push( Node { value: "e".to_string(), nodetype: Type::Pointer, children: vec![] } );
	datatree.push( Node { value: "c".to_string(), nodetype: Type::Pointer, children: vec![] } );

	/*
	datatree.push( Node { value: "c".to_string(), nodetype: Type::List, children: vec![] } );
	datatree.push( Node { value: "c".to_string(), nodetype: Type::List, children: vec![] } );
	*/

	return datatree;
}

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
	if names.len() < 2 { println!("OK"); return; }
	names.sort_by(|a, b| a.cmp(b));
	let mut duplicates: Vec<String> = vec![];
	for n in 0..names.len()-1 { if names[n] == names[n+1] { duplicates.push(names[n].clone()); } }
	duplicates.dedup();
	for n in 0..duplicates.len() {
		print!("{}", duplicates[n]);
		if n != duplicates.len()-1 { print!(","); } else { print!("\n"); }
	}
}

// gather all of the names of lists in the tree
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

// recursively search through the tree for a match
fn recursive_search (data: Vec<Node>, look: &str) -> (Vec<String>,bool) {
	let mut flag = false;
	let mut carry: Vec<String> = vec![];
	for node in data {
		match node.nodetype {
			Type::List => { 
				let duple = recursive_search(node.children, look);
				if duple.1 {
					if duple.0.len() == 0 { carry.push(node.value.clone()); }
					let s = ":".to_string() + &*node.value.clone();
					for each in duple.0 { carry.push(each + &*s); }
					flag = true;
				}
			},
			Type::ListString => {
				if node.value == look.to_string() {
					return (vec![],true);
				}
			},
			Type::Number => {
				if node.value == look.to_string() {
					return (vec![],true);
				}
			},
			_ => {}
		}
	}
	return (carry,flag);
}

// SEARCH: search through the tree for specified STRING or NUMBER
// prints out all matches in alphabetical order
fn search(data: Vec<Node>, look: &str) {
	let mut found: Vec<String> = recursive_search(data, look).0;
	found.sort_by(|a, b| a.cmp(b));
	if found.len() == 0 { println!("NIL"); return; }
	for n in 0..found.len() {
		print!("{}", found[n]);
		if n != found.len()-1 { print!(","); } else { print!("\n"); }
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
	println!("{}",treesum);
	namecheck(datatree.to_vec());
	pointercheck(datatree.to_vec());
	search(datatree.to_vec(), "30");	// c
	search(datatree.to_vec(), "apple"); // d:a
	search(datatree.to_vec(), "a");		// c:b,d:c -- note alphabetical
	search(datatree.to_vec(), "barbados");		// NIL
	
}