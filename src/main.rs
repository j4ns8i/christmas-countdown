use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use chrono_tz::{America::Los_Angeles, Tz};
use rocket;

#[rocket::get("/")]
fn index() -> String {
    let tz = Los_Angeles;
    let now = Utc::now().with_timezone(&tz);
    let next_christmas_date = find_next_christmas_date(&now);
    if now.date_naive() == next_christmas_date.date_naive() {
        "Merry christmas!".into()
    } else {
        format!(
            "Only {} until Christmas!",
            duration_string(next_christmas_date - now)
        )
    }
}

fn find_next_christmas_date(date: &DateTime<Tz>) -> DateTime<Tz> {
    let christmas_current_year = date
        .timezone()
        .with_ymd_and_hms(date.year(), 12, 25, 0, 0, 0)
        .unwrap();
    let delta = christmas_current_year.date_naive() - date.date_naive();
    if delta >= Duration::zero() {
        christmas_current_year
    } else {
        date.timezone()
            .with_ymd_and_hms(date.year() + 1, 12, 25, 0, 0, 0)
            .unwrap()
    }
}

fn duration_string(mut dur: Duration) -> String {
    let days = dur.num_days();
    dur = dur - Duration::days(days);
    let hours = dur.num_hours();
    dur = dur - Duration::hours(hours);
    let minutes = dur.num_minutes();
    dur = dur - Duration::minutes(minutes);
    let seconds = dur.num_seconds();
    format!(
        "{} {}, {} {}, {} {}, and {} {}",
        days,
        pluralize("day", days),
        hours,
        pluralize("hour", hours),
        minutes,
        pluralize("minute", minutes),
        seconds,
        pluralize("second", seconds)
    )
}

fn pluralize(unit: impl AsRef<str>, num: i64) -> String {
    if num == 1 {
        String::from(unit.as_ref())
    } else {
        format!("{}s", unit.as_ref())
    }
}

#[rocket::launch]
fn server() -> _ {
    rocket::build().mount("/", rocket::routes![index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_string() {
        assert_eq!(
            duration_string(Duration::days(4)),
            String::from("4 days, 0 hours, 0 minutes, and 0 seconds")
        );

        assert_eq!(
            duration_string(Duration::hours(3)),
            String::from("0 days, 3 hours, 0 minutes, and 0 seconds")
        );

        assert_eq!(
            duration_string(Duration::minutes(2)),
            String::from("0 days, 0 hours, 2 minutes, and 0 seconds")
        );

        assert_eq!(
            duration_string(Duration::seconds(1)),
            String::from("0 days, 0 hours, 0 minutes, and 1 second")
        );

        assert_eq!(
            duration_string(
                Duration::days(4)
                    + Duration::hours(3)
                    + Duration::minutes(2)
                    + Duration::seconds(1)
            ),
            String::from("4 days, 3 hours, 2 minutes, and 1 second")
        );

        assert_eq!(
            duration_string(
                Duration::days(0)
                    + Duration::hours(24)
                    + Duration::minutes(60)
                    + Duration::seconds(61)
            ),
            String::from("1 day, 1 hour, 1 minute, and 1 second")
        );
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize("day", 1), "day");
        assert_eq!(pluralize("day", 0), "days");
        assert_eq!(pluralize("day", 2), "days");
        assert_eq!(pluralize("day", 10), "days");
        assert_eq!(pluralize("month", 1), "month");
        assert_eq!(pluralize("test", 5), "tests");
    }

    #[test]
    fn test_find_next_christmas_date() {
        assert_eq!(
            find_next_christmas_date(
                &Los_Angeles
                    .with_ymd_and_hms(2022, 12, 20, 16, 30, 15)
                    .unwrap()
            ),
            Los_Angeles.with_ymd_and_hms(2022, 12, 25, 0, 0, 0).unwrap(),
        );

        assert_eq!(
            find_next_christmas_date(
                &Los_Angeles
                    .with_ymd_and_hms(2020, 12, 30, 16, 30, 15)
                    .unwrap()
            ),
            Los_Angeles.with_ymd_and_hms(2021, 12, 25, 0, 0, 0).unwrap(),
        );

        assert_eq!(
            find_next_christmas_date(
                &Los_Angeles
                    .with_ymd_and_hms(2020, 12, 25, 16, 30, 15)
                    .unwrap()
            ),
            Los_Angeles.with_ymd_and_hms(2020, 12, 25, 0, 0, 0).unwrap(),
        );
    }
}
