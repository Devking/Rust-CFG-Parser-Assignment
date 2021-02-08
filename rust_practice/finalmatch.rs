fn main() {
	println!("THIS THE FINAL MATCH!!!!!!!");

	let rhs = "a, b ,	c,	@f ,10		, {d :c,f }, {d:c, e, f} , {}, }}, {{}}, {{}}}";

	let matchalist: Vec<char> = rhs.chars().collect();
	let mut sss = "".to_string();
	let mut workinglist: Vec<String> = vec![];
	let mut innerlist = false;
	for c in matchalist {

		if innerlist {
			sss.push(c);
			if c == '}' { innerlist = false; }
		} else {

			if c == '{' { innerlist = true; }
			if c != ',' { sss.push(c); }
			else { 
			
			workinglist.push( sss.trim().to_string() ) ; 
			sss = "".to_string(); 
			}
		}
	}
	workinglist.push(sss.trim().to_string() );
	workinglist.retain( |x| x != "");

	for elem in workinglist {
		println!("{}", elem);
	}
}