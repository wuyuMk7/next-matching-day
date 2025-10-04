use chrono::{Datelike, Days, Months, NaiveDate, Weekday};

/// Calculates the next date that falls on a specific weekday.
///
/// If the current date is already on the desired weekday, it returns the date of the same weekday in the next week.
///
/// # Arguments
///
/// * `current_date` - The starting date.
/// * `next_weekday` - The target weekday.
///
/// # Examples
///
/// ```
/// use chrono::{NaiveDate, Weekday};
/// use next_matching_day::find_next_weekday;
///
/// // Starting from a Sunday, the next Monday is the next day.
/// let current_date = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap(); // A Sunday
/// let next_monday = find_next_weekday(&current_date, &Weekday::Mon).unwrap();
/// assert_eq!(next_monday, NaiveDate::from_ymd_opt(2023, 10, 16).unwrap());
///
/// // If it's already Monday, it returns the Monday of the next week.
/// let current_date = NaiveDate::from_ymd_opt(2023, 10, 16).unwrap(); // A Monday
/// let next_monday = find_next_weekday(&current_date, &Weekday::Mon).unwrap();
/// assert_eq!(next_monday, NaiveDate::from_ymd_opt(2023, 10, 23).unwrap());
/// ```
///
/// # Returns
///
/// An `Option<NaiveDate>` which is the next date with the given weekday.
/// Returns `None` if the calculation overflows, which is highly unlikely with `NaiveDate`.
pub fn find_next_weekday(current_date: &NaiveDate, next_weekday: &Weekday) -> Option<NaiveDate> {
    let days_since = next_weekday.days_since(current_date.weekday());
    let days_distance = Days::new(((days_since + 6) % 7 + 1).into());
    current_date.checked_add_days(days_distance)
}

/// Finds the next date with a specific day of the month.
///
/// This function searches for the next occurrence of a given day of the month.
/// If the day has not yet passed in the current month, it returns the date in the current month.
/// Otherwise, it searches for the next month that has that day.
///
/// # Arguments
///
/// * `current_date` - The starting date.
/// * `next_day` - The target day of the month (1-31).
///
/// # Returns
///
/// An `Option<NaiveDate>` containing the next matching date. Returns `None` if the
/// day is invalid (e.g., greater than 31) or if a valid date cannot be found
/// within a reasonable number of future months (currently 12).
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use next_matching_day::find_next_day_of_month;
///
/// // Find the next 20th from October 15th -> October 20th
/// let current_date = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap();
/// let next_20th = find_next_day_of_month(&current_date, 20).unwrap();
/// assert_eq!(next_20th, NaiveDate::from_ymd_opt(2023, 10, 20).unwrap());
///
/// // Find the next 31st from January 31st -> March 31st (skipping February)
/// let current_date = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
/// let next_31st = find_next_day_of_month(&current_date, 31).unwrap();
/// assert_eq!(next_31st, NaiveDate::from_ymd_opt(2023, 3, 31).unwrap());
/// ```
pub fn find_next_day_of_month(current_date: &NaiveDate, next_day: u32) -> Option<NaiveDate> {
    // If the day is in the future for the current month, use it.
    if current_date.day() < next_day {
        if let Some(date) = current_date.with_day(next_day) {
            return Some(date);
        }
    }

    // Otherwise, check subsequent months.
    for i in 1..=12 {
        if let Some(date) = current_date
            .checked_add_months(Months::new(i))
            .and_then(|d| d.with_day(next_day))
        {
            return Some(date);
        }
    }

    None
}

