Finance
-------------

This crate provides utility functions for the most common operations in Finance. Because of Rust borrowing system, all arguments need to be passed by reference as this library has a strong focus on immutability.

You will notice that most are passed by reference and then cloned before performing any calculations. This allows you to use the functions without worrying about issues (borrowing violations).

## Usage

Include this library with the following:
```
[dependencies]
finance = "0.0.0"
```

## API 

1. present_value
2. future_value
3. irr
4. payback period
5. ROI return_on_investment
6. amortization
7. discount factor
8. compound_interest
9. compound_annual_growth_rate
10. leverage_ratio
11. rule_of_72
12. weighted_cost_of_capital
13. pmt

> Unless specified the inputs are expected to be floats. Your program will `panic` ifyou pass integers.

> Also rates are annualized by default. Input accordingly if calculating in (monthly, semi-annual, daily) terms.

If you are wondering about compounding calculation. I got you [here](https://github.com/mohamedhayibor/rust_compound)

## Raison d'etre

Well save yourself some googling or fiddling wiki pages. All formulas have test cases and got battle tested before release. Feel free to send a PR if you feel like a missing formula should be included (preferably open an issue first :sunglasses:)

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
