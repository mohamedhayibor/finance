// finance - rust library (crate)
// GNU licensed, license file can be found at the root of the repository
// Copyright 2016 - Mohamed Hayibor

extern crate round;
use round::round;

//  present_value implementation (PV)
pub fn present_value(rate: f64, compounding_periods: f64, future_value: f64) -> f64 {
    let discount_factor: f64 = 1. + rate;

    future_value / (discount_factor.powf(compounding_periods))
}

#[test]
fn test_present_value() {
    let test_value = present_value(0.1, 1., 1000.);
    assert_eq!( round(test_value, 2), 909.09);
}

//  future_value implementation (FV)
pub fn future_value(rate: f64, compounding_periods: f64, present_value: f64) -> f64 {
    let compound_factor: f64 = 1. + rate;

    present_value * (compound_factor.powf(compounding_periods))
}

#[test]
fn test_future_value() {
    let test_value = future_value(0.1, 1., 1000.);
    assert_eq!( round(test_value, 2), 1100.00);
}

//  net_present_value implementation
//  Here cfs means cash_flows: it can be a slice or vector
//  cfs[0] being the cash flow at time 0
//  refer to, if you're not sure how this works: https://en.wikipedia.org/wiki/Net_present_value#Interpretation_as_integral_transform
pub fn net_present_value(rate: f64, cfs: &[f64]) -> f64 {
    let discount_factor = 1. + rate;
    let mut npv: f64 = 0.;

    for n in 0..cfs.len() {
        npv += cfs[n] / discount_factor.powf(n as f64);
    }

    npv
}

#[test]
fn test_net_present_value() {
    let test_npv = net_present_value(0.1, &[-1000., 500., 500., 500.]);

    assert_eq!(round(test_npv, 2), 243.43);
}

// payment implementation > PMT in formulas
pub fn payment(present_value: f64, number_of_compounding: f64, rate: f64) -> f64 {
    present_value / ( (1. - (1. / (1. + rate).powf(number_of_compounding)) ) / rate )
}

#[test]
fn main () {
    let test_value = payment(190000., 30.0, 0.08);
    assert_eq!(round(test_value, 2), 16877.21);
}

// implementing periodic_interest_rate
pub fn periodic_interest_rate(annual_percentage_rate: f64, number_of_compounding: f64) -> f64 {
    annual_percentage_rate / number_of_compounding
}

#[test]
fn test_periodic_interest_rate() {
    let test_value = periodic_interest_rate(0.10, 4.);

    assert_eq!(round(test_value, 3), 0.025);
}

// implementing HPR > holding period return
pub fn holding_period_return(profit: f64, cost: f64) -> f64 {
    profit / cost
}

#[test]
fn test_hpr() {
    let test_value = holding_period_return(5000., 4000.);
    assert_eq!(test_value, 1.25);
}

// implementing number of compounding > has the notation of n in formulas
pub fn number_of_compounding(future_value: f64, present_value: f64, rate: f64) -> f64 {
    (future_value / present_value).ln() / (1. + rate).ln()
}

#[test]
fn test_number_of_compounding() {
    let test_value = number_of_compounding(5000., 4000., 0.02);

    assert_eq!(round(test_value, 2), 11.27);
}

//  Return On Investment (ROI) implementation
// no rounding as ROI is super sensitive
pub fn return_on_investment(earnings: f64, cf0: f64) -> f64 {
    (earnings - cf0.abs()) / cf0.abs()
}

#[test]
fn test_roi() {
    let test_value = round( return_on_investment(5000., 4000.), 2);

    assert_eq!(test_value, 0.25);
}

// implementing interest_rate sometimes called growth rate or discount rate
pub fn interest_rate(future_value: f64, present_value: f64, number_of_compounding: f64) -> f64 {

    // recip > takes the inverse of a number
    (future_value / present_value).powf( number_of_compounding.recip() ) - 1.
}

#[test]
fn test_interest_rate() {
    let test_value = interest_rate(5000., 4000., 4.);
    assert_eq!(round(test_value, 4), 0.0574);
}


//  Rule of 72 (quick and dirty calculation to estimate when an investment will double: https://en.wikipedia.org/wiki/Rule_of_72
//  Please note that for consistency sake rate is getting passed as a plain not float and not as a percentage (%)
pub fn rule_of_72(rate: f64) -> f64 {
    72. / (rate * 100.)
}

#[test]
fn test_rule_of_72() {
    assert_eq!( round( rule_of_72(0.035), 2) , 20.57);
}

// Rule of 70
pub fn rule_of_70(rate: f64) -> f64 {
   70. / (rate * 100.)
}

#[test]
fn test_rule_of_70() {
    assert_eq!( round(rule_of_70(0.035), 2) , 20.);
}

//  Leverage ratio (LR) 
pub fn leverage_ratio(total_liabilities: f64, total_debts: f64, total_income: f64) -> f64 {
    ((total_liabilities + total_debts) / total_income)
}

#[test]
fn test_leverage_ratio() {
    let test_ratio = leverage_ratio(1000., 2000., 4000.);
    assert_eq!( round( test_ratio, 2) , 0.75);
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

    ((e / v) * re ) + (((d / v) * rd ) * (1. - t))
}

#[test]
fn test_wacc() {
    // implement an example later
    let test_value = weighted_cost_of_capital(2000000.00, 1000000.00, 0.07, 0.05, 0.4);
    assert_eq!( round(test_value, 4), 0.0567);
}

// implementing effective_annual_rate > EAR
pub fn effective_annual_rate(annual_rate: f64, number_of_compounding: f64) -> f64 {
    (1. + (annual_rate / number_of_compounding)).powf(number_of_compounding) - 1. 
}

#[test]
fn test_effective_annual_rate () {
    let test = effective_annual_rate(0.05, 12.);
    assert_eq!(round(test, 4), 0.0512);
}

