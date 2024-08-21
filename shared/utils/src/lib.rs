use agnus_daily_error::AgnusDailyError;
use std::time::Duration;
use wasm_bindgen::JsCast;

#[cfg(not(feature = "ssr"))]
pub fn is_webkit() -> bool {
    use leptos::window;
    match window().navigator().user_agent() {
        Ok(user_agent_string) => user_agent_string.contains("WebKit"),
        Err(_) => false,
    }
}

#[cfg(feature = "ssr")]
pub fn is_webkit() -> bool {
    false
}

// pub fn get_element_by_id<T>(id: &str) -> Result<T, AgnusDailyError>
// where
//     T: JsCast,
// {
//     use leptos::document;
//     let element = document()
//         .get_element_by_id(id)
//         .ok_or(AgnusDailyError::element_not_found(id))?
//         .dyn_into::<T>()
//         .ok()
//         .ok_or(AgnusDailyError::dyn_into_fail(id))?;
//     Ok(element)
// }

pub fn unix_to_hours_secs_mins(secs: f64) -> String {
    let duration = Duration::from_secs_f64(secs);
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;

    match hours > 0 {
        true => format!("{:0>1}:{:0>2}:{:0>2}", hours, minutes, seconds),
        false => format!("{:0>1}:{:0>2}", minutes, seconds),
    }
}
