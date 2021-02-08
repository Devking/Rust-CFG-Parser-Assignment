// Written in Rust Beta
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

// NUMBER ::= '-' [0-9]+ | [0-9]+
// checks if 's' is a NUMBER
fn check_numeric(s: &str) -> bool { s.parse::<i64>().is_ok() }

// STRING ::= [a-zA-z][0-9a-zA-z]
// checks if 's' is a STRING
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

// PTR :: = '@' STRING
// checks if 's' is a PTR
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
	let mut children: Vec<Node> = vec![];
	if s.len() == 0 { return (false, children); }
	let matchalist: Vec<char> = s.chars().collect();
	let mut sss = "".to_string();
	let mut workinglist: Vec<String> = vec![];
	let mut innerlist = false;
	for c in matchalist {
		if innerlist { sss.push(c); if c == '}' { innerlist = false; } }
		else {
			if c == '{' { innerlist = true; }
			if c != ',' { sss.push(c); }
			else { workinglist.push( sss.trim().to_string()); sss = "".to_string(); }
		}
	}
	let lastone = sss.trim().to_string();
	if lastone == "".to_string() { return (false, children); }
	workinglist.push(lastone);
	if workinglist.len() == 0 { return(false,children); }
	for elem in workinglist {
		let duple = check_oneitem(&*elem);
		if duple.0 { children.push(duple.1); } else { return(false, children); }
	}
	return (true, children);
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
		// note: rust does not have logical short-circuit evaluation
		if squery.len() != 2 { return (false, "FAIL".to_string()); }
		if squery[0] == "search" || squery[0] == "SEARCH" 
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
	if input[input.len()-1].trim() != "QUIT" { return (datatree, queries, false); }
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
	if duplicates.len() == 0 { println!("OK"); return; }
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
fn recursive_search (data: Vec<Node>, look: &str) -> (Vec<String>,bool,bool) {
	let mut flag = false;
	let mut carry: Vec<String> = vec![];
	let mut childflag = false;
	for node in data {
		match node.nodetype {
			Type::List => { 
				let duple = recursive_search(node.children, look);
				if duple.1 {
					if duple.2 { carry.push(node.value.clone()); }
					let s = ":".to_string() + &*node.value.clone();
					for each in duple.0 { carry.push(each + &*s); }
					flag = true;
				}
			},
			Type::ListString => {
				if node.value == look.to_string() {
					flag = true;
					childflag = true;
				}
			},
			Type::Number => {
				if node.value == look.to_string() {
					flag = true;
					childflag = true;
				}
			},
			_ => {}
		}
	}
	return (carry,flag, childflag);
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
	// get all user input and store into a vector
	let stdin = io::stdin();
	let mut v : Vec<String> = vec![];
	for line in stdin.lock().lines() { v.push(line.unwrap().to_string()); }
	
	// parse the input (lexical analysis)
	let triple = validate_input(v);

	// perform queries (output) if input was valid
	if triple.2 { run_queries(triple.0, triple.1); } 
	else { println!("ERR"); }
}
