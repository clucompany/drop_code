
/// A marker that defines automatically generated structures 
/// that participate only in the drop.
pub trait DropCodeMarker {}

/*
use core::fmt::Debug;
use core::fmt::Display;
use core::ops::DerefMut;
use core::ops::Deref;
*/

/*
!!!
!!! Not supported in rust code
!!!
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DropCodeCapturedArg<'a, T>(&'a mut T);

impl<'a, T> Debug for DropCodeCapturedArg<'a, T> where T: Debug {
	#[inline(always)]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		Debug::fmt(self.as_data(), f)
	}
}

impl<'a, T> Display for DropCodeCapturedArg<'a, T> where T: Display {
	#[inline(always)]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		Display::fmt(self.as_data(), f)
	}
}

impl<'a, T> DropCodeCapturedArg<'a, T> {
	#[inline(always)]
	pub /*const*/ fn new(a: &'a mut T) -> Self {
		Self(a)
	}
	
	#[inline(always)]
	pub fn as_data(&self) -> &T {
		&self.0
	}
	
	#[inline(always)]
	pub fn as_mut_data(&mut self) -> &mut T {
		&mut self.0
	}
}

default impl<'a, T> Deref for DropCodeCapturedArg<'a, T> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_data()
	}
}

default impl<'a, T> DerefMut for DropCodeCapturedArg<'a, T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_data()
	}
}

impl<'a, 'b, T> Deref for DropCodeCapturedArg<'a,
	DropCodeCapturedArg<'b, T>
> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.deref()
	}
}

impl<'a, 'b, T> DerefMut for DropCodeCapturedArg<'a,
	DropCodeCapturedArg<'b, T>
> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.deref_mut()
	}
}

impl<'a, 'b, 'c, T> Deref for DropCodeCapturedArg<'a,
	DropCodeCapturedArg<'b,
		DropCodeCapturedArg<'c, T>
	>
> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.deref().deref()
	}
}

impl<'a, 'b, 'c, T> DerefMut for DropCodeCapturedArg<'a, 
	DropCodeCapturedArg<'b, 
		DropCodeCapturedArg<'c, T>
	>
> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_data()
	}
}*/