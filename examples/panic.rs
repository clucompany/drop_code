
use drop_code::drop_code;

#[allow(unreachable_code)]
fn main() {
	drop_code! {
		println!("7 line drop code logic"); // 3
	}
	
	println!("10 line, main code"); // 1
	panic!("10 line code"); // 2
	println!("12 line, end code");
}
