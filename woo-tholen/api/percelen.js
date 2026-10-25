export default async function handler(req, res) {
  const bbox = req.query.bbox || "66000,396000,67000,397000";
  const url = `https://service.pdok.nl/kadaster/kadastralekaart/wfs/v5_0?service=WFS&version=2.0.0&request=GetFeature&typeName=kadastralekaart:perceel&outputFormat=application/json&srsName=EPSG:28992&bbox=${bbox},EPSG:28992`;
  const r = await fetch(url);
  res.setHeader("Cache-Control", "s-maxage=3600");
  res.status(r.status).json(await r.json());
}
