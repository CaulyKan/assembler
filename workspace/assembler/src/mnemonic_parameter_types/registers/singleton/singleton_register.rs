// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


macro_rules! singleton_register
{
    ($name: tt) =>
    {
        /// Singleton reference to `$name` register.
        ///
        /// These singleton register variants are usually optimally more encoded.
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $name
        {
        	/// Singleton instance of this register.
        	O,
        }
        
        impl Default for $name
        {
        	#[inline(always)]
        	fn default() -> Self
        	{
        		$name::O
        	}
        }
    }
}

