// finance - rust library (crate)
// GNU licensed, license file can be found at the root of the repository
// Copyright 2016 - Mohamed Hayibor

extern crate round;
use round::round;

//  present_value implementation
pub fn present_value(rate: f64, compounding_periods: f64, future_value: f64) -> f64 {
    let discount_factor: f64 = 1. + rate;

    round(future_value / (discount_factor.powf(compounding_periods)), 2)
}

#[test]
fn test_present_value() {
    let test_value = present_value(0.1, 1., 1000.);
    assert_eq!(test_value, 909.09);
}

//  future_value implementation
pub fn future_value(rate: f64, compounding_periods: f64, present_value: f64) -> f64 {
    let compound_factor: f64 = 1. + rate;

    round(present_value * (compound_factor.powf(compounding_periods)), 2)
}

#[test]
fn test_future_value() {
    let test_value = future_value(0.1, 1., 1000.);
    assert_eq!(test_value, 1100.00);
}

//  net_present_value implementation
//  Here cfs means cash_flows: it can be a slice or vector
//  cfs[0] being the cash flow at time 0
//  refer to, if you're not sure how this works: https://en.wikipedia.org/wiki/Net_present_value#Interpretation_as_integral_transform

/*
pub fn net_present_value(rate: f64, cfs: &[f64]) {
    let discount_factor = 1. + rate;
    let mut npv: f64;

    for n in cfs {
        npv += cfs[n as usize] / discount_factor.powf(n as f64);
    }
    
    round(npv, 2)
}

#[test]
fn test_net_present_value() {
    let test_npv = net_present_value(0.1, [-1000., 500., 500., 500.]);

    assert_eq!(test_npv, 243.43);
}
*/


//  Internal Rate of Return (IRR) implementation



//  Payback Period (PP) implementation





//  Return On Investment (ROI) implementation


//  Amortization implementation




//  Profitability Index



//  Rule of 72 (quick and dirty calculation to estimate when an investment will double: https://en.wikipedia.org/wiki/Rule_of_72
//  Please note that for consistency sake rate is getting passed as a plain not float and not as a percentage (%)
pub fn rule_of_72(rate: f64) -> f64 {
   round(72. / (rate * 100.), 2)
}

#[test]
fn test_rule_of_72() {
    assert_eq!(rule_of_72(0.035), 20.57);
}

// Rule of 70
pub fn rule_of_70(rate: f64) -> f64 {
   round(70. / (rate * 100.), 2)
}

#[test]
fn test_rule_of_70() {
    assert_eq!(rule_of_70(0.035), 20.);
}

