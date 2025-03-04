//@aux-build:non-exhaustive-enum.rs
#![allow(
    clippy::manual_unwrap_or_default,
    clippy::manual_unwrap_or,
    clippy::redundant_pattern_matching
)]
#![warn(clippy::unneeded_struct_pattern)]

extern crate non_exhaustive_enum;
use non_exhaustive_enum::*;

fn noop() {}

fn main() {
    match Some(114514) {
        Some(v) => v,
        None {} => 0,
    };

    match Some(1919810) {
        Some(v) => v,
        None { .. } => 0,
    };

    match Some(123456) {
        Some(v) => v,
        None => 0,
    };

    match Some(Some(123456)) {
        Some(Some(v)) => v,
        Some(None {}) => 0,
        None {} => 0,
    };

    if let None {} = Some(0) {}
    if let None { .. } = Some(0) {}
    if let Some(None {}) = Some(Some(0)) {}
    let None {} = Some(0) else { panic!() };
    let None { .. } = Some(0) else { panic!() };
    let Some(None {}) = Some(Some(0)) else { panic!() };

    enum Custom {
        HasFields {
            field: i32,
        },
        HasBracketsNoFields {},
        NoBrackets,
        #[non_exhaustive]
        NoBracketsNonExhaustive,
        Init,
    };

    match Custom::Init {
        Custom::HasFields { field: value } => value,
        Custom::HasBracketsNoFields {} => 0,
        Custom::NoBrackets {} => 0, //~ ERROR: struct pattern is not needed for a unit variant
        Custom::NoBracketsNonExhaustive {} => 0, //~ ERROR: struct pattern is not needed for a unit variant
        _ => 0,
    };

    match Custom::Init {
        Custom::HasFields { field: value } => value,
        Custom::HasBracketsNoFields { .. } => 0,
        Custom::NoBrackets { .. } => 0, //~ ERROR: struct pattern is not needed for a unit variant
        Custom::NoBracketsNonExhaustive { .. } => 0, //~ ERROR: struct pattern is not needed for a unit variant
        _ => 0,
    };

    match Custom::Init {
        Custom::NoBrackets {} if true => 0, //~ ERROR: struct pattern is not needed for a unit variant
        _ => 0,
    };

    match Custom::Init {
        Custom::NoBrackets {} | Custom::NoBracketsNonExhaustive {} => 0, //~ ERROR: struct pattern is not needed for a unit variant
        _ => 0,
    };

    if let Custom::HasFields { field: value } = Custom::Init {
        noop();
    }
    if let Custom::HasBracketsNoFields {} = Custom::Init {
        noop();
    }
    if let Custom::HasBracketsNoFields { .. } = Custom::Init {
        noop();
    }
    if let Custom::NoBrackets {} = Custom::Init {
        //~^ ERROR: struct pattern is not needed for a unit variant
        noop();
    }
    if let Custom::NoBrackets { .. } = Custom::Init {
        //~^ ERROR: struct pattern is not needed for a unit variant
        noop();
    }
    if let Custom::NoBrackets {} | Custom::NoBracketsNonExhaustive {} = Custom::Init {
        //~^ ERROR: struct pattern is not needed for a unit variant
        noop();
    }
    if let Custom::NoBracketsNonExhaustive {} = Custom::Init {
        //~^ ERROR: struct pattern is not needed for a unit variant
        noop();
    }
    if let Custom::NoBracketsNonExhaustive { .. } = Custom::Init {
        //~^ ERROR: struct pattern is not needed for a unit variant
        noop();
    }

    let Custom::HasFields { field: value } = Custom::Init else {
        panic!()
    };

    let Custom::HasBracketsNoFields {} = Custom::Init else {
        panic!()
    };

    let Custom::HasBracketsNoFields { .. } = Custom::Init else {
        panic!()
    };
    let Custom::NoBrackets {} = Custom::Init else { panic!() }; //~ ERROR: struct pattern is not needed for a unit variant

    let Custom::NoBrackets { .. } = Custom::Init else {
        //~^ ERROR: struct pattern is not needed for a unit variant
        panic!()
    };
    let Custom::NoBracketsNonExhaustive {} = Custom::Init else {
        //~^ ERROR: struct pattern is not needed for a unit variant
        panic!()
    };
    let Custom::NoBracketsNonExhaustive { .. } = Custom::Init else {
        //~^ ERROR: struct pattern is not needed for a unit variant
        panic!()
    };

    enum Refutable {
        Variant,
    }

    fn pat_in_fn_param_1(Refutable::Variant {}: Refutable) {} //~ ERROR: struct pattern is not needed for a unit variant
    fn pat_in_fn_param_2(Refutable::Variant { .. }: Refutable) {} //~ ERROR: struct pattern is not needed for a unit variant

    for Refutable::Variant {} in [] {} //~ ERROR: struct pattern is not needed for a unit variant
    for Refutable::Variant { .. } in [] {} //~ ERROR: struct pattern is not needed for a unit variant
}

fn external_crate() {
    use ExtNonExhaustiveVariant::*;

    match ExhaustiveUnit {
        // Expected
        ExhaustiveUnit => 0,
        _ => 0,
    };

    match ExhaustiveUnit {
        // Exhaustive variant
        ExhaustiveUnit { .. } => 0, //~ ERROR: struct pattern is not needed for a unit variant
        _ => 0,
    };

    match ExhaustiveUnit {
        // Exhaustive variant
        ExhaustiveUnit {} => 0, //~ ERROR: struct pattern is not needed for a unit variant
        _ => 0,
    };

    match ExhaustiveUnit {
        ExhaustiveUnit => 0,
        // vvvvv Non-exhaustive variants, should all be ignored
        Unit { .. } => 0,
        Tuple { 0: field, .. } => field,
        StructNoField { .. } => 0,
        Struct { field, .. } => field,
        _ => 0,
    };
}
