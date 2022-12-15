// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.

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

use crate::ByteEmitter::ByteEmitter;

pub(crate) trait Displacement {
    fn emit(self, byte_emitter: &mut ByteEmitter);
}

impl Displacement for u8 {
    #[inline(always)]
    fn emit(self, byte_emitter: &mut ByteEmitter) {
        byte_emitter.emit_u8(self)
    }
}

impl Displacement for u16 {
    #[inline(always)]
    fn emit(self, byte_emitter: &mut ByteEmitter) {
        byte_emitter.emit_u16(self)
    }
}

impl Displacement for u32 {
    #[inline(always)]
    fn emit(self, byte_emitter: &mut ByteEmitter) {
        byte_emitter.emit_u32(self)
    }
}

impl Displacement for u64 {
    #[inline(always)]
    fn emit(self, byte_emitter: &mut ByteEmitter) {
        byte_emitter.emit_u64(self)
    }
}
