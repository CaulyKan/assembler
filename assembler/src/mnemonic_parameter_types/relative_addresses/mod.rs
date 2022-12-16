// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.

use crate::ByteEmitter::ByteEmitter;
use crate::ExecutableAnonymousMemoryMap::ExecutableAnonymousMemoryMap;
use crate::InstructionPointer::InstructionPointer;
use crate::InstructionStream::InstructionStream;
use crate::InstructionStreamHints::InstructionStreamHints;
use crate::LabelledLocations::LabelledLocations;
use crate::ShortJmpResult::ShortJmpResult;

use super::*;

include!("impl_relative_address.rs");

include!("RelativeAddress.rs");
include!("RelativeAddress8Bit.rs");
include!("RelativeAddress32Bit.rs");