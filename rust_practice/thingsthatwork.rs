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

//////////////////////////////////////////////////////////////////////
// Lexical Analysis Functions (Building the Parse Tree)
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

// ONEITEM: check if it's one of four possible items
// ONEITEM ::= NUMBER | STRING | PTR | LIST
fn check_oneitem(s: &str) -> (bool, Node) {
	let mut thenode;
	let mut flag = true;
	if check_numeric(s) { 
		thenode = Node { value: s.to_string(), nodetype: Type::Number, children: vec![] };
	} else if check_string(s) {
		thenode = Node { value: s.to_string(), nodetype: Type::ListString, children: vec![] };
	} else if check_pointer(s) {
		// NOTE: NO NEED TO STORE THE "@"" FOR POINTERS -- get rid of it first
		let mut newsome = String::new();
		for n in 1..s.len() { newsome.push ( s.chars().nth(n).unwrap() ); }
		thenode = Node { value: newsome, nodetype: Type::Pointer, children: vec![] };
	} else { let tuple = check_data(s); flag = tuple.0; thenode = tuple.1; }
	return (flag, thenode);
}

// ITEMS ::= ONEITEM | ONEITEM ',' ITEMS
// Be sure to deal with whitespace!
fn check_items(s: &str) -> (bool,Vec<Node>) {
	let children: Vec<Node> = vec![];
	let flag = true;

	// there needs to be at least one item for this to be true
	// "5, 6, 10, 11 , something	, oranother, {a:b,c}" is a valid string
	// be sure to deal with whitespace! deal with the ',' case that can appear in children lists

	// loop over some 's' and break it up by , --> this would cause problems with lists
	// let oneitem: Node = check_oneitem( ------ each delimited part of the string ------ );
	// children.push(oneitem);
	
	return (flag, children);
}

// LIST: check if valid list and return the node (each line can only have a single tree)
// Note: Allow for whitespace between tokens
// Whitespace at front and end of string is already trimmed at this point
fn check_data(s: &str) -> (bool, Node) {
	// Convert slice to a String so we can do String operations
	let ss = s.to_string();
	// Set default node and default flag
	let mut thenode = Node { value: "ERROR".to_string(), nodetype: Type::List, children: vec![] };
	let mut flag = true;
	// If length is not long enough to satisfy basic conditions, fail
	if ss.len() < 5 { return(false,thenode); }
	// Check { (first char)
	if !ss.starts_with('{') { return(false,thenode); }
	// Check } (last char)
	if !ss.ends_with('}') { return(false,thenode); }
	// Break up the String by the first instance of ':'
	let matchalist: Vec<char> = s.chars().collect();
	let mut num = matchalist.len();
	for n in 0..matchalist.len() { if matchalist[n] == ':' { num = n; break; } }
	let mut lhs = "".to_string();
	let mut rhs = "".to_string();
	// If no ':' was found, or it's found right before the '{', the string is invalid
	if num > matchalist.len()-2 { return(false,thenode); }
	else {
		for j in 1..num { lhs.push(matchalist[j]); }
		for j in num+1..matchalist.len()-1 { rhs.push(matchalist[j]); }
	}
	// If we get to here, the string is valid, and we have both parts broken up
	// We will trim the leading and trailing whitespace
	let namefield = lhs.trim();
	let childrenfield = rhs.trim();
	// NAME: Check NAME (everything between "{ :") using check_string()
	if !check_string(namefield) { return(false,thenode); }
	thenode.value = namefield.to_string();
	// Check ITEMS (everything between ": }") using check_items())
	let itemtuple = check_items(childrenfield);
	if !itemtuple.0 { flag = false; }
	thenode.children = itemtuple.1;
	return (flag, thenode);
}

