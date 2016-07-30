# Finance (NOT FINISHED - COME BACK LATER)

This crate (Rust library) provides utility functions for the most common operations (calculations) in Finance.

## Usage

Include this library with the following:
```rust
[dependencies]
finance = "0.0.0" // preferrably the latest version on crates.io
```
## Examples
```rust
extern crate finance;
extern crate round;

use round::round;
use finance::{ present_value, future_value };

fn main() {
    let test_value = round(present_value(0.1, 1., 1000.), 2); // 909.09

    let test_value = round(future_value(0.1, 1., 1000.), 2);  // 1100.00
    // and so on with the other apis just feed what's expected
    // particularly the exact types
}

```

## API 

1. present_value (rate: f64, compounding_periods: f64, future_value: f64) -> f64
2. future_value (rate: f64, compounding_periods: f64, present_value: f64) -> f64
3. net_present_value (
4. payback period
5. ROI return_on_investment (earnings: f64, cf0: f64) -> f64
6. amortization
7. discount factor
8. compound_interest
9. compound_annual_growth_rate (beginning_value: f64, ending_value: f64, number_of_periods: f64) -> f64
10. leverage_ratio (total_liabilities: f64, total_debts: f64, total_income: f64) -> f64
11. rule_of_72 (rate: f64) -> f64
12. rule_of_70 (rate: f64) -> f64
12. weighted_cost_of_capital (market_value_of_equity: f64, market_value_of_debt: f64, cost_of_equity: f64, cost_of_debt: f64, tax_rate: f64) -> f64
13. PMT (rate: f64, number_of_payments: f64, principal: f64) -> f64

> Unless specified the inputs are expected to be floats. Your program will `panic` if you pass integers.

Input accordingly if calculating in (monthly, semi-annual, daily) terms. In another words, make sure that your compounding_periods (daily, monthly, annual...) is used with corresponding rates (daily, monthly, annual).

> For consistency sake `rate` is getting passed as a plain float and not as a percentage (%)

If you are wondering about compounding calculation. I got you [here](https://github.com/mohamedhayibor/rust_compound)

## Raison d'etre

Well save yourself some googling or fiddling wiki pages. All formulas have test cases. Feel free to send a PR if you feel like a missing formula should be included (preferably open an issue first :sunglasses:)

> Do not worry about the functions taking owernership of anything. As primitives support the copy trait.

> In the case an array or vector is being processed be assured that they are getting passed as reference and cloned if there is complex operations that have to be made.

Also some of the function names can be judged as quiet verbose and it is worthit. Abbreviations that you have no idea about is the last thing you would want. (There aren't many programmers with a Finance background anyways)

## Maintenance

In the forseeable future, I plan to actively manage the repos, so I will follow the salesman motto "If you open it, I will close", meaning that I will fix issues in a speedy matter.

## License

This library is distributed with the GPLv2 software license.

```
    finance (rust library - crate)
    Copyright (C) 2016 Mohamed Hayibor

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
```
