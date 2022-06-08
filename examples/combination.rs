
use drop_code::drop_code;

fn main() {
	drop_code! {
		println!("line 6");
	}
	drop_code! {
		println!("line 9");
	}
	if 1 == 1 {
		drop_code! {
			println!("line 13");
		}
		println!("line 12");
	}
	
	// out:
	// line 12
	// line 13
	// line 9
	// line 6
}
