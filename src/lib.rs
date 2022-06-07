//Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

// #Ulin Project 2022
//
// *_---

#![no_std]

pub mod core;

/// A handy macro for quickly implementing code that will be executed on deletion. 
/// Replaces the tedious work of creating a hidden struct with data types inside a 
/// function and implementing the `Drop` hidden trait for that struct.
#[macro_export]
macro_rules! drop_code {
	[ $(#[ $meta:meta ]:)* $name_struct:ident ( $($($args_in:tt)+)? ) {$($drop_code:tt)*} ] => {
		#[allow(unused_mut)]
		#[allow(unused_variables)]
		#[allow(non_snake_case)]
		let mut $name_struct = {
			use $crate::core::DropCodeMarker;
			$crate::__drop_code_compareimpls! (
				#[allow(non_snake_case)]
				struct $name_struct {
					$($($args_in)*)?
				}
				
				impl[$($($args_in)*)?] DropCodeMarker for $name_struct {}
				
				impl[$($($args_in)*)?] Drop for $name_struct {
					#[allow(unused_attributes)]
					$(#[$meta])*
					fn drop(&mut self) {
						/*$(
							#[allow(unused_variables)]
							#[allow(unused_mut)]
							let ref mut $args_in = self.$args_in;
						)*/
						$(
							$crate::__drop_code_init_automut_refslet! {
								let ref automut self.( $($args_in)* )
							}
						)?
						
						$($drop_code)*
					}
				}
			);
			
			$crate::__drop_code_init_struct! {
				$name_struct {
					$($($args_in)*)?
				}
			}
		};
		$(
			$crate::__drop_code_init_alwaysmut_refslet! {
				let ref mut $name_struct.( $($args_in)* )
			}
		)?
	};
	
	// ARGSIN
	[ $(#[ $meta:meta ]:)* ( $($args_in:tt)* ) { $($drop_code:tt)* } ] => {
		$crate::drop_code! {
			#[inline]:
			$(#[$meta]:)*
			__DropCode( $($args_in)* ) { $($drop_code)* }
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
			"Invalid macro syntax, ",
			stringify!( $($unk)* )
		));
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __drop_code_init_struct {
	[ $name_struct: ident { $( $(mut)? $args_in:ident $(:$args_ty:ty)? ),* $(,)? } ] => {
		$name_struct {
			$($args_in),*
		}
	};
	[ $($unk:tt)* ] => {
		compile_error!(concat!(
			"Invalid macro syntax, ",
			stringify!( $($unk)* )
		));
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __drop_code_init_alwaysmut_refslet {
	[
		let ref mut $self: ident.( $( $(mut)? $args_in:ident $(:$args_ty:ty)? ),* $(,)? )
	] => {
		$(
			#[allow(unused_variables)]
			#[allow(unused_mut)]
			let ref mut $args_in = $self.$args_in;
		)*
	};
	[ $($unk:tt)* ] => {
		compile_error!(concat!(
			"Invalid macro syntax, ",
			stringify!( $($unk)* )
		));
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __drop_code_init_automut_refslet {
	[
		let ref automut $self: ident. (
			mut $args_in:ident $(:$args_ty:ty)? 
			
			$(, $($continue_args:tt)+)?
		)
	] => {
		#[allow(unused_variables)]
		#[allow(unused_mut)]
		let ref mut $args_in = $self.$args_in;
		
		$($crate::__drop_code_init_automut_refslet!(
			let ref automut $self.(
				$($continue_args)*
			)
		))?
	};
	[
		let ref automut $self: ident. (
			$args_in:ident $(:$args_ty:ty)? 
			
			$(, $($continue_args:tt)+)?
		)
	] => {
		#[allow(unused_variables)]
		#[allow(unused_mut)]
		let ref $args_in = $self.$args_in;
		
		$($crate::__drop_code_init_automut_refslet!(
			let ref automut $self.(
				$($continue_args)*
			)
		))?
	};
	
	[ // AUTOEND
		let ref automut $self: ident. (
			,
		)
	] => {};
	[ $($unk:tt)* ] => {
		compile_error!(concat!(
			"Invalid macro syntax, ",
			stringify!( $($unk)* )
		));
	};
}

/// Compound macro for seamless code implementation of `rust` drop_code.
/// support: struct<1 arg | 2 args | 3 args>, impl<1 arg | 2 args | 3 args>
#[doc(hidden)]
#[macro_export]
macro_rules! __drop_code_compareimpls {
	[] => {};
	
	[ $(#[$meta:meta])* struct $name_struct: ident { $(,)? } $($($unk:tt)+)? ] => (
		// struct $name_struct {}
		
		#[repr(transparent)] $(#[$meta])* 
		struct $name_struct {}
		
		$($crate::__drop_code_compareimpls!{ $($unk)+ })?
	);
	[ $(#[$meta:meta])* struct $name_struct: ident { $(mut)? $a1: ident $(:$a1ty:ty)? $(,)? } $($($unk:tt)+)? ] => (
		// struct $name_struct<A> {$a: A}
		
		#[repr(transparent)] $(#[$meta])* 
		$crate::__drop_code_compareimpls_ifexistsargsty_then!(
			// 1
			if1 $(:$a1ty)? =>
			[struct $name_struct {$a1: $($a1ty)?}]
			
			// EMPTY
			else [struct $name_struct<A> {$a1: A}]
		);
		
		$($crate::__drop_code_compareimpls!{ $($unk)+ })?
	);
	[ $(#[$meta:meta])* struct $name_struct: ident { $(mut)? $a1: ident $(:$a1ty:ty)?, $(mut)? $a2:ident $(:$a2ty:ty)? $(,)? } $($($unk:tt)+)? ] => (
		// struct $name_struct<A | {}, B | {}> {$a: A | $aty, $b: B | $bty}
		
		$(#[$meta])*
		$crate::__drop_code_compareimpls_ifexistsargsty_then!{
			// 1&2
			if2 $(:$a1ty)? $(:$a2ty)? =>
			[struct $name_struct {$a1: $($a1ty)?, $a2: $($a2ty)?}] elsem
			
			// 1 | 2
			if1 $(:$a1ty)? =>
			[struct $name_struct<A> {$a1: $($a1ty)?,	$a2: A}] elsem
			if1 $(:$a2ty)? =>
			[struct $name_struct<A> {$a2: $($a2ty)?,	$a1: A}]
			
			// EMPTY
			else [struct $name_struct<A, B> {			$a1: A, $a2: B}]
		}
		
		$($crate::__drop_code_compareimpls!{ $($unk)+ })?
	);
	[ $(#[$meta:meta])* struct $name_struct: ident { $(mut)? $a1: ident $(:$a1ty:ty)?, $(mut)? $a2:ident $(:$a2ty:ty)?, $(mut)? $a3: ident $(:$a3ty:ty)? $(,)? } $($($unk:tt)+)? ] => (
		// struct $name_struct<A | {}, B | {}, C | {}> {$a: A | $ty, $b: B | $ty, $c: C | $ty} 
		
		$(#[$meta])*
		$crate::__drop_code_compareimpls_ifexistsargsty_then!{
			// 1&2&3
			if3 $(:$a1ty)? $(:$a2ty)? $(:$a3ty)? =>
			[struct $name_struct {$a1: $($a1ty)?, $a2: $($a2ty)?, $a3: $($a3ty)?}] elsem
			
			// 1&2|2&3|3&1
			if2 $(:$a1ty)? $(:$a2ty)? =>
			[struct $name_struct<A> {$a1: $($a1ty)?, $a2: $($a2ty)?,	$a3: A}] elsem
			if2 $(:$a2ty)? $(:$a3ty)? =>
			[struct $name_struct<A> {$a2: $($a2ty)?, $a3: $($a3ty)?,	$a1: A}] elsem
			if2 $(:$a3ty)? $(:$a1ty)? =>
			[struct $name_struct<A> {$a3: $($a3ty)?, $a1: $($a1ty)?,	$a2: A}] elsem
			
			// 1|2|3
			if1 $(:$a1ty)? =>
			[struct $name_struct<A, B> {$a1: $($a1ty)?,				$a2: A, $a3: B}] elsem
			if1 $(:$a2ty)? =>
			[struct $name_struct<A, B> {$a2: $($a2ty)?,				$a1: A, $a3: B}] elsem
			if1 $(:$a3ty)? =>
			[struct $name_struct<A, B> {$a3: $($a3ty)?,				$a1: A, $a2: B}]
			
			// EMPTY
			else [struct $name_struct<A, B, C> {$a1: A, $a2: B, $a3: C}]
		}
		
		$($crate::__drop_code_compareimpls!{ $($unk)+ })?
	);
	
	[ impl[$(,)?] $name_ty: ident $(for $name_struct: ident)? { $($incode:tt)* } $($($unk:tt)+)? ] =>										(
		// impl $name_ty $(for $name_struct)? { $($incode)* }
		
		impl $name_ty $(for $name_struct)? { $($incode)* } 
		
		$($crate::__drop_code_compareimpls!{ $($unk)+ })?
	);
	[ impl[$(mut)? $a1: ident $(: $a1ty:ty)? $(,)?] $name_ty: ident $(for $name_struct: ident)? { $($incode:tt)* } $($($unk:tt)+)? ] =>								(
		// impl<A | {}> $name_ty $(for $name_struct<A | {}>)? { $($incode)* }
		
		$crate::__drop_code_compareimpls_ifexistsargsty_then!(
			if1 $(:$a1ty)? =>
			[impl $name_ty $(for $name_struct)? { $($incode)* }]
			
			else [impl<A> $name_ty $(for $name_struct<A>)? { $($incode)* }]
		);
		
		$($crate::__drop_code_compareimpls!{ $($unk)+ })?
	);
	[ impl[$(mut)? $a1: ident $(:$a1ty:ty)?, $(mut)? $a2:ident $(:$a2ty:ty)? $(,)?] $name_ty: ident $(for $name_struct: ident)? { $($incode:tt)* } $($($unk:tt)+)? ] =>						(
		// impl<A | {}, B | {}> $name_ty $(for $name_struct<A | {}, B | {}>)? { $($incode)* }
		
		$crate::__drop_code_compareimpls_ifexistsargsty_then!{
			if2 $(:$a1ty)? $(:$a2ty)? =>
			[impl $name_ty $(for $name_struct)? { $($incode)* }] elsem
			
			if1 $(:$a1ty)? =>
			[impl<A> $name_ty $(for $name_struct<A>)? { $($incode)* }] elsem
			if1 $(:$a2ty)? =>
			[impl<A> $name_ty $(for $name_struct<A>)? { $($incode)* }]
			
			else [impl<A, B> $name_ty $(for $name_struct<A, B>)? { $($incode)* }]
		}
		
		$($crate::__drop_code_compareimpls!{ $($unk)+ })?
	);
	[ impl[$(mut)? $a1: ident $(:$a1ty:ty)?, $(mut)? $a2:ident $(:$a2ty:ty)?, $(mut)? $a3:ident $(:$a3ty:ty)? $(,)?] $name_ty: ident $(for $name_struct: ident)? { $($incode:tt)* } $($($unk:tt)+)? ] =>				(
		// impl<A | {}, B | {}, C | {}> $name_ty $(for $name_struct<A | {}, B | {}, C | {}>)? { $($incode)* }
		
		$crate::__drop_code_compareimpls_ifexistsargsty_then!{
			// 1&2&3
			if3 $(:$a1ty)? $(:$a2ty)? $(:$a3ty)? =>
			[impl $name_ty $(for $name_struct)? { $($incode)* }] elsem
			
			// 1&2|2&3|3&1
			if2 $(:$a1ty)? $(:$a2ty)? =>
			[impl<A> $name_ty $(for $name_struct<A>)? { $($incode)* }] elsem
			if2 $(:$a2ty)? $(:$a3ty)? =>
			[impl<A> $name_ty $(for $name_struct<A>)? { $($incode)* }] elsem
			if2 $(:$a3ty)? $(:$a1ty)? =>
			[impl<A> $name_ty $(for $name_struct<A>)? { $($incode)* }] elsem
			
			// 1|2|3
			if1 $(:$a1ty)? =>
			[impl<A, B> $name_ty $(for $name_struct<A, B>)? { $($incode)* }] elsem
			if1 $(:$a2ty)? =>
			[impl<A, B> $name_ty $(for $name_struct<A, B>)? { $($incode)* }] elsem
			if1 $(:$a3ty)? =>
			[impl<A, B> $name_ty $(for $name_struct<A, B>)? { $($incode)* }]
			
			// EMPTY
			else [impl<A, B, C> $name_ty $(for $name_struct<A, B, C>)? { $($incode)* }]
		}
		
		$($crate::__drop_code_compareimpls!{ $($unk)+ })?
	);
	
	[ $($unk:tt)+ ] => {
		compile_error!(concat!(
			"Invalid macros syntax, ",
			stringify!($($unk)+)
		));
	};
}

/// Macro of the `rust` code tree or code tree for the following macro, 
/// which accepts conditions to compare the presence or absence of the macro's arguments.
/// The macro is read from top to bottom, if the condition is met, 
/// the macro tree is exited, in the case of `else` | `elsem` 
/// does everything related to `else` | `elsam`.
///
/// Syntax:
/// if1..=3 (3->number of input arguments with `?`) $(:$nty:ty)? (input arguments)
/// (order must be observed, first `then`, and only then `otherwise then`)
/// syntax_then:
/// => [rust_code]
/// => m[macro_code]
/// syntax_else_then:
/// else [rust_code]
/// elsem macro_code
/// 
/// 1 Example (in):
/// if3 $(:$a1ty)? $(:$a2ty)? $(:$a3ty)? =>
/// [struct $name_struct {$a1: $($a1ty)?, $a2: $($a2ty)?, $a3: $($a3ty)?}] elsem
/// if2 $(:$a1ty)? $(:$a2ty)? =>
/// [struct $name_struct<A> {$a1: $($a1ty)?, $a2: $($a2ty)?,	$a3: A}]
/// 
/// 1 Example (out):
/// if (is_exists($a1ty) && is_exists($a2ty) && is_exists($a3ty)) {
///		struct $name_struct {$a1: $a1ty, $a2: $a2ty, $a3: $a3ty}
/// } else
/// if (is_exists($a1ty) && is_exists($a2ty)) {
/// 		struct $name_struct<A> {$a1: $a1ty, $a2: $a2ty,	$a3: A}
/// }
#[doc(hidden)]
#[macro_export]
macro_rules! __drop_code_compareimpls_ifexistsargsty_then {

	[
		if1 :$nty:ty => $([$($next:tt)*])? $(m[$($nextm:tt)*])?
		
		// alwayselse
		$(else [$($invalid:tt)*])? $(elsem $($elsem:tt)*)?
	] => {
		// VALID, RUN_THEN
		
		$($($next)*)?
		$($crate::__drop_code_compareimpls_ifexistsargsty_then!{
			$($nextm)*
		})?
	};
	[
		if1 /**/ => $([$($next:tt)*])? $(m[$($nextm:tt)*])?
		
		// alwayselse
		$(else [$($invalid:tt)*])? $(elsem $($elsem:tt)*)?
	] => {
		// INVALID, RUN_ELSE_THEN
		
		$($($invalid)*)?
		$($crate::__drop_code_compareimpls_ifexistsargsty_then!{
			$($elsem)*
		})?
	};
	// END 1
	
	[ 
		if2 :$nty:ty :$nty2:ty => $([$($next:tt)*])? $(m[$($nextm:tt)*])?
		
		// alwayselse
		$(else [$($invalid:tt)*])? $(elsem $($elsem:tt)*)?
	] => {
		// VALID, RUN_THEN
		
		$($($next)*)?
		$($crate::__drop_code_compareimpls_ifexistsargsty_then!{
			$($nextm)*
		})?
	};
	[
		if2 :$nty:ty /**/ => $([$($next:tt)*])? $(m[$($nextm:tt)*])?
		
		// alwayselse
		$(else [$($invalid:tt)*])? $(elsem $($elsem:tt)*)?
	] => {
		// INVALID, RUN_ELSE_THEN
		
		$($($invalid)*)?
		$($crate::__drop_code_compareimpls_ifexistsargsty_then!{
			$($elsem)*
		})?
	};
	[
		if2 /**/ /**/ => $([$($next:tt)*])? $(m[$($nextm:tt)*])?
		
		// alwayselse
		$(else [$($invalid:tt)*])? $(elsem $($elsem:tt)*)?
	] => {
		// INVALID, RUN_ELSE_THEN
		
		$($($invalid)*)?
		$($crate::__drop_code_compareimpls_ifexistsargsty_then!{
			$($elsem)*
		})?
	};
	// END 2
	
	[
		if3 :$nty:ty :$nty2:ty :$nty3:ty => $([$($next:tt)*])? $(m[$($nextm:tt)*])?
		
		// alwayselse
		$(else [$($invalid:tt)*])? $(elsem $($elsem:tt)*)?
	] => {
		// VALID, RUN_THEN
		
		$($($next)*)?
		$($crate::__drop_code_compareimpls_ifexistsargsty_then!{
			$($nextm)*
		})?
	};
	[
		if3 :$nty:ty :$nty2:ty /**/ => $([$($next:tt)*])? $(m[$($nextm:tt)*])?
		
		// alwayselse
		$(else [$($invalid:tt)*])? $(elsem $($elsem:tt)*)?
	] => {
		// INVALID, RUN_ELSE_THEN
		
		$($($invalid)*)?
		$($crate::__drop_code_compareimpls_ifexistsargsty_then!{
			$($elsem)*
		})?
	};
	[
		if3 :$nty:ty /**/ /**/ => $([$($next:tt)*])? $(m[$($nextm:tt)*])?
		
		// alwayselse
		$(else [$($invalid:tt)*])? $(elsem $($elsem:tt)*)?
	] => {
		// INVALID, RUN_ELSE_THEN
		
		$($($invalid)*)?
		$($crate::__drop_code_compareimpls_ifexistsargsty_then!{
			$($elsem)*
		})?
	};
	[
		if3 /**/ /**/ /**/ => $([$($next:tt)*])? $(m[$($nextm:tt)*])?
		
		// alwayselse
		$(else [$($invalid:tt)*])? $(elsem $($elsem:tt)*)?
	] => {
		$($($invalid)*)?
		$($crate::__drop_code_compareimpls_ifexistsargsty_then!{
			$($elsem)*
		})?
	};
	
	[ $($unk:tt)+ ] => {
		compile_error!(concat!(
			"Invalid macro syntax, ",
			stringify!($($unk)+)
		))
	};
}
