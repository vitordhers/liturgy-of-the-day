use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use enums::{LiturgicalColor, Month};

use crate::functions::{calculate_easter_sunday, next_weekday_after, previous_weekday_before};

struct MovingLiturgyDay {
    id: MovingLiturgyDayId,
    day: Option<u32>,
    month: Option<Month>,
    year: i32,
    reference_day: ReferenceDay,
    name: &'static str,
    color: LiturgicalColor,
}

enum LiturgicalDateError {
    DateConversionError(DateConversionError),
    CalculateDayAndMonthError(CalculateDayAndMonthError),
}

#[derive(Debug)]
enum DateConversionError {
    InvalidDate,
    MissingDate,
}

enum CalculateDayAndMonthError {
    NotFoundDay,
    NotFoundMonth,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum MovingPeriod {
    SameDay,
    DaysBefore(u32),
    DaysAfter(u32),
    PastWeekday(Weekday),
    NextWeekday(Weekday),
}

#[derive(Clone, Debug, PartialEq)]
enum MovingLiturgyDayId {
    HolyEaster,
    GoodFriday,
    PalmSunday,
    Pentecost,
    SolemnityOfTheMostHolyTrinity,
    AshWednesday,
}

#[derive(Clone, Debug, PartialEq)]
struct ReferenceDay {
    id: MovingLiturgyDayId,
    period: MovingPeriod,
}

impl ReferenceDay {
    fn new(id: MovingLiturgyDayId, period: MovingPeriod) -> Self {
        Self { id, period }
    }
}

pub const LITURGY_MOVING_DAYS: &'static [MovingLiturgyDay] = &[
    MovingLiturgyDay::new_easter_sunday(),
    MovingLiturgyDay::new(
        MovingLiturgyDayId::GoodFriday,
        "Holy Friday", // The rescue of the humankind
        LiturgicalColor::Red,
        ReferenceDay::new(MovingLiturgyDayId::HolyEaster, MovingPeriod::DaysBefore(2)),
    ),
    MovingLiturgyDay::new(
        MovingLiturgyDayId::Pentecost,
        "Pentecost",
        LiturgicalColor::Red,
        ReferenceDay::new(MovingLiturgyDayId::HolyEaster, MovingPeriod::DaysAfter(50)),
    ),
    MovingLiturgyDay::new(
        MovingLiturgyDayId::PalmSunday,
        "Palm Sunday",
        LiturgicalColor::Red,
        ReferenceDay::new(MovingLiturgyDayId::HolyEaster, MovingPeriod::DaysBefore(7)),
    ),
    MovingLiturgyDay::new(
        MovingLiturgyDayId::AshWednesday,
        "Ash Wednestday",
        LiturgicalColor::Purple,
        ReferenceDay::new(MovingLiturgyDayId::HolyEaster, MovingPeriod::DaysBefore(40)),
    ),
    MovingLiturgyDay::new(
        MovingLiturgyDayId::SolemnityOfTheMostHolyTrinity,
        "Solemnity of the Most Holy Trinity", // All glory and honour is yours, for ever and ever
        LiturgicalColor::Gold,
        ReferenceDay::new(
            MovingLiturgyDayId::Pentecost,
            MovingPeriod::NextWeekday(Weekday::Sun),
        ),
    ),
];

impl MovingLiturgyDay {
    fn new(
        id: MovingLiturgyDayId,
        name: &'static str,
        color: LiturgicalColor,
        reference_day: ReferenceDay,
    ) -> Self {
        let current_date_time = Local::now();
        let current_year = current_date_time.year();

        Self {
            id,
            day: None,
            month: None,
            year: current_year,
            reference_day,
            name,
            color,
        }
    }

    fn new_easter_sunday() -> Self {
        let current_date_time = Local::now();
        let current_year = current_date_time.year();
        let easter_sunday = calculate_easter_sunday(current_year);

        Self {
            id: MovingLiturgyDayId::HolyEaster,
            day: Some(easter_sunday.day()),
            month: Some(Month::try_from(easter_sunday.month()).expect("month to be valid")),
            year: current_year,
            reference_day: ReferenceDay::new(MovingLiturgyDayId::HolyEaster, MovingPeriod::SameDay),
            name: "Feast of the Resurrection of the Lord",
            color: LiturgicalColor::Gold,
        }
    }

