
use drop_code::drop_code;

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
