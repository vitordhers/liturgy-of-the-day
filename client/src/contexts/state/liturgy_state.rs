use agnus_daily_error::AgnusDailyError;
use enums::Timezone;
use leptos::{logging::log, *};

use chrono::Utc;
use structs::serialized::{Commemoration, LiturgicalDate};

use crate::contexts::config::LocaleConfigCtx;

#[cfg(not(feature = "csr"))]
#[server(GetTzLocaleLiturgicalDate, "/api", "Url", "now")]
pub async fn get_current_liturgical_date(
    timezone: Timezone,
    locale: String,
    calendar: String,
) -> Result<LiturgicalDate, ServerFnError> {
    use chrono::{DateTime, Datelike, Duration, FixedOffset, Timelike};
    use db::{db, queries::get_liturgical_date_by_date_and_lang};
    use structs::entities::LiturgicalDateEntity;

    let db = db().await;

    let current_utc_datetime = Utc::now();
    let today_datetime: DateTime<FixedOffset> =
        current_utc_datetime.with_timezone(&timezone.into());
    let current_hour = today_datetime.hour();
    let is_within_vespers_period = current_hour >= 17 && current_hour <= 23;
    let today_liturgical_date_uuid = format!(
        "{}-{}-{}",
        today_datetime.day(),
        today_datetime.month(),
        today_datetime.year()
    );
    // TODO delete this on prod
    let today_liturgical_date_uuid = String::from("5-7-2024");
    let mut query = get_liturgical_date_by_date_and_lang(
        &today_liturgical_date_uuid,
        &locale,
        &calendar,
    );
    // store tomorrow_datetime in case of vespers
    let mut tomorrow_datetime = None;
    // check for vespers
    if is_within_vespers_period {
        let tomorrow_dt = today_datetime + Duration::days(1);
        tomorrow_datetime = Some(tomorrow_dt);
        // if within vespers time, adds next day to query
        let tomorrow_liturgical_date_uuid = format!(
            "{}-{}-{}",
            tomorrow_dt.day(),
            tomorrow_dt.month(),
            tomorrow_dt.year()
        );
        query += get_liturgical_date_by_date_and_lang(
            &tomorrow_liturgical_date_uuid,
            &locale,
            &calendar,
        )
        .as_str();
    }

    // println!("query {}", query);

    let result = db.query_vec::<LiturgicalDateEntity>(&query).await;

    if result.is_err() {
        log!("server query err {:?}", result.err());
        return Err(ServerFnError::new("error 500"));
    }

    let result = result.unwrap();

    // println!("QUERY RESULT {:?}", result);

    let today_date = result
        .get(0)
        .expect(format!("Expect liturgical date {today_liturgical_date_uuid} at index 0 to exist").as_str());

    let mut proper_date = today_date.clone();

    // if within vesperas period
    if is_within_vespers_period && result.len() > 1 {
        // get tomorrow date
        let tomorrow_date = result
            .get(1)
            .expect("expect liturgical date at index 1 to exist");

        // and its precedent commemoration
        let tomorrow_precedent_commemoration = tomorrow_date.get_precedent_commemoration();

        // then check if it has I vespers
        if tomorrow_precedent_commemoration.category.has_i_vespers(
            tomorrow_datetime.expect("expect tomorrow datetime be defined"),
            tomorrow_date.time,
        ) {
            // if so, check the precedence between today and tomorrow vespers by
            // getting today's precedent commemoration
            let today_precedent_commemoration = today_date.get_precedent_commemoration();
            // and checking whether tomorrow's liturgical category has precedence over today's
            if tomorrow_precedent_commemoration.category > today_precedent_commemoration.category {
                proper_date = tomorrow_date.clone();
            }
        }
    }

    let result: LiturgicalDate = proper_date.into();
    // log!("data: {:?}", result);
    Ok(result)
}

pub fn create_liturgy_state_ctx() -> LiturgyStateCtx {
    // let (current_timestamp, set_current_timestamp) = create_signal(Utc::now());
    let locale_ctx = expect_context::<LocaleConfigCtx>();

    // let tz_timestamp = create_memo(move |_| {
    //     let ts = current_timestamp.get().timestamp();
    //     let tz = locale_config.get().timezone;
    //     (ts, tz)
    // });

    let current_liturgical_date: Resource<
        (Timezone, String, String),
        Result<LiturgicalDate, AgnusDailyError>,
    > = create_resource(
        move || locale_ctx.liturgy_fetch_data.get(),
        |(timezone, locale, calendar)| async move {
            // log!("fetch data {:?}", data);
            get_current_liturgical_date(timezone, locale, calendar)
                .await
                .map_err(|err| AgnusDailyError {
                    title: String::from("Fetch current liturgical date Error"),
                    description: err.to_string(),
                })
        },
    );

    let (current_commemoration_index, set_current_commemoration_index): (
        ReadSignal<Option<usize>>,
        WriteSignal<Option<usize>>,
    ) = create_signal(None);

    let commemorations = create_memo(move |_| {
        let loaded_date = current_liturgical_date.get();
        if loaded_date.is_none() {
            return vec![];
        }
        let loaded_date = loaded_date.expect("loaded date to be Some");
        if loaded_date.is_err() {
            return vec![];
        }

        let loaded_date = loaded_date.expect("loaded date to be Some");
        loaded_date.commemorations
    });

    // create_effect(move |_| {
    //     let loaded_date = current_liturgical_date.get();
    //     if loaded_date.is_none() {
    //         return;
    //     }
    //     let loaded_date = loaded_date.expect("loaded date to be Some");
    //     if loaded_date.is_err() {
    //         return;
    //     }
    //     let liturgical_date = loaded_date.expect("expect loaded date to be Ok");

    //     let precedent_commemoration = liturgical_date.get_precedent_commemoration();

    //     set_current_commemoration.set(Some(precedent_commemoration.clone()));
    // });

    LiturgyStateCtx {
        current_liturgical_date,
        commemorations,
        current_commemoration_index,
    }
}

#[derive(Clone)]
pub struct LiturgyStateCtx {
    pub current_liturgical_date:
        Resource<(Timezone, String, String), Result<LiturgicalDate, AgnusDailyError>>,
    pub commemorations: Memo<Vec<Commemoration>>,
    pub current_commemoration_index: ReadSignal<Option<usize>>, // pub set_current_commemoration: WriteSignal<Option<Commemoration>>,
}
