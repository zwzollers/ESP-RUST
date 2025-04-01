use core::time::Duration;
use esp_idf_svc::mqtt::client::*;
use anyhow::Result;

fn mqtt_create<'a, F>(
    url: &str,
    client_id: &str,
    callback: F
) -> Result<EspMqttClient <'a>> where F: for<'b> FnMut(EspMqttEvent<'b>) + Send + 'static{
    let mqtt_client = EspMqttClient::new_cb(
        url,
        &MqttClientConfiguration {
            client_id: Some(client_id),
            ..Default::default()
        },
        callback
    )?;

    Ok(mqtt_client)
}

fn mqtt_subscribe(client: &mut EspMqttClient, topics: Vec<&str>, timeout: u64) -> Result<(), EspError> {
    for topic in topics {
        while let Err(e) = client.subscribe(topic, QoS::AtMostOnce) {
            std::thread::sleep(Duration::from_millis(timeout));
    
            continue;
        }
    }
    Ok(())
}
