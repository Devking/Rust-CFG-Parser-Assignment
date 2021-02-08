
fn main () {
	// to convert from str to String
	let mut thing = " {a: a} ".to_string();

	// to get rid of white space from String
	let line = thing.trim();
	println!("|{}|", line);

	// to convert from String to str
	let slicer : &str = &*thing;

	// can't do .words()
	let talk = "search space";
	// talk.words();
	let talker = talk.replace("\t","a");
	println!("{}", talker);
	let v: Vec<&str> = "lion tiger leopard".split(char::is_whitespace).collect();
	for word in v {
		println!("{}", word);
	}

	// trim_left_matches(s)
	// trim_right_matches(s)

	let matcher = "{a:b:d,e}";
	let matchalist: Vec<char> = matcher.chars().collect();
	
	let mut num = matchalist.len() + 1;

	for n in 0..matchalist.len() {
		if matchalist[n] == ':' {
			num = n;
			break;
		}
	}

	let mut neustring = "".to_string();
	let mut follostring = "".to_string();

	if num > matchalist.len()-2 { println!("Failed"); }
	else {
		for j in 1..num {
			neustring.push(matchalist[j]);
		}
		for j in num+1..matchalist.len()-1 {
			follostring.push(matchalist[j]);
		}
	}

	let trimstring = neustring.trim();
	let tfolstring = follostring.trim();

	println!("List Name: |{}|",trimstring);
	println!("Items: |{}|",tfolstring);

}