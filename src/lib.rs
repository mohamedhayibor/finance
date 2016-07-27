

pub fn present_value(rate: &f64, compounding_periods: &f64, future_value: &f64) -> f64 {
    // cloning passed in args to avoid mutability
    let r = rate.clone();
    let n = compounding_periods.clone();
    let F = future_value.clone();
    
    



}

pub fn future_value(rate: &f64, compounding_periods: &f64, present_value: &f64) -> f64 {
    // cloning passed in args to avoid mutability
    let r = rate.clone();
    let n = compounding_periods.clone();
    let P = present_value.clone();

    // 

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
