
pub unsafe fn millis() -> i64 {
    esp_idf_svc::sys::esp_timer_get_time()/1000
}

pub unsafe fn micros() -> i64 {
    esp_idf_svc::sys::esp_timer_get_time()
}