
pub unsafe fn millis() {
    esp_idf_svc::sys::esp_timer_get_time()/1000;
}

pub unsafe fn micros() {
    esp_idf_svc::sys::esp_timer_get_time();
}