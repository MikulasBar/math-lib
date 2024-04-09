#![allow(non_snake_case, unused)]

use std::f64::consts::{E};

use crate::{
    fn_args, functions::*
};

#[test]
fn test_derivative_AddFn() {
    let func = AddFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = fn_args!(
        "x" => 8,
        "y" => 6
    );

    assert_eq!(func.apply(&args), Ok(14.0));
    assert_eq!(dfunc.apply(&args), Ok(1.0));
}

#[test]
fn test_derivative_SubFn() {
    let func = SubFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = fn_args!(
        "x" => 8,
        "y" => 6
    );

    assert_eq!(func.apply(&args), Ok(2.0));
    assert_eq!(dfunc.apply(&args), Ok(1.0));
}

#[test]
fn test_derivative_MulFn() {
    let func = MulFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = fn_args!(
        "x" => 2,
        "y" => 3
    );

    assert_eq!(func.apply(&args), Ok(6.0));
    assert_eq!(dfunc.apply(&args), Ok(3.0));
}

#[test]
fn test_derivative_DivFn() {
    let func = DivFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = fn_args!(
        "x" => 50,
        "y" => 10
    );

    assert_eq!(func.apply(&args), Ok(5.0));
    assert_eq!(dfunc.apply(&args), Ok(0.1));
}

#[test]
fn test_derivative_CoefFn() {
    let func = CoefFn::new(5.0, "x");
    let dfunc = func.derivative("x");

    let args = fn_args!(
        "x" => 6
    );

    assert_eq!(func.apply(&args), Ok(30.0));
    assert_eq!(dfunc.apply(&args), Ok(5.0));
}

#[test]
fn test_derivative_ExpFn() {
    let func = ExpFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = fn_args!(
        "x" => 2,
        "y" => 5
    );

    assert_eq!(func.apply(&args), Ok(32.0));
    assert_eq!(dfunc.apply(&args), Ok(80.0));
}

#[test]
fn test_derivative_LogFn() {
    let func = LogFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = fn_args!(
        "x" => E,
        "y" => E.powf(2.0)
    );

    assert_eq!(func.apply(&args), Ok(2.0));
    assert_eq!(dfunc.apply(&args), Ok(-2.0/E));
}