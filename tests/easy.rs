
use std::sync::{Mutex, OnceLock};
use drop_code::drop_code;

#[test]
fn auto_test_syntax() {
	{ // AnonEmpty
		static CHECK: OnceLock<bool> = OnceLock::new();
		{
			drop_code! {
				CHECK.set(true).unwrap();
			};
			// auto_run_drop_code
		}
		assert_eq!(CHECK.get(), Some(&true));
	}
	
	{ // EmptyBlock
		static CHECK: OnceLock<bool> = OnceLock::new();
		{
			drop_code!(() {
				CHECK.set(true).unwrap();
			});
			// auto_run_drop_code
		}
		assert_eq!(CHECK.get(), Some(&true));
	}
	
	{ // EmptyBlock + meta for drop_trait
		static CHECK: OnceLock<bool> = OnceLock::new();
		{
			drop_code!(#[inline(always)]: () {
				CHECK.set(true).unwrap();
			});
		}
		assert_eq!(CHECK.get(), Some(&true));
	}
	
	{ // Arg1Block + str type arg
		let mut test: String = "test_str".to_string(); // size &A != &str (&A: ptr)!=(&str: leng+ptr)
		{
			drop_code!((mut test: String) {
				// test: A (A - UNK TYPE) <<";
				test.push_str("++");
			});
			assert_eq!(test.as_str(), "test_str");
		}
		assert_eq!(test, "test_str++");
	}
	
	{ // Arg2Block + unk type arg
		static SIZE_A: OnceLock<usize> = OnceLock::new();
		static SIZE_B: OnceLock<usize> = OnceLock::new();
		
		let a: u64 = 0;
		let b: u64 = 0;
		{
			drop_code!((a, b) {
				// test: A (A - UNK TYPE) <<
				
				SIZE_A.set(core::mem::size_of_val(a)).unwrap();
				SIZE_B.set(core::mem::size_of_val(b)).unwrap();
			});
		}
		assert_eq!(SIZE_A.get(), Some(&core::mem::size_of_val(&a as &u64)));
		assert_eq!(SIZE_B.get(), Some(&core::mem::size_of_val(&b as &u64)));
	}
	
	{
		let test = "test";
		let test2 = "test2";
		let test3 = "test3";
		drop_code!((test, test2: &'static str) {
			assert_eq!(
				core::mem::size_of_val(&"test" as &&str),
				core::mem::size_of_val(test)
			);
			assert_eq!(
				core::mem::size_of_val(&"test2" as &&str),
				core::mem::size_of_val(test2)
			);
		});
		drop_code!((test, test2, test3) {
			assert_eq!(
				core::mem::size_of_val(&"test" as &&str),
				core::mem::size_of_val(test)
			);
			assert_eq!(
				core::mem::size_of_val(&"test2" as &&str),
				core::mem::size_of_val(test2)
			);
			assert_eq!(
				core::mem::size_of_val(&"test3" as &&str),
				core::mem::size_of_val(test3)
			);
		});
	}
}

#[test]
fn test_one_block() {
	
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
	static TEST_CINTERNAL: Mutex<u8> = Mutex::new(0);
	
	fn __tinternal(a: usize, b: usize) -> bool {
		drop_code! {
			let mut lock = TEST_CINTERNAL.lock().unwrap();
			*lock += 1;
		}
		if a == b {
			// autorun drop code
			return true;
		}
		
		// autorun drop code
		false
	}
	
	assert_eq!(*TEST_CINTERNAL.lock().unwrap(), 0);
	let rin = __tinternal(1, 1);
	assert_eq!(rin, true);
	assert_eq!(*TEST_CINTERNAL.lock().unwrap(), 1);
}


#[test]
fn easy_use_twoargs() {
	static OLD_A: OnceLock<usize> = OnceLock::new();
	static OLD_B: OnceLock<usize> = OnceLock::new();
	
	fn __tinternal(a: usize, b: usize) -> bool {
		drop_code!((a: usize, b: usize) {
			OLD_A.set(*a).unwrap();
			OLD_B.set(*b).unwrap();
		});
		if a == b {
			// autorun drop code
			return true;
		}
		
		if a == 1 {
			// autorun drop code
			return true;
		}
		
		// autorun drop code
		false
	}
	
	assert_eq!(OLD_A.get(), None);
	assert_eq!(OLD_B.get(), None);
	let rin = __tinternal(1, 1);
	assert_eq!(rin, true);
	assert_eq!(OLD_A.get(), Some(&1));
	assert_eq!(OLD_B.get(), Some(&1));
}
