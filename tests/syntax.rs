use drop_code::drop_code;
use std::sync::OnceLock;

#[allow(clippy::size_of_ref)]
#[test]
fn auto_test_syntax() {
	{
		// AnonEmpty
		static CHECK: OnceLock<bool> = OnceLock::new();
		{
			drop_code! {
				CHECK.set(true).unwrap();
			};
			// auto_run_drop_code
		}
		assert_eq!(CHECK.get(), Some(&true));
	}

	{
		// EmptyBlock
		static CHECK: OnceLock<bool> = OnceLock::new();
		{
			drop_code!(() {
				CHECK.set(true).unwrap();
			});
			// auto_run_drop_code
		}
		assert_eq!(CHECK.get(), Some(&true));
	}

	{
		// EmptyBlock + meta for drop_trait
		static CHECK: OnceLock<bool> = OnceLock::new();
		{
			drop_code!(#[inline(always)]: () {
				CHECK.set(true).unwrap();
			});
		}
		assert_eq!(CHECK.get(), Some(&true));
	}

	{
		// Arg1Block + str type arg
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

	{
		// Arg2Block + unk type arg
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
		drop_code!((test, test2) {
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
