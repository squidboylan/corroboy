// Each type of operation has macros in its own file.
// This is to prevent a single file from getting too big, as there are many operations and each
// operation requires a minimum of 4 lines of code to create a macro. If an operation does not have
// a file.rs named after it, check the misc file as that is likely where it is


#[macro_use]
pub mod and;

#[macro_use]
pub mod or;

#[macro_use]
pub mod xor;

#[macro_use]
pub mod inc;

#[macro_use]
pub mod dec;

#[macro_use]
pub mod cp;

#[macro_use]
pub mod add;

#[macro_use]
pub mod sub;

#[macro_use]
pub mod swap;

#[macro_use]
pub mod misc;

#[macro_use]
pub mod ld;

#[macro_use]
pub mod push;

#[macro_use]
pub mod pop;

#[macro_use]
pub mod sbc;

#[macro_use]
pub mod adc;

#[macro_use]
pub mod rl;

#[macro_use]
pub mod rr;

#[macro_use]
pub mod bit;

#[macro_use]
pub mod res;

#[macro_use]
pub mod jump;

#[macro_use]
pub mod rst;

#[macro_use]
pub mod ret;

