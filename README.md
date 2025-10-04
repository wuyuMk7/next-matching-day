# `next-matching-day`

A simple Rust library, built on `chrono`, to find the next occurrence of a date based on various criteria.

## Features

- Find the next specific weekday (e.g., the next Monday).
- Find the next specific day of the month (e.g., the next 25th).
- Find the next specific annual date (e.g., the next Christmas).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
next-matching-day = "0.1.0" # Or the latest version
chrono = "0.4"
```

## Usage

Here are some examples of how to use the functions provided by this library.

### `find_next_weekday`

Calculates the next date that falls on a specific weekday. If the current date is already on the desired weekday, it returns the date of the same weekday in the *next* week.

```rust
use chrono::{NaiveDate, Weekday};
use next_matching_day::find_next_weekday;

// Starting from a Sunday, the next Monday is the next day.
let current_date = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap(); // A Sunday
let next_monday = find_next_weekday(&current_date, &Weekday::Mon).unwrap();
assert_eq!(next_monday, NaiveDate::from_ymd_opt(2023, 10, 16).unwrap());

// If it's already Monday, it returns the Monday of the next week.
let current_date = NaiveDate::from_ymd_opt(2023, 10, 16).unwrap(); // A Monday
let next_monday = find_next_weekday(&current_date, &Weekday::Mon).unwrap();
assert_eq!(next_monday, NaiveDate::from_ymd_opt(2023, 10, 23).unwrap());
```

### `find_next_day_of_month`

Finds the next date with a specific day of the month. If the day has not yet passed in the current month, it returns the date in the current month. Otherwise, it searches for the next month that has that day.

```rust
use chrono::NaiveDate;
use next_matching_day::find_next_day_of_month;

// Find the next 20th from October 15th -> October 20th
let current_date = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap();
let next_20th = find_next_day_of_month(&current_date, 20).unwrap();
assert_eq!(next_20th, NaiveDate::from_ymd_opt(2023, 10, 20).unwrap());

// Find the next 31st from January 31st -> March 31st (skipping February)
let current_date = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
let next_31st = find_next_day_of_month(&current_date, 31).unwrap();
assert_eq!(next_31st, NaiveDate::from_ymd_opt(2023, 3, 31).unwrap());
```

### `find_next_annual_date`

Calculates the next occurrence of a specific month and day. If the date has already passed in the current year, it finds the date in the following year. It correctly handles leap years.

```rust
use chrono::NaiveDate;
use next_matching_day::find_next_annual_date;

// Target date is in the same year and after the current date
let date = NaiveDate::from_ymd_opt(2023, 5, 15).unwrap();
let result = find_next_annual_date(&date, 6, 20).unwrap();
assert_eq!(result, NaiveDate::from_ymd_opt(2023, 6, 20).unwrap());

// Target date has already passed, so it finds the date in the next year.
let date = NaiveDate::from_ymd_opt(2023, 8, 1).unwrap();
let result = find_next_annual_date(&date, 7, 1).unwrap();
assert_eq!(result, NaiveDate::from_ymd_opt(2024, 7, 1).unwrap());
```