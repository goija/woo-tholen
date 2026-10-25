use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.pdok.nl/kadaster/brk-kadastrale-kaart/ogc/v1/collections/perceel/items?bbox=66000%2C396000%2C67000%2C397000&bbox-crs=http%3A%2F%2Fwww.opengis.net%2Fdef%2Fcrs%2FEPSG%2F0%2F28992&crs=http%3A%2F%2Fwww.opengis.net%2Fdef%2Fcrs%2FEPSG%2F0%2F28992&cursor=LGzk%7Cg1aU6A&f=json&limit=5";
    
    let client = Client::new();
    let res = client.get(url)
        .header("Accept", "application/geo+json")
        .send()?
        .text()?;

    let geojson: Value = serde_json::from_str(&res)?;
    let mut parsed_data = Vec::new();

    if let Some(features) = geojson["features"].as_array() {
        for feature in features {
            let props = &feature["properties"];
            let geom = &feature["geometry"];
            
            let perceel_id = format!("{} {} {}", 
                props["kadastraleGemeenteWaarde"].as_str().unwrap_or(""),
                props["sectie"].as_str().unwrap_or(""),
                props["perceelnummer"].as_str().unwrap_or("")
            );

            parsed_data.push(serde_json::json!({
                "id": perceel_id.trim(),
                "coordinates": geom["coordinates"]
            }));
        }
    }

    fs::write("percelen_export.json", serde_json::to_string_pretty(&parsed_data)?)?;
    println!("Export voltooid: percelen_export.json");
    Ok(())
}
