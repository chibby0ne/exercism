fn is_evenly_divisible(year: i32, number: i32) -> bool {
    return year % number == 0;
}

pub fn is_leap_year(year: i32) -> bool {
    if is_evenly_divisible(year, 400)
        || is_evenly_divisible(year, 4) && !is_evenly_divisible(year, 100)
    {
        return true;
    } else {
        return false;
    }
}
