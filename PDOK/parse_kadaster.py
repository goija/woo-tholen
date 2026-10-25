#!/usr/bin/env python3
import urllib.request
import urllib.parse
import json
import sys

def fetch_and_parse(url):
    print(f"Bevragen van PDOK OGC API...\nURL: {url}\n")
    
    # Headers instellen zoals PDOK verwacht
    req = urllib.request.Request(url, headers={'Accept': 'application/geo+json'})
    
    try:
        with urllib.request.urlopen(req) as response:
            data = json.loads(response.read().decode('utf-8'))
            
            features = data.get('features', [])
            print(f"Aantal percelen op deze pagina: {len(features)}\n")
            print(f"{'AANDUIDING':<25} | {'PERCEEL':<8} | {'SECTIE':<6} | {'GEOMETRIE'}")
            print("-" * 65)
            
            for f in features:
                props = f.get('properties', {})
                geom = f.get('geometry', {}) or {}
                
                # Attributen extraheren (afhankelijk van de PDOK veldnamen)
                gemeente = props.get('kadastraleGemeenteWaarde', 'ONBEKEND')
                sectie = props.get('sectie', '')
                perceelnummer = props.get('perceelnummer', '')
                aanduiding = f"{gemeente} {sectie} {perceelnummer}".strip()
                
                geom_type = geom.get('type', 'Geen Geometrie')
                
                print(f"{aanduiding:<25} | {perceelnummer:<8} | {sectie:<6} | {geom_type}")
            
            print("-" * 65)
            
            # Zoek naar de next-link voor paginatie (cursor)
            links = data.get('links', [])
            next_url = next((link['href'] for link in links if link.get('rel') == 'next'), None)
            
            if next_url:
                print(f"\nVolgende pagina (cursor) beschikbaar:")
                print(next_url)
            else:
                print("\nGeen verdere pagina's (einde van dataset).")
                
    except urllib.error.URLError as e:
        print(f"Fout bij het ophalen van de API: {e}")
        sys.exit(1)

if __name__ == '__main__':
    # Gebruik de opgegeven URL als argument, of de standaard cursor-URL
    default_url = "https://api.pdok.nl/kadaster/brk-kadastrale-kaart/ogc/v1/collections/perceel/items?bbox=66000%2C396000%2C67000%2C397000&bbox-crs=http%3A%2F%2Fwww.opengis.net%2Fdef%2Fcrs%2FEPSG%2F0%2F28992&crs=http%3A%2F%2Fwww.opengis.net%2Fdef%2Fcrs%2FEPSG%2F0%2F28992&cursor=LGzk%7Cg1aU6A&f=json&limit=5"
    
    target_url = sys.argv[1] if len(sys.argv) > 1 else default_url
    fetch_and_parse(target_url)
