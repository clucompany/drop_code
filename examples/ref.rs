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
			adl.drop_logic(); // 1
		});
	}
	adl.valid_logic(); // 2
	// out:
	// #[] drop_logic!
	// #[] valid_logic!
}
