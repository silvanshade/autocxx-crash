use autocxx::prelude::*;

include_cpp! {
    #include "gen.hxx"
    safety!(unsafe)
    generate!("Test")
}

pub(crate) use self::ffi::*;
