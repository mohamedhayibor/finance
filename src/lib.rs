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

/*  Will implement IRR > later - haven't found the most effective way
//  Internal Rate of Return (IRR) implementation
pub fn internal_rate_of_return() {
    unimplemented!();
}

#[test]
fn test_irr() {
    let test_value = internal_rate_of_return();
    assert_eq!(test_value, 1000.);
}
*/


//  Payback Period (PP) implementation
//  How long will it take you to get the money back from your investment
//  cfs means cash flows > streams of cash :)
pub fn payback_period(number_of_periods: f64, cfs: &[f64]) -> f64 {
    
}

#[test]
fn test_PP() {
    let test_value = payback_period(3., &[-1000.00, 300.00, 500.00, 500.00]);

    assert_eq!(test_value, 3.);
}

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

//  Leverage ratio (LR) 
pub fn leverage_ratio(total_liabilities: f64, total_debts: f64, total_income: f64) -> f64 {
    round( ((total_liabilities + total_debts) / total_income), 2)
}

#[test]
fn test_leverage_ratio() {
    let test_ratio = leverage_ratio(1000., 2000., 4000.);
    assert_eq!(test_ratio, 0.75);
}

//  Weighted Cost of capital (WACC)
//  Be aware that decimal formats are expected to be passed and not percentages.
pub fn weighted_cost_of_capital(market_value_of_equity: f64, market_value_of_debt: f64, cost_of_equity: f64, cost_of_debt: f64, tax_rate: f64) -> f64 {
    // champion of verbosity, lets abrreviate
    let e = market_value_of_equity;
    let d = market_value_of_debt;
    // v: market value of the entity, no worries it is not taking owership as primitives have the copy trait implemented
    let v = e + d;
    let re = cost_of_equity;
    let rd = cost_of_debt;
    let t = tax_rate;

    let wacc = ((e / v) * re ) + (((d / v) * rd ) * (1. - t));
    round(wacc, 4)
}

#[test]
fn test_wacc() {
    // implement an example later
    let test_value = weighted_cost_of_capital(2000000.00, 1000000.00, 0.07, 0.05, 0.4);
    assert_eq!(test_value, 0.0567);
}

// PMT: amount of loan to pay assuming a constant interest rate
// a negative value represent a negative cash flow in Finance, meaning that you have to pay out that particular amount (in another words, money from your pockets, going into someonelse
pub fn PMT(rate: f64, number_of_payments: f64, principal: f64) -> f64 {
    let discount_factor = 1. + rate;

    round( -principal * rate / ( 1. - discount_factor.powf(-number_of_payments) ), 2)
}

#[test]
fn test_PMT() {
    // lets consider the case where you are getting a $50,000.00 laon for 30 years at the rate of 5%
    let test_value = PMT(0.05, 360., 50000.00);

    assert_eq!(test_value, 3000.);
}

// Compound Annual Growth Rate: CAGR
pub fn compound_annual_rate(beginning_value: f64, ending_value: f64, number_of_periods: f64) -> f64 {
    let delta = ending_value / beginning_value;

    let CAGR = delta.powf( 1. / number_of_periods) - 1.;
    // Alert highly double check the formula
    round(CAGR, 2)
}

#[test]
fn test_CAGR() {
    let test_value = compound_annual_rate(20000.00, 15000.00, 3.);
    // PLEASE double check formula before
    assert_eq!(test_value, 12.88);
}
