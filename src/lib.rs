extern crate scanlex;
extern crate time;
extern crate chrono;
use chrono::prelude::*;

mod parser;
mod errors;
mod types;
use types::*;
use errors::*;

#[derive(Clone,Copy)]
pub enum Dialect {
    Uk,
    Us
}

pub fn parse_date_string<Tz: TimeZone>(s: &str, now: DateTime<Tz>, dialect: Dialect) -> DateResult<DateTime<Tz>>
where Tz::Offset: Copy {
    let mut dp = parser::DateParser::new(s);
    if let Dialect::Us = dialect {
        dp = dp.american_date();
    }
    let d = dp.parse()?;
    //println!("parsed {:?}",d);

    // we may have explicit hour:minute:sec
    let tspec = match d.time {
        Some(tspec) => tspec,
        None => TimeSpec::new(0,0,0)
    };

    let date_time = if let Some(dspec) = d.date {
        dspec.to_date_time(now,tspec).or_err("bad date")?
    } else { // no date, time set for today's date
        tspec.to_date_time(now.date()).or_err("bad time")?
    };
    Ok(date_time)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FMT_ISO: &str = "%+";

    fn display(t: DateResult<DateTime<Utc>>) -> String {
        t.unwrap().format(FMT_ISO).to_string()
    }

    #[test]
    fn basics() {
        let base = parse_date_string("2018-03-21 11:00",Utc::now(),Dialect::Uk).unwrap();

        // Day of week - relative to today. May have a time part
        assert_eq!(display(parse_date_string("friday",base,Dialect::Uk)),"2018-03-23T00:00:00+00:00");
        assert_eq!(display(parse_date_string("friday 10:30",base,Dialect::Uk)),"2018-03-23T10:30:00+00:00");
        assert_eq!(display(parse_date_string("friday 8pm",base,Dialect::Uk)),"2018-03-23T20:00:00+00:00");
        assert_eq!(display(parse_date_string("next mon",base,Dialect::Uk)),"2018-03-26T00:00:00+00:00");
        assert_eq!(display(parse_date_string("last fri 9.30",base,Dialect::Uk)),"2018-03-16T09:30:00+00:00");

        // date expressed as month, day - relative to today
        assert_eq!(display(parse_date_string("9/11",base,Dialect::Us)),"2018-09-11T00:00:00+00:00");
        assert_eq!(display(parse_date_string("last 9/11",base,Dialect::Us)),"2017-09-11T00:00:00+00:00");
        assert_eq!(display(parse_date_string("April 1 8.30pm",base,Dialect::Uk)),"2018-04-01T20:30:00+00:00");

        // advance by time unit from today
        assert_eq!(display(parse_date_string("2d",base,Dialect::Uk)),"2018-03-23T11:00:00+00:00");
        assert_eq!(display(parse_date_string("3 weeks",base,Dialect::Uk)),"2018-04-11T11:00:00+00:00");
        assert_eq!(display(parse_date_string("3h",base,Dialect::Uk)),"2018-03-21T14:00:00+00:00");
        assert_eq!(display(parse_date_string("6 months",base,Dialect::Uk)),"2018-09-21T00:00:00+00:00");
        assert_eq!(display(parse_date_string("6 months ago",base,Dialect::Uk)),"2017-09-21T00:00:00+00:00");
        assert_eq!(display(parse_date_string("3 hours ago",base,Dialect::Uk)),"2018-03-21T08:00:00+00:00");
        assert_eq!(display(parse_date_string(" -3h",base,Dialect::Uk)),"2018-03-21T08:00:00+00:00");
        assert_eq!(display(parse_date_string(" -3 month",base,Dialect::Uk)),"2017-12-21T00:00:00+00:00");

        // absolute date with year, month, day - formal ISO and informal UK or US
        assert_eq!(display(parse_date_string("2017-06-30",base,Dialect::Uk)),"2017-06-30T00:00:00+00:00");
        assert_eq!(display(parse_date_string("30/06/17",base,Dialect::Uk)),"2017-06-30T00:00:00+00:00");
        assert_eq!(display(parse_date_string("06/30/17",base,Dialect::Us)),"2017-06-30T00:00:00+00:00");

        // may be followed by time part, formal and informal
        assert_eq!(display(parse_date_string("2017-06-30 08:20:30",base,Dialect::Uk)),"2017-06-30T08:20:30+00:00");
        assert_eq!(display(parse_date_string("2017-06-30 8.20",base,Dialect::Uk)),"2017-06-30T08:20:00+00:00");
        assert_eq!(display(parse_date_string("2017-06-30 8.30pm",base,Dialect::Uk)),"2017-06-30T20:30:00+00:00");
        assert_eq!(display(parse_date_string("2017-06-30 2am",base,Dialect::Uk)),"2017-06-30T02:00:00+00:00");
        assert_eq!(display(parse_date_string("30 June 2018",base,Dialect::Uk)),"2018-06-30T00:00:00+00:00");
        assert_eq!(display(parse_date_string("June 30, 2018",base,Dialect::Uk)),"2018-06-30T00:00:00+00:00");
        assert_eq!(display(parse_date_string("June   30,    2018",base,Dialect::Uk)),"2018-06-30T00:00:00+00:00");


    }

}
