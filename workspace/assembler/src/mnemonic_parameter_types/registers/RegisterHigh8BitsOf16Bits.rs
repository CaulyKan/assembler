// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// High 8-bits of the first four general purpose registers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RegisterHigh8BitsOf16Bits
{
	/// Register 0.
	AH = 4,
	
	/// Register 1.
	///
	/// The fourth function argument in the System V Application Binary Interface for x86-64.
	CH = 5,
	
	/// Register 2.
	///
	/// The third function argument in the System V Application Binary Interface for x86-64.
	DH = 6,
	
	/// Register 3.
	BH = 7,
}

impl Default for RegisterHigh8BitsOf16Bits
{
	#[inline(always)]
	fn default() -> Self
	{
		RegisterHigh8BitsOf16Bits::AH
	}
}

impl Into<u8> for RegisterHigh8BitsOf16Bits
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}

impl Register for RegisterHigh8BitsOf16Bits
{
	#[inline(always)]
	fn index(self) -> u8
	{
		self.into()
	}
}

impl GeneralPurposeRegister for RegisterHigh8BitsOf16Bits
{
}
