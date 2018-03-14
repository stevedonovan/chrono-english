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


