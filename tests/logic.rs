use drop_code::drop_code;
use std::sync::{Mutex, OnceLock};

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
	assert!(rin);
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
	assert!(rin);
	assert_eq!(OLD_A.get(), Some(&1));
	assert_eq!(OLD_B.get(), Some(&1));
}
