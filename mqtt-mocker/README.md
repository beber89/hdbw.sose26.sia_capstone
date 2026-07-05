# Mqtt Simulator (Mocker)

## Getting Started

- First we need to generate a `settings.json`. To achieve that we run:
```
python3 render_settings.py
```
  - This to render a `settings.json` from `settings.templ.json`

- Run the docker container

```
docker run -v $(pwd)/settings.json:/usr/src/app/settings.json mqtt-simulator:latest -f settings.json
```

