use crate::utils::*;
use phf::phf_map;
use std::collections::HashMap;
use std::fs;
use toml_edit::{value, Document, Item, Table};

struct Types {
    types: phf::Map<&'static str, &'static str>,
}

static DATA_TYPES_CPP: Types = Types {
    types: phf_map! {
        "entity" => "entt::entity",
        "uint8" => "std::uint8_t",
        "uint16" => "std::uint15_t",
        "uint32" => "std::uint32_t",
        "int8" => "std::int8_t",
        "int16" => "std::int15_t",
        "int32" => "std::int32_t",
        "byte" => "std::byte",
        "flags8" => "std::uint8_t",
        "flags16" => "std::uint16_t",
        "flags32" => "std::uint32_t",
        "ref" => "entt::hashed_string::hash_type",
    },
};
static DATA_TYPES_LUA: Types = Types {
    types: phf_map! {
        "entity" => "uint32_t",
        "uint8" => "uint8_t",
        "uint16" => "uint15_t",
        "uint32" => "uint32_t",
        "int8" => "int8_t",
        "int16" => "int15_t",
        "int32" => "int32_t",
        "byte" => "uint8_t",
        "flags8" => "uint8_t",
        "flags16" => "uint16_t",
        "flags32" => "uint32_t",
        "ref" => "uint32_t",
    },
};

impl Types {
    fn generate_event_field(&self, (field_name, field_type): (&str, &Item)) -> String {
        let data_type = field_type
            .as_str()
            .expect(format!("\"{}\" must be a string naming the field type", field_name).as_str());
        format!(
            "    {} {};",
            self.types
                .get(data_type)
                .expect("\"{}\" must name a valid field data type"),
            field_name
        )
    }
}

fn generate_event(
    types: &Types,
    name_suffix: &str,
    (event_name, event_fields): (&str, &Item),
) -> String {
    format!(
        "struct {}{} {{\n{}\n}};",
        event_name,
        name_suffix,
        event_fields
            .as_table()
            .expect(format!("\"{}\" must be a TOML table", event_name).as_str())
            .iter()
            .map(|x| types.generate_event_field(x))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn generate_events<'a>(types: &Types, name_suffix: &str, events: &Document) -> String {
    events
        .iter()
        .map(|x| generate_event(types, name_suffix, x))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn generate(source: &str, generate_lua: bool, generate_header: bool, output_dir: &str) {
    let events = fs::read_to_string(source)
        .expect(format!("Events file {} must exist", source).as_str())
        .parse::<Document>()
        .expect(format!("Events file {} must be valid TOML file", source).as_str());
    if generate_lua {
        let events_code = generate_events(&DATA_TYPES_LUA, "Event", &events);
        println!("Outputting to: {}/events.lua", output_dir);
        make_file(
            &format!("{}/events.lua", output_dir),
            "project/events.lua",
            &liquid::object!({
                "cdef": events_code,
                "event_types_map": "-- events-map",
            }),
        );
    }
    if generate_header {
        let events_code = generate_events(&DATA_TYPES_CPP, "", &events);
        let cpp = format!("namespace events {{\n{}\n}}\n", events_code);
        println!("{}", cpp);
        println!("Outputting to: {}/events.hpp", output_dir);
    }
}
