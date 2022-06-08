
use drop_code::drop_code;

fn main() {
	drop_code! {
		// AUTO_PANIC_LOGIC
		println!("#[4] main_panic_logic(): "); // 2
	}
	
	println!("#[1] main_code: ");
	if 1 == 1 { // << 1 == 1 | 1 == 2
		let always_set = &mut 0;
		panic_function(&[1], always_set);
		assert_eq!(always_set, &mut 11);
	}
	println!("end_main_code: ");
}

fn panic_function(array: &[u8], mut always_set: &mut usize) {
	println!("#[2] panic_function: ");
	assert_eq!(*always_set, 0);
	
	drop_code!((mut always_set: usize) {
		// AUTO_PANIC_LOGIC
		println!("[3] panic_function_logic(): "); // 1
		**always_set += 1;
	});
	assert_eq!(**always_set, 0);
	**always_set = 10;
	let _b = array[24]; // << -- panic and auto run drop
	assert_eq!(**always_set, 10);
	
	println!("end_panic_function: ");
}

