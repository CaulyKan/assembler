// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.
use crate::mnemonic_parameter_types::immediates::*;
use crate::mnemonic_parameter_types::memory::*;
use crate::mnemonic_parameter_types::memory_offsets::*;
use crate::mnemonic_parameter_types::registers::*;
use crate::mnemonic_parameter_types::relative_addresses::*;
use crate::mnemonic_parameter_types::*;
use libc::*;
use std::alloc::alloc;
use std::alloc::dealloc;
use std::alloc::realloc;
use std::alloc::Layout;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::mem::align_of;
use std::mem::size_of;
use std::mem::transmute;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Not;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ptr::copy_nonoverlapping;
use std::ptr::null_mut;
use std::ptr::NonNull;
use std::slice::from_raw_parts;

/// Hints to use when creating an instruction stream.
///
/// Also can be retrieved using `InstructionStream.hints_for_next_instance()`, to fine-tune future instances to avoid reallocations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstructionStreamHints
{
	/// Number of labels.
	pub number_of_labels: usize,
	
	/// Number of 8-bit jumps.
	pub number_of_8_bit_jumps: usize,
	
	/// Number of 32-bit jumps.
	pub number_of_32_bit_jumps: usize,
	
	/// Number of emitted labels.
	pub number_of_emitted_labels: usize,
}

impl Default for InstructionStreamHints
{
	#[inline(always)]
	fn default() -> Self
	{
		InstructionStreamHints
		{
			number_of_labels: Self::MinimumValue,
			number_of_8_bit_jumps: Self::MinimumValue,
			number_of_32_bit_jumps: Self::MinimumValue,
			number_of_emitted_labels: Self::MinimumValue,
		}
	}
}

impl InstructionStreamHints
{
	const MinimumValue: usize = 4096;
	
	/// Ensures that sensible minimals are present.
	#[inline(always)]
	pub fn adjust(&mut self)
	{
		self.number_of_labels = Self::adjust_value(self.number_of_labels);
		self.number_of_8_bit_jumps = Self::adjust_value(self.number_of_8_bit_jumps);
		self.number_of_32_bit_jumps = Self::adjust_value(self.number_of_32_bit_jumps);
		self.number_of_emitted_labels = Self::adjust_value(self.number_of_emitted_labels);
	}
	
	/// Ensures that a maximum high-water-mark is kept.
	#[inline(always)]
	pub fn maximize(&mut self, mut newer_unadjusted_hints: Self)
	{
		newer_unadjusted_hints.adjust();
		
		if newer_unadjusted_hints.number_of_labels > self.number_of_labels
		{
			self.number_of_labels = newer_unadjusted_hints.number_of_labels
		}
		
		if newer_unadjusted_hints.number_of_8_bit_jumps > self.number_of_8_bit_jumps
		{
			self.number_of_8_bit_jumps = newer_unadjusted_hints.number_of_8_bit_jumps
		}
		
		if newer_unadjusted_hints.number_of_32_bit_jumps > self.number_of_32_bit_jumps
		{
			self.number_of_32_bit_jumps = newer_unadjusted_hints.number_of_32_bit_jumps
		}
		
		if newer_unadjusted_hints.number_of_emitted_labels > self.number_of_emitted_labels
		{
			self.number_of_emitted_labels = newer_unadjusted_hints.number_of_emitted_labels
		}
	}
	
	fn adjust_value(value: usize) -> usize
	{
		if value < Self::MinimumValue
		{
			Self::MinimumValue
		}
		else
		{
			value.next_power_of_two()
		}
	}
}