/// Calculates the next occurrence of a specific month and day after a given date.
///
/// This function finds the next date that matches the provided `next_month` and `next_day`.
/// It first checks if the target date is in the future of the current year. If not, it
/// searches for the first valid occurrence in the subsequent years.
///
/// This approach correctly handles cases like leap years when searching for February 29.
///
/// # Arguments
///
/// * `current_date` - The starting date.
/// * `next_month` - The target month (1-12).
/// * `next_day` - The target day (1-31).
///
/// # Returns
///
/// An `Option<NaiveDate>` containing the next matching date. Returns `None` if a valid
/// date cannot be found within a reasonable number of future years (currently 8).
///
/// ```
/// use chrono::NaiveDate;
/// use next_matching_day::find_next_annual_date;
///
/// // Target date is in the same year and after the current date
/// let date = NaiveDate::from_ymd_opt(2023, 5, 15).unwrap();
/// let result = find_next_annual_date(&date, 6, 20).unwrap();
/// assert_eq!(result, NaiveDate::from_ymd_opt(2023, 6, 20).unwrap());
///
/// // Target date has already passed, so it finds the date in the next year.
/// let date = NaiveDate::from_ymd_opt(2023, 8, 1).unwrap();
/// let result = find_next_annual_date(&date, 7, 1).unwrap();
/// assert_eq!(result, NaiveDate::from_ymd_opt(2024, 7, 1).unwrap());
/// ```
pub fn find_next_annual_date(
    current_date: &NaiveDate,
    next_month: u32,
    next_day: u32,
) -> Option<NaiveDate> {
    let cur_year = current_date.year();

    // Try the date with the current year and see if it's applicable.
    let next_year = cur_year;
    let next_date = NaiveDate::from_ymd_opt(next_year, next_month, next_day);
    if let Some(next_date) = next_date {
        if next_date.gt(current_date) {
            return Some(next_date);
        }
    }

    // Loop through the next few years to find a valid date.
    // This handles regular dates and leap years (for Feb 29) gracefully.
    // We check up to 8 years ahead, which is sufficient to find the next leap year.
    for i in 1..=8 {
        if let Some(date) = NaiveDate::from_ymd_opt(cur_year + i, next_month, next_day) {
            return Some(date);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Weekday};

    #[test]
    fn test_find_next_weekday() {
        // Test case 1: Next weekday is the next day
        let date = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap(); // Sunday
        let result = find_next_weekday(&date, &Weekday::Mon).unwrap(); // Next Monday
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 10, 16).unwrap());

        // Test case 2: Next weekday is in the next week
        let date = NaiveDate::from_ymd_opt(2023, 10, 16).unwrap(); // Monday
        let result = find_next_weekday(&date, &Weekday::Sun).unwrap(); // Next Sunday
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 10, 22).unwrap());

        // Test case 3: Next weekday is the same day
        let date = NaiveDate::from_ymd_opt(2023, 10, 16).unwrap(); // Monday
        let result = find_next_weekday(&date, &Weekday::Mon).unwrap(); // Next Monday
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 10, 23).unwrap());

        // Test case 4: Next weekday is in the next month
        let date = NaiveDate::from_ymd_opt(2023, 10, 30).unwrap(); // Monday
        let result = find_next_weekday(&date, &Weekday::Sun).unwrap(); // Next Sunday
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 11, 5).unwrap());

        // Test case 5: Next weekday is in the next year
        let date = NaiveDate::from_ymd_opt(2023, 12, 28).unwrap(); // Thursday
        let result = find_next_weekday(&date, &Weekday::Wed).unwrap(); // Next Wednesday
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 1, 3).unwrap());

        // Test case 6: Leap year, before Feb 29
        let date = NaiveDate::from_ymd_opt(2024, 2, 26).unwrap(); // Monday
        let result = find_next_weekday(&date, &Weekday::Wed).unwrap(); // Next Wednesday
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 2, 28).unwrap());

        // Test case 7: Leap year, on Feb 28
        let date = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap(); // Wednesday
        let result = find_next_weekday(&date, &Weekday::Thu).unwrap(); // Next Thursday
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());

        // Test case 8: Leap year, on Feb 29
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(); // Thursday
        let result = find_next_weekday(&date, &Weekday::Fri).unwrap(); // Next Friday
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 3, 1).unwrap());
    }

    #[test]
    fn test_find_next_day_of_month() {
        // Test case 1: Next day is in the same month
        let date = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap();
        let result = find_next_day_of_month(&date, 20).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 10, 20).unwrap());

        // Test case 2: Next day is in the next month
        let date = NaiveDate::from_ymd_opt(2023, 10, 25).unwrap();
        let result = find_next_day_of_month(&date, 10).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 11, 10).unwrap());

        // Test case 3: Current day is the same as next_day, should find it in the next month
        let date = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap();
        let result = find_next_day_of_month(&date, 15).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 11, 15).unwrap());

        // Test case 4: Next day is 31, current month is short (September)
        let date = NaiveDate::from_ymd_opt(2023, 9, 15).unwrap();
        let result = find_next_day_of_month(&date, 31).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 10, 31).unwrap());

        // Test case 5: Next day is 31, current month is January, next month is February (short)
        let date = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
        let result = find_next_day_of_month(&date, 31).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 3, 31).unwrap());

        // Test case 6: Leap year, looking for the 29th from Feb 1st.
        let date = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let result = find_next_day_of_month(&date, 29).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());

        // Test case 7: Non-leap year, looking for the 29th from Feb 1st.
        let date = NaiveDate::from_ymd_opt(2023, 2, 1).unwrap();
        let result = find_next_day_of_month(&date, 29).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 3, 29).unwrap());
    }

    #[test]
    fn test_find_next_annual_date() {
        // Test case 1: Target date is in the same year and after the current date
        let date = NaiveDate::from_ymd_opt(2023, 5, 15).unwrap();
        let result = find_next_annual_date(&date, 6, 20).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 6, 20).unwrap());

        // Test case 2: Target date is in the same month and day as the current date, so it returns the next year's date
        let date = NaiveDate::from_ymd_opt(2023, 5, 15).unwrap();
        let result = find_next_annual_date(&date, 5, 15).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 5, 15).unwrap());

        // Test case 3: Target date is in the same month but earlier in the month, so it returns the next year's date
        let date = NaiveDate::from_ymd_opt(2023, 5, 15).unwrap();
        let result = find_next_annual_date(&date, 5, 13).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 5, 13).unwrap());

        // Test case 4: Target date is February 29, and the current date is after February 29 in a leap year; returns the next leap year's date
        let date = NaiveDate::from_ymd_opt(2024, 3, 20).unwrap();
        let result = find_next_annual_date(&date, 2, 29).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2028, 2, 29).unwrap());

        // Test case 5: Target date is February 29, and the current date is in a non-leap year; returns the next leap year's date
        let date = NaiveDate::from_ymd_opt(2025, 2, 20).unwrap();
        let result = find_next_annual_date(&date, 2, 29).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2028, 2, 29).unwrap());
    }
}
