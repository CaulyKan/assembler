// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ProtectedModeRelocationLocation
{
	pub(crate) assembly_offset: AssemblyOffset,
	pub(crate) relocation: ProtectedModeRelocation,
}

impl ProtectedModeRelocationLocation
{
	#[inline(always)]
	pub(crate) fn set_relocation_value(&self, start_instructions_pointer: *mut u8, target_address: usize)
	{
		let pointer_to_location_to_patch = self.pointer_to_location_to_patch(start_instructions_pointer);
		
		use self::ProtectedModeRelocationKind::*;
		
		let address = match self.relocation.relocation_kind
		{
			Relative => (target_address as u32).wrapping_sub(start_instructions_pointer as u32 + self.assembly_offset as u32),
			
			Absolute => unimplemented!(),
			
			Extern => unimplemented!(),
		};
		
		self.relocation.overwrite_address_at(pointer_to_location_to_patch, address)
	}
	
	#[inline(always)]
	fn pointer_to_location_to_patch(&self, start_instructions_pointer: *mut u8) -> *mut u8
	{
		let relative_offset_to_location_to_patch = self.assembly_offset - self.relocation.subtract_to_get_starting_pointer_put_relocation_at();
		
		unsafe { start_instructions_pointer.offset(relative_offset_to_location_to_patch as isize) }
	}
}
