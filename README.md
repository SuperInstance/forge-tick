# forge-tick

Tile-to-Tick conversion for Plato agent rooms.

Tiles become ticks. Ticks are what Plato agents actually read. This crate converts between the two representations.

## Types

### `Tick`

The core unit — a timestamped, typed piece of content bound to a room and agent.

### `TickType`

Variants: `SensorReading`, `TextLine`, `CodeBlock`, `DataRow`, `AudioChunk`, `ImageRegion`, `SubtitleEntry`, `Status`, `Error`, `Command`.

## Mappers

### `TileToTickMapper`

- `map_text(content, index)` → `TextLine` tick
- `map_sensor(sensor_type, value, unit, timestamp_ms)` → `SensorReading` tick
- `map_code(language, kind, name, body)` → `CodeBlock` tick
- `map_data_row(row_json, index)` → `DataRow` tick
- `map_subtitle(text, start_ms, end_ms)` → `SubtitleEntry` tick
- `map_generic(content, kind)` → tick with inferred type

### `TickToTileMapper`

- `to_text(tick)` — extract raw content
- `to_sensor_values(tick)` — extract `(sensor_type, value, unit)` if sensor tick
- `to_meta_map(tick)` — return full metadata map

## Formatting

### `TickFormatter`

- `format_for_room(tick, room)` — room-stamped line
- `format_compact(tick)` — one-line summary (truncates at 60 chars)
- `format_verbose(tick)` — full multi-line detail
- `format_batch(ticks)` — compact one-per-line for room feeds

## Dependencies

- `serde` + `serde_json`
- `uuid` (v4)

## License

MIT
