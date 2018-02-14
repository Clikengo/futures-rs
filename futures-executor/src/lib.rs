//! TODO: dox

#![no_std]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/futures-executor/0.2")]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

macro_rules! if_std {
    ($($i:item)*) => ($(
        #[cfg(feature = "std")]
        $i
    )*)
}

if_std! {
    extern crate futures_core;
    extern crate futures_util;

    mod thread;

    mod local_pool;
    pub use local_pool::{block_on, LocalPool, LocalExecutor};

    mod enter;
    pub use enter::{enter, Enter, EnterError};
}
