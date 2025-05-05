# Homeassistant-rs

 Implements the Homeassistant API for use in rust

 A simple lib, that queries different endpoints and returns the data in a usable format.

 The first 2 arguments of each function are always: `HA_URL`, `API_Token`.

 These arguments do not have to be filled with actual data, they can be `None`, but in this case you will need to use environment variables.

 Under the hood we use dotenvy.

 Example env:
 ```text
 HA_URL="http://localhost:8123"
 HA_TOKEN="api_token_from_hass"
 ```

 - Easily get HA's config:
 ```rust
 use homeassistant_rs::{self, hass};
 let config = hass().config(None, None).await?;

 println!("{}", config.version);
 ```

 You can check all available endpoints here: [`HomeAssistant`]

 - More Examples:


 ```rust
 hass().config(None, None).await?;
 hass().events(None, None).await?;
 hass().services(None, None).await?;
 hass()
     .history(
         Some("http://localhost:8123"),
         Some("API_Token"),
         Some("entity_id"),
         /// minimal_response
         true,
         /// no_attributes
         true,
         /// significant_changes_only
         true,
     )
     .await?;
 hass().logbook(None, None, Some("light.bedroom_local_bedroom_local")).await?;
 hass().states(None, None, Some("light.bedroom_local_bedroom_local")).await?;
 hass().states(None, None, None).await?;
 hass().error_log(None, None).await?;
 ```