// ONEQ: check if valid query and return the query name
// 'SUM', 'sum', 'ptrs', 'PTRS', 'NAMECHECK', 'namecheck', 'SEARCH'/'search' STRING/NUMBER
// Assumes that the leading and trailing whitespace has already been removed
// We can still expect tabs/spaces after 'search'
fn check_query(s: &str) -> (bool, String) {
	if s == "SUM" || s == "sum" {
		return (true, "SUM".to_string());
	} else if s == "PTRS" || s == "ptrs" {
		return (true, "PTRS".to_string());
	} else if s == "NAMECHECK" || s == "namecheck" {
		return (true, "NAMECHECK".to_string());
	} else {
		// either a search query, or just junk
		let talker = s.replace("\t"," ");
		let mut squery: Vec<&str> = talker.split(char::is_whitespace).collect();
		squery.retain( |&x| x != "");
		if squery.len() == 2 
		&& squery[0] == "search" || squery[0] == "SEARCH" 
		&& check_string(squery[1].clone()) || check_numeric(squery[1].clone()) {
				let qsearch = "SEARCH ".to_string() + squery[1];
				return (true, qsearch);
		}
		return (false, "FAIL".to_string());
	}
}

// INPUT: Build the parse tree for data and list what queries need to be run
// Also, check that the form of the input is valid, and return a bool telling whether it is
fn validate_input (input: Vec<String>) -> (Vec<Node>, Vec<String>, bool) {
	let mut datatree: Vec<Node> = vec![];
	let mut queries: Vec<String> = vec![];
	let mut valid = false;
	// input is too short -- error
	if input.len() < 2 { return (datatree, queries, false); }
	// QUIT: no "QUIT" detected -- error
	if input[input.len()-1] != "QUIT" { return (datatree, queries, false); }
	// keep track of the line number that we are on
	let mut lineno = 0;
	// DATA: goes through the data portion of the input
	while lineno < input.len()-1 {
		let line = input[lineno].trim();
		if line == "." { lineno += 1; valid = true; break; }
		let duple = check_data(&*line);
		if !duple.0 { return(datatree, queries, false); }
		datatree.push(duple.1);
		lineno += 1;
	}
	// no "." detected -- error
	if !valid { return (datatree, queries, false); }
	// QUERY: goes through the query portion of the input
	while lineno < input.len()-1 {
		let line = input[lineno].trim();
		let duple = check_query(&*line);
		if !duple.0 { return (datatree, queries, false); }
		queries.push(duple.1);
		lineno += 1;
	}
	return (datatree, queries, valid);
}

//////////////////////////////////////////////////////////////////////
// Query Functions (Recursive Descent Through The Parse Tree)
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
	if names.len() < 2 { println!("OK"); return; }
	names.sort_by(|a, b| a.cmp(b));
	let mut duplicates: Vec<String> = vec![];
	for n in 0..names.len()-1 { if names[n] == names[n+1] { duplicates.push(names[n].clone()); } }
	if duplicates.len() == 0 { println!("OK"); return; }
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

// Recursively search through the tree for a match
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

// AFTER the data tree and queries list is confirmed valid, do the queries
fn run_queries(data: Vec<Node>, queries: Vec<String>) {
	for query in queries {
		if query == "SUM" || query == "sum" {
			println!("{}",recursive_sum(data.to_vec()));
		} else if query == "NAMECHECK" || query == "namecheck" {
			namecheck(data.to_vec());
		} else if query == "PTRS" || query == "ptrs" {
			pointercheck(data.to_vec());
		} else {
			let squery: Vec<&str> = query.split(char::is_whitespace).collect();
			search(data.to_vec(), squery[1]);
		}
	}
}

fn main () {
	// Get all of the user input and store into a vector
	let stdin = io::stdin();
	let mut v : Vec<String> = vec![];
	for line in stdin.lock().lines() { v.push(line.unwrap().to_string()); }
	// Parse the input (lexical analysis)
	let triple = validate_input(v);
	// Perform queries (output) if input was valid
	if triple.2 { run_queries(triple.0, triple.1); } 
	else { println!("ERR"); }
}