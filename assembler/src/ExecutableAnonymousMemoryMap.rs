// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.

use std::ptr;

use crate::mnemonic_parameter_types::immediates::*;
use crate::mnemonic_parameter_types::memory::*;
use crate::mnemonic_parameter_types::memory_offsets::*;
use crate::mnemonic_parameter_types::registers::*;
use crate::mnemonic_parameter_types::relative_addresses::*;
use crate::mnemonic_parameter_types::*;
use crate::ExecutableAnonymousMemoryMapCreationError::ExecutableAnonymousMemoryMapCreationError;
use crate::InstructionStream::InstructionStream;
use crate::InstructionStreamHints::InstructionStreamHints;
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
#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::*;
#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Memory::*;
/// Represents an executable memory map that can be used to generate program code into.
#[derive(Debug)]
pub struct ExecutableAnonymousMemoryMap {
    pub address: *mut u8,
    pub length: usize,
    ignore_mlock_failure: bool,
}

impl Drop for ExecutableAnonymousMemoryMap {
    #[inline(always)]
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        {
            unsafe {
                VirtualFree(self.address as *mut _, 0, MEM_RELEASE);
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            unsafe { munmap(self.address as *mut _, self.length) };
        }
    }
}

impl ExecutableAnonymousMemoryMap {
    /// Create a new instance.
    ///
    /// `length` is rounded up to the nearest power of two, and is floored at the smallest page size (4Kb).
    ///
    /// On Linux, `allocate_in_first_2Gb` should ideally be `true`.
    ///
    /// On Linux, there are historic bugs in `mlock` which may require `ignore_mlock_failure` to be `true`.
    ///
    /// Memory is created using an anonymous, shared mmap with no access rights (not even read) which is then locked (`mlock`'d).
    #[cfg_attr(
        not(any(target_os = "android", target_os = "linux")),
        allow(unused_variables)
    )]
    //#[inline(always)]
    pub fn new(
        length: usize,
        allocate_in_first_2Gb: bool,
        ignore_mlock_failure: bool,
    ) -> Result<Self, ExecutableAnonymousMemoryMapCreationError> {
        use self::ExecutableAnonymousMemoryMapCreationError::*;

        const PageSize: usize = 4096;
        const NoFileDescriptor: i32 = -1;
        const NoOffset: i64 = 0;

        let aligned_length = if length < PageSize {
            PageSize
        } else {
            length.next_power_of_two()
        };

        #[cfg(target_os = "windows")]
        {
            // let h = unsafe {
            //     CreateFileMappingA(
            //         INVALID_HANDLE_VALUE,
            //         ptr::null(),
            //         PAGE_EXECUTE_READWRITE | SEC_COMMIT,
            //         0,
            //         u32::MAX / 1024,
            //         ptr::null(),
            //     )
            // };

            // if h == 0 {
            //     panic!("CreateFileMappingA failed with {}", unsafe {
            //         GetLastError()
            //     });
            // }

            let addr: u64 = 0;
            let address = unsafe {
                VirtualAlloc(
                    addr as *const _,
                    (u32::MAX / 1024) as usize,
                    MEM_COMMIT,
                    PAGE_EXECUTE_READWRITE,
                )
            };

            Ok(Self {
                address: address as *mut _,
                length,
                ignore_mlock_failure,
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            let flags = {
                #[cfg(any(target_os = "android", target_os = "linux"))]
                {
                    let mut flags = MAP_ANON | MAP_SHARED;
                    if allocate_in_first_2Gb {
                        flags |= MAP_32BIT
                    }
                    flags
                }

                #[cfg(not(any(target_os = "android", target_os = "linux")))]
                {
                    MAP_ANON | MAP_SHARED
                }
            };

            let result = unsafe {
                mmap(
                    null_mut(),
                    aligned_length,
                    PROT_NONE,
                    flags,
                    NoFileDescriptor,
                    NoOffset,
                )
            };
            if result == MAP_FAILED {
                Err(MMapFailed(io::Error::last_os_error(), aligned_length))
            } else {
                let address = result;
                let result = unsafe { mlock(address, length) };
                if !ignore_mlock_failure && result != 0 {
                    if result == -1 {
                        return Err(MLockFailed(io::Error::last_os_error(), aligned_length));
                    } else {
                        panic!("Unexpected result code from mlock (new) '{}'", result)
                    }
                }

                Ok(Self {
                    address: address as *mut _,
                    length,
                    ignore_mlock_failure,
                })
            }
        }
    }

    /// Get an assembler to this anonymous map.
    #[inline(always)]
    pub fn instruction_stream(
        &mut self,
        instruction_stream_hints: &InstructionStreamHints,
    ) -> InstructionStream {
        InstructionStream::new(self, &instruction_stream_hints)
    }

    #[inline(always)]
    pub(crate) fn make_writable(&mut self) {
        #[cfg(target_os = "windows")]
        {}
        #[cfg(not(target_os = "windows"))]
        {
            self.mprotect(self.address, self.length, PROT_WRITE)
        }
    }

    //#[inline(always)]
    pub(crate) fn make_executable(&mut self) {
        #[cfg(target_os = "windows")]
        {
            unsafe {
                let mut old: u32 = 0;
                let pold: *mut u32 = &mut old;
                VirtualProtect(
                    self.address as *const _,
                    (u32::MAX / 1024) as usize,
                    PAGE_EXECUTE,
                    pold,
                );
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.mprotect(self.address, self.length, PROT_EXEC)
        }
    }

    #[cfg(target_os = "windows")]
    #[inline(always)]
    pub(crate) fn attempt_to_resize_in_place_whilst_writing(&mut self) -> io::Result<usize> {
        let new_length = self.length * 2;
        if new_length > (u32::MAX / 1024) as usize {
            panic!("memory more than u32::MAX/1024");
        }
        self.length = new_length;
        Ok(self.length)
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline(always)]
    pub(crate) fn attempt_to_resize_in_place_whilst_writing(&mut self) -> io::Result<usize> {
        const NoFlags: i32 = 0;
        let old_length = self.length;
        let new_length = self.length * 2;
        let old_address = self.address;
        let new_address = unsafe { mremap(old_address as *mut _, old_length, new_length, NoFlags) };
        if new_address == MAP_FAILED {
            Err(io::Error::last_os_error())
        } else {
            debug_assert_eq!(new_address as *mut u8, old_address, "address has changed");

            let new_memory_address = unsafe { old_address.add(old_length) };

            let result = unsafe { mlock(new_memory_address as *mut _, old_length) };
            if !self.ignore_mlock_failure && result != 0 {
                if result == -1 {
                    return Err(io::Error::last_os_error());
                } else {
                    panic!("Unexpected result code from mlock (attempt_to_resize_in_place_whilst_writing) '{}'", result)
                }
            }

            self.mprotect(new_memory_address, old_length, PROT_WRITE);
            self.length = new_length;
            Ok(new_length)
        }
    }

    #[inline(always)]
    fn mprotect(&self, address: *mut u8, length: usize, protection_flags: i32) {
        #[cfg(target_os = "windows")]
        {}
        #[cfg(not(target_os = "windows"))]
        {
            unsafe { mprotect(address as *mut _, length, protection_flags) };
        }
    }
}
