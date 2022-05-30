
use drop_code::drop_code;

#[test]
fn test_one_block() {
	let test = "";
	let test2 = "";
	let test3 = "";
	
	drop_code! {
		println!("1");
	};
	drop_code!((test) {
		println!("#1 drop {}", std::mem::size_of_val(test));
	});
	drop_code!((test, test2: &'static str) {
		*test2 = "GG";
		println!(
			"#2 drop {}:{}", 
			std::mem::size_of_val(test),
			std::mem::size_of_val(test2)
		);
	});
	drop_code!((test, test2, test3) {
		println!(
			"#3 drop {}:{}:{}", 
			std::mem::size_of_val(test), 
			std::mem::size_of_val(test2), 
			std::mem::size_of_val(test3)
		);
	});
	***test = "ok"; // ***???, 3dropfn!
	println!("{test}");
}

#[test]
fn test_drop_code() {
	{
		let test = "test";
		let test2 = "test2";
		drop_code!(#[inline(always)]: () {
			//println!("{}", test);
			
			println!("End0");
		});
		
		drop_code! {
			println!("End1");
		}
		
		drop_code! {
			println!("End2");
		}
		drop_code! ((test) {
			println!("End3, size_of_val: {}", std::mem::size_of_val(&test));
		});
		drop_code! ((test, test2) {
			println!("End4, size_of_val: {}", std::mem::size_of_val(&test));
		});
		
		println!("@{}", test);
		println!("@@{}", test);
	}
}

#[test]
fn easy_use_emptyargs() {
	static mut TEST_CINTERNAL: u8 = 0;
	
	fn __tinternal(a: usize, b: usize) -> bool {
		drop_code! {
			unsafe { TEST_CINTERNAL += 1; }
		}
		if a == b {
			// autorun drop code
			return true;
		}
		
		// autorun drop code
		false
	}
	
	assert_eq!(unsafe { TEST_CINTERNAL }, 0);
	let rin = __tinternal(1, 1);
	assert_eq!(rin, true);
	assert_eq!(unsafe { TEST_CINTERNAL }, 1);
}


#[test]
fn easy_use_twoargs() {
	static mut OLD_A: usize = 0;
	static mut OLD_B: usize = 0;
	
	fn __tinternal(a: usize, b: usize) -> bool {
		drop_code!((a: usize, b: usize) {
			unsafe { OLD_A = *a; }
			unsafe { OLD_B = *b; }
		});
		if a == b {
			// autorun drop code
			return true;
		}
		
		if a == &mut 1 {
			// autorun drop code
			return true;
		}
		
		// autorun drop code
		false
	}
	
	assert_eq!(unsafe { OLD_A }, 0);
	assert_eq!(unsafe { OLD_B }, 0);
	let rin = __tinternal(1, 1);
	assert_eq!(rin, true);
	assert_eq!(unsafe { OLD_A }, 1);
	assert_eq!(unsafe { OLD_B }, 1);
	
	//assert_eq!(unsafe { TEST_CINTERNAL }, 0);
	//let rin = __tinternal(1, 1);
	//assert_eq!(rin, true);
	//assert_eq!(unsafe { TEST_CINTERNAL }, 1);
}


#[test]
fn easy_use_oneargs() {
	static mut OLD_A: usize = 0;
	//static mut OLD_B: usize = 0;
	
	fn __tinternal(a: usize) -> bool {
		drop_code!((a: usize) {
			unsafe { OLD_A = *a; }
		});
		if a == &mut 1 {
			// autorun drop code
			return true;
		}
		
		// autorun drop code
		false
	}
	
	//assert_eq!(unsafe { TEST_CINTERNAL }, 0);
	//let rin = __tinternal(1, 1);
	//assert_eq!(rin, true);
	//assert_eq!(unsafe { TEST_CINTERNAL }, 1);
}