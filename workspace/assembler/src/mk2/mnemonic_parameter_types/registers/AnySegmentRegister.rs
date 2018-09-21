// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// A segment register suitable for a memory operand.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnySegmentRegister
{
	#[allow(missing_docs)]
	_S(SegmentRegister),
	
	#[allow(missing_docs)]
	_FS(FS),
	
	#[allow(missing_docs)]
	_GS(GS),
}

impl Default for GeneralPurposeRegister
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::_S(SegmentRegister::default())
	}
}
