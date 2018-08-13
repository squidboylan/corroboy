// Copyright (c) 2018 Caleb Boylan
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Each type of operation has macros in its own file.
// This is to prevent a single file from getting too big, as there are many operations and each
// operation requires a minimum of 4 lines of code to create a macro. If an operation does not have
// a file.rs named after it, check the misc file as that is likely where it is

#[macro_use]
pub mod and;
pub use self::and::*;

#[macro_use]
pub mod or;
pub use self::or::*;

#[macro_use]
pub mod xor;
pub use self::xor::*;

#[macro_use]
pub mod inc;
pub use self::inc::*;

#[macro_use]
pub mod dec;
pub use self::dec::*;

#[macro_use]
pub mod cp;
pub use self::cp::*;

#[macro_use]
pub mod add;
pub use self::add::*;

#[macro_use]
pub mod sub;
pub use self::sub::*;

#[macro_use]
pub mod swap;
pub use self::swap::*;

#[macro_use]
pub mod misc;
pub use self::misc::*;

#[macro_use]
pub mod ld;
pub use self::ld::*;

#[macro_use]
pub mod push;
pub use self::push::*;

#[macro_use]
pub mod pop;
pub use self::pop::*;

#[macro_use]
pub mod sbc;
pub use self::sbc::*;

#[macro_use]
pub mod adc;
pub use self::adc::*;

#[macro_use]
pub mod rl;
pub use self::rl::*;

#[macro_use]
pub mod rr;
pub use self::rr::*;

#[macro_use]
pub mod sl;
pub use self::sl::*;

#[macro_use]
pub mod sr;
pub use self::sr::*;

#[macro_use]
pub mod bit;
pub use self::bit::*;

#[macro_use]
pub mod res;
pub use self::res::*;

#[macro_use]
pub mod set;
pub use self::set::*;

#[macro_use]
pub mod jump;
pub use self::jump::*;

#[macro_use]
pub mod rst;
pub use self::rst::*;

#[macro_use]
pub mod ret;
pub use self::ret::*;
