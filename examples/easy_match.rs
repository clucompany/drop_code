
use drop_code::drop_code;

fn main() {
	let a: u8 = 1;
	match a {
		0 => {
			drop_code! {
				println!("0");
			};
			if 1 == 1 {
				drop_code! {
					println!("1 == 1");
				};
			}
			// out: 
			// 1 == 1
			// 0
		},
		1 => {
			drop_code! {
				println!("1");
			};
			if 1 == 1 {
				panic!("1 == 1");
			}
			// out: 
			// thread 'main' panicked at '1 == 1', examples/easy_match.rs:25:17
			// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
			// 1
		},
		_ => {
			drop_code! {
				println!("_");
			};
			// out: 
			// _
		},
	}
}
