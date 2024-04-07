
use drop_code::drop_code;

struct AlwaysDropLogic {}

impl AlwaysDropLogic {
	fn drop_logic(&mut self) {
		println!("#[] drop_logic!");
	}
	
	fn valid_logic(&mut self) {
		println!("#[] valid_logic!");
	}
}

fn main() {
	let mut adl = AlwaysDropLogic {};
	drop_code!((mut adl: AlwaysDropLogic) {
		adl.drop_logic();
	});
	
	adl.valid_logic();
	// out: 
	// #[] valid_logic!
	// #[] drop_logic!
}

