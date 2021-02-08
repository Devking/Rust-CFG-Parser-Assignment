fn main() {
	let x = "3".to_string();
	let y = x.parse::<i64>().unwrap();

	if x.parse::<i64>().is_ok()	{
		println!("conversion was ok");
		let z = 5;
		let p = y+z;
		println!("{}", p);
	}
}