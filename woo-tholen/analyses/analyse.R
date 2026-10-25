library(jsonlite)

data <- fromJSON("percelen_export.json", simplifyVector = FALSE)

plot(0, 0, type="n", xlab="RD X (m)", ylab="RD Y (m)", 
     xlim=c(66000, 67000), ylim=c(396000, 397000), 
     main="Geometrische controle Platteweg")

calc_area <- function(x, y) {
  0.5 * abs(sum(x[-length(x)] * y[-1] - x[-1] * y[-length(y)]))
}

for (perceel in data) {
  coords <- perceel$coordinates[[1]]
  x_vals <- sapply(coords, function(c) c[[1]])
  y_vals <- sapply(coords, function(c) c[[2]])
  
  polygon(x_vals, y_vals, border="blue", col=rgb(0,0,1,0.1))
  
  oppervlakte <- calc_area(x_vals, y_vals)
  cat(sprintf("Perceel: %s | Berekende indicatieve oppervlakte: %.2f m²\n", perceel$id, oppervlakte))
  
  text(mean(range(x_vals)), mean(range(y_vals)), labels=perceel$id, cex=0.7)
}
