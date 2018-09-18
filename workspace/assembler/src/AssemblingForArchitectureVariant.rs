// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Represents the architecture variant we are assembling for.
///
/// Default to Long mode with all x86-64 features enabled.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssemblingForArchitectureVariant
{
	/// Are we using Long mode or Protected mode?
	pub mode: SupportedOperationalMode,
	
	/// Which combination of features are being used?
	pub features: CpuFeatures,
}

impl Default for AssemblingForArchitectureVariant
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			mode: Default::default(),
			features: CpuFeatures::All,
		}
	}
}

impl AssemblingForArchitectureVariant
{
	#[inline(always)]
	pub(crate) fn default_address_size(&self) -> AddressSize
	{
		self.mode.default_address_size()
	}
	
	#[inline(always)]
	pub(crate) fn address_size_override_prefix_required(self, address_size: AddressSize) -> bool
	{
		self.mode.address_size_override_prefix_required(address_size)
	}
	
	#[inline(always)]
	pub(crate) fn is_for_protected_mode(&self) -> bool
	{
		self.mode == SupportedOperationalMode::Protected
	}
	
	#[inline(always)]
	pub(crate) fn not_supported_in_operational_mode(&self, not_supported_in_long_mode: bool) -> bool
	{
		not_supported_in_long_mode && self.mode == SupportedOperationalMode::Long
	}
	
	#[inline(always)]
	pub(crate) fn does_not_support_one_or_more_features(&self, features_required: CpuFeatures) -> bool
	{
		!self.features.contains(features_required)
	}
}
