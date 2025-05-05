
use super::*;

#[cfg(test)]
#[tokio::test]
async fn main() -> anyhow::Result<()> {
    // allow unused, since it'll cry otherwise, when testing in release mode
    #[allow(unused_imports)]
    use protokoll::log;
    use serde_json::json;

    protokoll::debug!("testing config");
    hass().config(None, None).await?;
    protokoll::debug!("finished testing config");
    protokoll::debug!("testing events");
    hass().events(None, None).await?;
    protokoll::debug!("finished testing events");
    protokoll::debug!("testing services");
    hass().services(None, None).await?;
    protokoll::debug!("finished testing services");
    protokoll::debug!("testing history");
    hass()
        .history(
            None,
            None,
            Some("light.bedroom_local_bedroom_local"),
            true,
            true,
            true,
        )
        .await?;
    protokoll::debug!("finished testing history");
    protokoll::debug!("testing logbook");
    hass()
        .logbook(None, None, Some("light.bedroom_local_bedroom_local"))
        .await?;
    protokoll::debug!("finished testing logbook");
    protokoll::debug!("testing states");
    protokoll::debug!("testing single entity");
    hass()
        .states(None, None, Some("light.bedroom_local_bedroom_local"))
        .await?;
    protokoll::debug!("testing multiple entities");
    hass().states(None, None, None).await?;
    protokoll::debug!("finished testing states");
    protokoll::debug!("testing error log");
    hass().error_log(None, None).await?;
    protokoll::debug!("finished testing error log");
    protokoll::debug!("testing camera_proxy");
    protokoll::debug!("unable to test camera_proxy, as i (Blexyel) do not have this set up");
    //hass().camera_proxy(None, None, "", 1).await?;
    protokoll::debug!("finished testing camera_proxy");
    protokoll::debug!("testing calendars");
    protokoll::debug!("unable to test calendars, see function");
    //hass().calendars(None, None).await?;
    protokoll::debug!("finished testing calendars");
    protokoll::debug!("testing state post request");
    hass()
        .request()
        .state(
            None,
            None,
            "sensor.i_am_testing",
            structs::StatesRequest {
                state: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs()
                    .to_string(),
                attributes: Some(json!({"friendly_name": "homeassistant-rs testing"})),
            },
        )
        .await?;
    protokoll::debug!("finished testing state post request");
    protokoll::debug!("testing events post request");
    hass()
        .request()
        .events(None, None, "tag_scanned", json!({}))
        .await?;
    protokoll::debug!("finished testing events post request");
    protokoll::debug!("testing service post request");
    hass()
        .request()
        .service(
            None,
            None,
            "light",
            "turn_on",
            json!({"entity_id": "light.bedroom_local_bedroom_local", "rgb_color": [0,0,255]}),
            false,
        )
        .await?;
    protokoll::debug!("finished testing service post request");
    protokoll::debug!("testing template post request");
    hass()
        .request()
        .template(
            None,
            None,
            structs::TemplateRequest {
                template: "It is right now {{ now() }}".to_string(),
            },
        )
        .await?;
    protokoll::debug!("finished testing template post request");
    protokoll::debug!("testing config check post request");
    hass().request().config_check(None, None).await?;
    protokoll::debug!("finished testing config check post request");
    protokoll::debug!("testing Intent post request");
    hass().request().intent(None, None, json!({})).await?;
    protokoll::debug!("finished testing Intent post request");
    Ok(())
}
