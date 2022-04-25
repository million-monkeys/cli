use crate::utils::*;
use case_style::CaseStyle;
use colored::*;
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
        "uint64" => "std::uint64_t",
        "int8" => "std::int8_t",
        "int16" => "std::int15_t",
        "int32" => "std::int32_t",
        "int64" => "std::int64_t",
        "byte" => "std::byte",
        "ref" => "entt::hashed_string::hash_type",
        "hashed-string" => "entt::hashed_string",
        "vec3" => "glm::vec3",
        "vec4" => "glm::vec4",
        "vec2" => "glm::vec2",
        "float" => "float",
        "double" => "double",
        "bool" => "bool",
        "rgb" => "glm::vec3",
        "rgba" => "glm::vec4",
    },
};
static DATA_TYPES_LUA: Types = Types {
    types: phf_map! {
        "entity" => "uint32_t",
        "uint8" => "uint8_t",
        "uint16" => "uint15_t",
        "uint32" => "uint32_t",
        "uint64" => "uint64_t",
        "int8" => "int8_t",
        "int16" => "int15_t",
        "int32" => "int32_t",
        "int64" => "int64_t",
        "byte" => "uint8_t",
        "ref" => "uint32_t",
        "hashed-string" => "uint32_t",
        "vec2" => "struct Vec2",
        "vec3" => "struct Vec3",
        "vec4" => "struct Vec4",
        "float" => "float",
        "double" => "double",
        "bool" => "bool",
        "rgb" => "struct RGB",
        "rgba" => "struct RGBA",
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
        CaseStyle::from_kebabcase(event_name).to_pascalcase(),
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

fn generate_event_name_pair((event_name, _): (&str, &Item)) -> String {
    format!(
        "\t{{name='{}', type='{}_Event'}},",
        event_name,
        CaseStyle::from_kebabcase(event_name).to_pascalcase(),
    )
}

fn generate_events_map(events: &Document) -> String {
    events
        .iter()
        .map(generate_event_name_pair)
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn generate(source: &str, generate_lua: bool, generate_header: bool, output_dir: &str) {
    let events = fs::read_to_string(source)
        .expect(format!("Events file {} must exist", source).as_str())
        .parse::<Document>()
        .expect(format!("Events file {} must be valid TOML file", source).as_str());
    if generate_lua {
        println!(
            "Outputting to: {}",
            format!("{}/events.lua", output_dir).blue()
        );
        make_file(
            &format!("{}/events.lua", output_dir),
            "project/events.lua",
            &liquid::object!({
                "cdef":  generate_events(&DATA_TYPES_LUA, "_Event", &events),
                "event_types_map": generate_events_map(&events),
            }),
        );
    }
    if generate_header {
        let events_code = generate_events(&DATA_TYPES_CPP, "", &events);
        let cpp = format!("namespace events {{\n{}\n}}\n", events_code);
        println!("{}", cpp);
        println!(
            "Outputting to: {}",
            format!("{}/events.hpp", output_dir).blue()
        );
    }
}
