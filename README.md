# Saleae binary format parser

This Rust crate allows you to parse Saleae binary export format that is used in data exports from Saleae Logic 2.

## Usage
```rust
let data0 = include_bytes!("../test/digital_0.bin");
let mut br = BufReader::new(&data0[..]);
match SaleaePacket::decode_sync(&mut br) {
    Err(e) => println!("e: {}", e),
    Ok(a) => println!("{:#?}", a),
}
```

## Format description
The official format documentation can be found [here](https://support.saleae.com/faq/technical-faq/binary-export-format-logic-2).

All types are in little-endian.

## Header
```
id: "<SALEAE>"
version: i32
type_id: i32
```

Where `id` is always `<SALEAE>`, version `0` (for now) and `type_id` can be `0` for digital and `1` for analog.

## Digital Type
```
initial_state: u32
begin_time: u32
end_time: u32
num_transitions: u64
transitions: Vec<f64>
```
|Field|Meaning|
|---|---|
|`initial_state`|The digital state the capture started  with|
|`begin_time`|The base time of samples|
|`end_time`|The end time of samples|
|`num_transitions`|Number of transitions|
|`transitions`|`Vec<f64>` containing times of transitions|

## Analog Type
```
begin_time: f64
sample_rate: u64
downsample: u64
num_samples: u64
samples: Vec<f32>
```
|Field|Meaning|
|---|---|
|`begin_time`|The base time of samples|
|`sample_rate`|Amount of samples per second|
|`downsample`|?|
|`num_samples`|Amount of samples present|
|`samples`|`Vec<f32>` containing voltages at the sample time|