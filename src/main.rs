use chrono::{Datelike, Duration};
use chrono_tz;
use rocket;

#[rocket::get("/")]
fn index() -> String {
    let tz = chrono_tz::America::Los_Angeles;
    let now = chrono::Utc::now().with_timezone(&tz);
    let christmas_date = chrono::NaiveDate::from_ymd_opt(now.year(), 12, 25).unwrap();
    if now.date_naive() == christmas_date {
        "Merry christmas!".into()
    } else {
        "TODO".into()
    }
}

fn duration_string(mut dur: chrono::Duration) -> String {
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
}
