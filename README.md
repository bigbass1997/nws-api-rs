[![License: BSD 2-Clause](https://img.shields.io/badge/License-BSD%202--Clause-blue)](LICENSE)
### Description
This is a blocking API wrapper for the [National Weather Service's (NWS) public JSON-LD API](https://www.weather.gov/documentation/services-web-api).

Endpoints currently supported:
```
/gridpoints/{wfo}/{x},{y}
/gridpoints/{wfo}/{x},{y}/stations
/stations/{stationId}/observations
/stations/{stationId}/observations/latest
/stations/{stationId}/observations/{time}
/stations
/stations/{stationId}
/points/{point}
```