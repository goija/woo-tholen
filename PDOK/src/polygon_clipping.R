# Vereist: install.packages("sf") en install.packages("jsonlite")
library(jsonlite)
library(sf)

# 1. Laad de geparste data
data <- fromJSON("percelen_export.json", simplifyVector = FALSE)

# Hulpfunctie: converteer GeoJSON coördinaten naar een sf-polygoon
create_sf_polygon <- function(coords) {
  # Zet de geneste lijsten om naar een matrix die 'sf' begrijpt
  mat <- do.call(rbind, lapply(coords[[1]], function(c) c(c[[1]], c[[2]])))
  st_polygon(list(mat))
}

poly_1551 <- NULL
poly_1552 <- NULL

# 2. Zoek specifiek naar de percelen E 1551 en E 1552
for (perceel in data) {
  if (grepl("E 1551", perceel$id)) {
    poly_1551 <- st_sfc(create_sf_polygon(perceel$coordinates), crs = 28992)
  }
  if (grepl("E 1552", perceel$id)) {
    poly_1552 <- st_sfc(create_sf_polygon(perceel$coordinates), crs = 28992)
  }
}

# 3. Bereken en visualiseer de intersectie (als beide percelen gevonden zijn)
if (!is.null(poly_1551) && !is.null(poly_1552)) {
  
  # st_intersection roept GEOS aan voor exacte topologische overlap
  overlap <- st_intersection(poly_1551, poly_1552)
  
  if (length(overlap) > 0) {
    # Bereken de oppervlakte van de overlap
    overlap_area <- st_area(overlap)
    cat(sprintf("Topologische overlap gevonden!\nIndicatieve overlappende oppervlakte: %.2f m²\n", overlap_area))
    
    # Visualisatie met kleuren
    plot(poly_1551, border = "blue", col = rgb(0,0,1,0.1), main = "Intersectie Scherpenisse E 1551 & E 1552", axes=TRUE)
    plot(poly_1552, border = "red", col = rgb(1,0,0,0.1), add = TRUE)
    plot(overlap, border = "purple", col = rgb(1,0,1,0.5), add = TRUE)
    
    legend("topright", legend=c("E 1551", "E 1552", "Overlap"), 
           fill=c(rgb(0,0,1,0.1), rgb(1,0,0,0.1), rgb(1,0,1,0.5)), 
           border=c("blue", "red", "purple"))
           
  } else {
    cat("Geen geometrische overlap gedetecteerd tussen E 1551 en E 1552 in de OGC-data.\n")
    
    # Teken ze toch om de grenslijn te inspecteren
    plot(poly_1551, border = "blue", main = "Grenslijn E 1551 & E 1552 (Geen overlap)", axes=TRUE)
    plot(poly_1552, border = "red", add = TRUE)
  }
} else {
  cat("Waarschuwing: E 1551 en/of E 1552 ontbreken in de dataset (buiten BBOX of niet opgehaald).\n")
}