    fn add_to_vec_with_day_and_month(
        &self,
        mut moving_liturgy_days: Vec<MovingLiturgyDay>,
    ) -> Result<(), CalculateDayAndMonthError> {

        // check this
        if self.reference_day.period == MovingPeriod::SameDay
            || (self.day.is_some() && self.month.is_some())
        {
            return Ok(());
        }

        let reference_day = moving_liturgy_days
            .iter()
            .find(|&&day| day.id == self.reference_day.id);

        if reference_day.is_none() {
            return Err(CalculateDayAndMonthError::NotFoundDay);
        }

        let reference_day = reference_day.expect("day to exist");
        let reference_day_date: NaiveDate = reference_day
            .try_into()
            .expect("reference day to be naive date valid");

        let date = match self.reference_day.period {
            MovingPeriod::DaysBefore(days) => reference_day_date - Duration::days(days as i64),
            MovingPeriod::DaysAfter(days) => reference_day_date + Duration::days(days as i64),
            MovingPeriod::PastWeekday(weekday) => next_weekday_after(reference_day_date, weekday),
            MovingPeriod::NextWeekday(weekday) => {
                previous_weekday_before(reference_day_date, weekday)
            }
            MovingPeriod::SameDay => unreachable!(),
        };

        let mut moving_liturgy_day = *self.clone();

        moving_liturgy_day.day = Some(date.day());
        moving_liturgy_day.month = Some(
            date.month()
                .try_into()
                .expect("month as u32 to be convertable to Month enum"),
        );

        moving_liturgy_days.push(moving_liturgy_day);

        return Ok(());
    }
}

impl TryInto<NaiveDate> for MovingLiturgyDay {
    type Error = DateConversionError;

    fn try_into(self) -> Result<NaiveDate, Self::Error> {
        match (self.year, self.month, self.day) {
            (year, Some(month), Some(day)) => {
                if day < 1 {
                    return Err(DateConversionError::InvalidDate);
                }

                match month {
                    Month::February => {
                        let last_day_of_feb: u32 =
                            if NaiveDate::from_ymd_opt(year, 1, 1).unwrap().leap_year() {
                                29
                            } else {
                                28
                            };

                        if day > last_day_of_feb {
                            return Err(DateConversionError::InvalidDate);
                        }
                    }
                    Month::January
                    | Month::March
                    | Month::May
                    | Month::July
                    | Month::August
                    | Month::October
                    | Month::October => {
                        if day > 31 {
                            return Err(DateConversionError::InvalidDate);
                        }
                    }
                    _ => {
                        if day > 30 {
                            return Err(DateConversionError::InvalidDate);
                        }
                    }
                }

                let date =
                    NaiveDate::from_ymd_opt(year, month.into(), day).expect("date to be valid");

                Ok(date)
            }
            _ => Err(DateConversionError::MissingDate),
        }
    }
}

impl TryInto<NaiveDate> for &MovingLiturgyDay {
    type Error = DateConversionError;

    fn try_into(self) -> Result<NaiveDate, Self::Error> {
        (*self).try_into()
    }
}

/*
//// Red
/// Pentecost
/// Feasts of Passion of the Lord
// Sexagesima Sunday is the second Sunday before Ash Wednesday and the eighth Sunday before Easter Sunday
// Septuagesima Sunday: Septuagesima Sunday is the third Sunday before Ash Wednesday and the ninth Sunday before Easter Sunday

// The Prayer of Our Lord Jesus Christ (in the Garden of Gethsemane)—on the Tuesday after Septuagesima Sunday.
// The Commemoration of the Passion of Our Lord Jesus Christ (Votive Mass of the Passion)— on the Tuesday after Sexagesima Sunday.
// The Sacred Crown of Thorns of Our Lord Jesus Christ—on the Friday after Ash Wednesday.
// The Sacred Lance and Nails of Our Lord Jesus Christ—Ember Friday in Lent (1st Week of Lent).
// The Most Sacred Shroud of Our Lord Jesus Christ—on Friday of the Second Week in Lent.
// The Five Sacred Wounds of Our Lord Jesus Christ—on Friday of the Third Week in Lent.
// The Most Precious Blood of Our Lord Jesus Christ—on Friday of the Fourth Week in Lent.
/// Good Friday
/// Palm Sunday

//// White / Gold
// The Baptism of the Lord is celebrated on the Sunday following the Feast of the Epiphany - Feast

// The Saturday following the Feast of the Epiphany, which is often referred to as the First Saturday after Epiphany. 
/ This day is sometimes celebrated with special Marian devotions, such as the recitation of the Rosary or the observance of Mass in honor of the Blessed Virgin Mary. - optional memorial

// Monday after Pentecost: The Blessed Virgin Mary, Mother of the Church
// 1 day after the Solemnity of the Most Sacred Heart of Jesus: The Immaculate Heart of the Blessed Virgin Mary
/// Solemnity of the Most Holy Trinity - Trinity Sunday is the first Sunday after Pentecost
*/

/// Rose
// 4th Sunday of Lent (Laetare) 
