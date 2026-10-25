use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let client = Client::new();
    
    // Lees de BBOX uit de request URL of gebruik de standaardwaarde (Scherpenisse)
    let query = req.uri().query().unwrap_or("");
    let bbox = query.split('&')
        .find(|p| p.starts_with("bbox="))
        .map(|s| &s[5..])
        .unwrap_or("66000,396000,67000,397000");
        
    // PDOK verwacht URL-encoding voor de komma's (%2C)
    let encoded_bbox = bbox.replace(",", "%2C");
    
    let mut next_url = Some(format!(
        "https://api.pdok.nl/kadaster/brk-kadastrale-kaart/ogc/v1/collections/perceel/items?bbox={}&bbox-crs=http%3A%2F%2Fwww.opengis.net%2Fdef%2Fcrs%2FEPSG%2F0%2F28992&crs=http%3A%2F%2Fwww.opengis.net%2Fdef%2Fcrs%2FEPSG%2F0%2F28992&f=json&limit=100", 
        encoded_bbox
    ));
    
    let mut parsed_data = Vec::new();

    while let Some(url) = next_url {
        let res = client.get(&url)
            .header("Accept", "application/geo+json")
            .send()
            .await?
            .text()
            .await?;

        let geojson: Value = serde_json::from_str(&res)?;

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

        next_url = None;
        if let Some(links) = geojson["links"].as_array() {
            for link in links {
                if link["rel"].as_str() == Some("next") {
                    if let Some(href) = link["href"].as_str() {
                        next_url = Some(href.to_string());
                    }
                    break;
                }
            }
        }
    }

    let json_body = serde_json::to_string(&parsed_data)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Cache-Control", "s-maxage=3600")
        .body(Body::Text(json_body))?)
}
