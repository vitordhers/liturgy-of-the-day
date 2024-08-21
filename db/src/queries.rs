use crate::tables_names::{DATES_TABLE, GOSPELS_TABLE};

pub fn get_liturgical_date_by_date_and_lang(
    date: &str,
    locale: &str,
    calendar: &str,
) -> String {
    format!(
        r#"
    SELECT id, day, month, time, year,
    (SELECT id, category, color,
        (SELECT id,
            (SELECT * FROM $parent.locale_video_assets_ids_map['{}']) as video_assets
            FROM ONLY type::thing('{}', $parent.gospel_uuid)
        ) AS gospel,
        locale_title_map['{}'] as title
        FROM $parent.calendar_commemorations_map['{}'] || $parent.calendar_commemorations_map.roman
    ) as commemorations 
    FROM type::thing('{}', '{}');"#,
        locale, GOSPELS_TABLE, locale, calendar, DATES_TABLE, date
    )
}
