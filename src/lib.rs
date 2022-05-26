
//#![no_std]

pub mod core;

#[macro_export]
macro_rules! drop_code {
	[ $(#[ $meta:meta ]:)* $name_struct:ident ( $($args_in:tt),* $(,)? ) {$($drop_code:tt)*} ] => {
		#[allow(unused_mut)]
		#[allow(unused_variables)]
		#[allow(non_snake_case)]
		let mut $name_struct = {
			use $crate::core::DropCodeMarker;
			$crate::__drop_code_compareinargs! (
				#[allow(non_snake_case)]
				struct $name_struct {
					$($args_in),*
				}
				
				impl[$($args_in),*] DropCodeMarker for $name_struct {}
				
				impl[$($args_in),*] Drop for $name_struct {
					#[allow(unused_attributes)]
					$(#[$meta])*
					fn drop(&mut self) {
						$(
							#[allow(unused_variables)]
							#[allow(unused_mut)]
							let ref mut $args_in = self.$args_in;
						)*
						
						$($drop_code)*
					}
				}
			);
			
			
			$name_struct {
				$($args_in),*
			}
		};
		$(
			#[allow(unused_variables)]
			#[allow(unused_mut)]
			let ref mut $args_in = $name_struct.$args_in;
		)*
	};
	
	// ARGSIN
	[ $(#[ $meta:meta ]:)* ( $($args_in:ident),* $(,)? ) { $($drop_code:tt)* } ] => {
		$crate::drop_code! {
			#[inline]:
			$(#[$meta]:)*
			__DropCode( $($args_in),* ) { $($drop_code)* }
		}
	};
	[ $($drop_code:tt)* ] => {
		$crate::drop_code! {
			#[inline]:
			__DropCode() { $($drop_code)* }
		}
	};
	// ENDARGSIN
	
	[ $($unk:tt)* ] => {
		compile_error!(concat!(
			"Invalid syntax, ",
			stringify!($($unk)*)
		));
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __drop_code_compareinargs {
	[] => {};
	
	[ $(#[$meta:meta])* struct $name_struct: ident {} $($unk:tt)* ] => 									(#[repr(transparent)] $(#[$meta])* struct $name_struct {} $crate::__drop_code_compareinargs!{ $($unk)* } );
	[ $(#[$meta:meta])* struct $name_struct: ident {$a: ident} $($unk:tt)* ] => 							(#[repr(transparent)] $(#[$meta])* struct $name_struct<A> {$a: A} $crate::__drop_code_compareinargs!{ $($unk)* } );
	[ $(#[$meta:meta])* struct $name_struct: ident {$a: ident, $b:ident} $($unk:tt)* ] => 					($(#[$meta])* struct $name_struct<A, B> {$a: A, $b: B} $crate::__drop_code_compareinargs!{ $($unk)* } );
	[ $(#[$meta:meta])* struct $name_struct: ident {$a: ident, $b:ident, $c: ident} $($unk:tt)* ] => 			($(#[$meta])* struct $name_struct<A, B, C> {$a: A, $b: B, $c: C} $crate::__drop_code_compareinargs!{ $($unk)* } );
	[ $(#[$meta:meta])* struct $name_struct: ident {$a: ident, $b:ident, $c: ident, $d: ident} $($unk:tt)* ] => 	($(#[$meta])* struct $name_struct<A, B, C, D> {$a: A, $b: B, $d: D} $crate::__drop_code_compareinargs!{ $($unk)* } );
	
	[ impl[] $name_ty: ident $(for $name_struct: ident)? { $($incode:tt)* } $($unk:tt)* ] =>										(impl $name_ty $(for $name_struct)? { $($incode)* } $crate::__drop_code_compareinargs!{ $($unk)* } );
	[ impl[$a: ident] $name_ty: ident $(for $name_struct: ident)? { $($incode:tt)* } $($unk:tt)* ] =>								(impl<A> $name_ty $(for $name_struct<A>)? { $($incode)* } $crate::__drop_code_compareinargs!{ $($unk)* } );
	[ impl[$a: ident, $b:ident] $name_ty: ident $(for $name_struct: ident)? { $($incode:tt)* } $($unk:tt)* ] =>						(impl<A, B> $name_ty $(for $name_struct<A, B>)? { $($incode)* } $crate::__drop_code_compareinargs!{ $($unk)* } );
	[ impl[$a: ident, $b:ident, $c: ident] $name_ty: ident $(for $name_struct: ident)? { $($incode:tt)* } $($unk:tt)* ] =>				(impl<A, B, C> $name_ty $(for $name_struct<A, B, C>)? { $($incode)* } $crate::__drop_code_compareinargs!{ $($unk)* } );
	[ impl[$a: ident, $b:ident, $c: ident, $d: ident] $name_ty: ident $(for $name_struct: ident)? { $($incode:tt)* } $($unk:tt)* ] =>		(impl<A, B, C, D> $name_ty $(for $name_struct<A, B, C, D>)? { $($incode)* } $crate::__drop_code_compareinargs!{ $($unk)* } );
	
	[ $($unk:tt)+ ] => {
		compile_error!(concat!(
			"Invalid, ",
			stringify!($($unk)+)
		));
	};
}

#[cfg(test)]
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
