
use drop_code::drop_code;

struct AlwaysDropLogic {}

impl AlwaysDropLogic {
	fn drop_logic(&self) {
		println!("#[] drop_logic!");
	}
	
	fn valid_logic(&mut self) {
		println!("#[] valid_logic!");
	}
}

fn main() {
	let mut adl = AlwaysDropLogic {};
	{
		drop_code!((adl: AlwaysDropLogic) {
			adl.drop_logic();
		});
		
		if 1 == 2 {
			#[allow(unconditional_panic)]
			let _b = [1, 2][256];
		}
	}
	adl.valid_logic();
	// out: 
	// if 1 == 2
	// #[] valid_logic!
	// #[] drop_logic!
	//
	// if 1 == 1
	// thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 256', examples/easy_ref2.rs:24:18
	// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
	// #[] drop_logic!
}

