fn main() {
    protospec_build::compile_spec(
        "format",
        include_str!("../spec/format.pspec"),
        &protospec_build::Options {
            // include_async: true,
            // use_anyhow: true,
            // debug_mode: true,
            enum_derives: vec![
                "Default".to_string(),
                "Clone".to_string(),
                "Debug".to_string(),
                "PartialEq".to_string(),
            ],
            struct_derives: vec![
                "Default".to_string(),
                "Clone".to_string(),
                "Debug".to_string(),
                "PartialEq".to_string(),
            ],
            ..Default::default()
        },
    )
    .expect("failed to build format.pspec");
}
