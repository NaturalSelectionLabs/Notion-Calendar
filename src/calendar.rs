use chrono::{Duration, NaiveDate};
use icalendar::{Calendar, Class, Component, Event, EventLike};

use crate::database::DatabaseEntry;

pub fn from_results(results: Vec<DatabaseEntry>) -> Result<Calendar, Box<dyn std::error::Error>> {
    let mut cal = Calendar::new();

    cal.name("Editorial Calendar");

    for r in &results {
        let summary = match r.properties.name.title.get(0) {
            Some(title) => match &title.text {
                Some(text) => &text.content,
                None => "",
            },
            None => "",
        };

        let description = match &r.properties.content.rich_text.get(0) {
            Some(rt) => &rt.text.content,
            None => "",
        };

        let publish_date = r
            .properties
            .publish_date
            .date
            .clone()
            .and_then(|date| NaiveDate::parse_from_str(&date.start, "%Y-%m-%d").ok());

        if let Some(publish_date) = publish_date {
            cal.push(
                Event::new()
                    .summary(&summary)
                    .description(&description)
                    .starts(publish_date)
                    .class(Class::Public)
                    .ends(publish_date + Duration::days(1))
                    .done(),
            );
        }

        // let datetime_utc =
        //     DateTime::<Utc>::from_utc(publish_date.and_hms_opt(0, 0, 0).unwrap(), Utc);
    }

    Ok(cal)
}
