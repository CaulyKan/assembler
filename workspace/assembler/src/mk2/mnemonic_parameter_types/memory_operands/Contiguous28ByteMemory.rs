// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// An 'array' of 28 bytes in memory referenced by `MemoryOperand`.
///
/// Used only by x87 Floating Point Unit (FPU) instructions.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Contiguous28ByteMemory(pub MemoryOperand);

impl MemoryOrRegister for Contiguous28ByteMemory
{
	/// Value.
	#[inline(always)]
	fn value(self) -> u8
	{
		self.0.value()
	}
}
