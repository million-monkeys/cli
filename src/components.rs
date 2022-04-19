use crate::utils::*;
use case_style::CaseStyle;
use multimap::MultiMap;
use phf::phf_map;
use std::collections::{HashMap, HashSet};
use std::fs;
use toml_edit::{value, ArrayOfTables, Document, Item, Table};

#[derive(PartialEq)]
pub enum GeneratorType {
    HeaderFile,
    LuaDefinitions,
    CppDefinitions,
}

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
        "flags8" => "std::uint8_t",
        "flags16" => "std::uint16_t",
        "flags32" => "std::uint32_t",
        "flags64" => "std::uint64_t",
        "ref" => "entt::hashed_string::hash_type",
        "signal" => "entt::hashed_string::hash_type",
        "hashed-string" => "entt::hashed_string",
        "vec3" => "glm::vec3",
        "vec4" => "glm::vec4",
        "vec2" => "glm::vec2",
        "resource" => "monkeys::resources::Handle",
        "texture" => "monkeys::resources::Handle",
        "mesh" => "monkeys::resources::Handle",
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
        "flags8" => "uint8_t",
        "flags16" => "uint16_t",
        "flags32" => "uint32_t",
        "flags64" => "uint64_t",
        "ref" => "uint32_t",
        "signal" => "uint32_t",
        "hashed-string" => "uint32_t",
        "vec3" => "struct {float x, y, z;}",
        "vec4" => "struct {float x, y, z, w;}",
        "vec2" => "struct {float x, y;}",
        "resource" => "uint32_t",
        "texture" => "uint32_t",
        "mesh" => "uint32_t",
        "float" => "float",
        "double" => "double",
        "bool" => "bool",
        "rgb" => "struct {float r, g, b;}",
        "rgba" => "struct {float r, g, b, a;}",
    },
};

impl Types {
    fn generate_component_field(
        &self,
        indent: &str,
        foreign_types: &mut HashSet<String>,
        include_specifier: bool,
        (field_name, field_type): (&str, &Item),
    ) -> String {
        let data_type = match field_type.as_str() {
            Some(data) => data,
            None => field_type
                .get("type")
                .unwrap_or(&Item::None)
                .as_str()
                .expect(
                    format!(
                        "\"{}\" or \"{}.type\" must be a string naming the field type",
                        field_name, field_name
                    )
                    .as_str(),
                ),
        };
        format!(
            "{}\t{} {};",
            indent,
            (if data_type.starts_with("ptr:") {
                let type_name = data_type
                    .get(4..)
                    .expect("ptr:<type-name> must include a type name");
                foreign_types.insert(type_name.to_string());
                format!(
                    "{}{}*",
                    if include_specifier { "struct " } else { "" },
                    type_name
                )
            } else {
                self.types
                    .get(data_type)
                    .expect(
                        format!(
                            "\"{}\" must name a valid field data type, got: {}",
                            field_name, data_type
                        )
                        .as_str(),
                    )
                    .to_string()
            })
            .as_str(),
            CaseStyle::from_kebabcase(field_name).to_snakecase(),
        )
    }
}

fn gen_field(temp_vars: &mut Vec<String>, field_type: &str, field_name: &str) -> String {
    match field_type {
        "hashed-string" => format!(
            "entt::hashed_string{{toml::find<std::string>(table, \"{}\").c_str()}}",
            field_name
        ),
        "ref" | "signal" => format!(
            "entt::hashed_string::value(toml::find<std::string>(table, \"{}\").c_str())",
            field_name
        ),
        "resource" | "texture" | "mesh" => format!(
            "engine->findResource(entt::hashed_string::value(toml::find<std::string>(table, \"{}\").c_str()))",
            field_name
        ),
        "vec2" => {
            temp_vars.push(format!("auto {0} = table.at(\"{0}\");", field_name));
            format!("glm::vec3{{float(toml::find<toml::floating>({0}, \"x\")), float(toml::find<toml::floating>({0}, \"y\"))}}", field_name)
        },
        "vec3" => {
            temp_vars.push(format!("auto {0} = table.at(\"{0}\");", field_name));
            format!("glm::vec3{{float(toml::find<toml::floating>({0}, \"x\")), float(toml::find<toml::floating>({0}, \"y\")), float(toml::find<toml::floating>({0}, \"z\"))}}", field_name)
        },
        "vec4" => {
            temp_vars.push(format!("auto {0} = table.at(\"{0}\");", field_name));
            format!("glm::vec3{{float(toml::find<toml::floating>({0}, \"x\")), float(toml::find<toml::floating>({0}, \"y\")), float(toml::find<toml::floating>({0}, \"z\")), float(toml::find<toml::floating>({0}, \"w\"))}}", field_name)
        },
        "rgb" => {
            temp_vars.push(format!("auto {0} = table.at(\"{0}\");", field_name));
            format!("glm::vec3{{float(toml::find<toml::floating>({0}, \"r\")), float(toml::find<toml::floating>({0}, \"g\")), float(toml::find<toml::floating>({0}, \"b\"))}}", field_name)
        },
        "rgba" => {
            temp_vars.push(format!("auto {0} = table.at(\"{0}\");", field_name));
            format!("glm::vec3{{float(toml::find<toml::floating>({0}, \"r\")), float(toml::find<toml::floating>({0}, \"g\")), float(toml::find<toml::floating>({0}, \"b\")), float(toml::find<toml::floating>({0}, \"a\"))}}", field_name)
        },
        type_name @ ("uint64" | "uint32" | "uint16" | "uint8" |
        "int64" | "int32" | "int16" | "int8" |
        "byte" | "flags8" | "flags16" | "flags32" | "flags64") => format!("{}(toml::find<toml::integer>(table, \"{}\"))", DATA_TYPES_CPP.types.get(type_name).unwrap(), field_name),
        type_name @ ("float" | "double") => format!("{}(toml::find<toml::floating>(table, \"{}\"))", DATA_TYPES_CPP.types.get(type_name).unwrap(), field_name),
        "bool" => format!("bool(toml::find<toml::boolean>(table, \"{}\"))", field_name),
        type_name => {
            if type_name.starts_with("ptr:") {
                String::from("nullptr")
            } else {
                format!("/* UNKNOWN: {} {} */", field_type, field_name)
            }
        }
    }
}

fn generate_field_accessor(
    temp_vars: &mut Vec<String>,
    (field_name, field_type): (&str, &Item),
) -> String {
    if field_type.is_str() {
        gen_field(temp_vars, field_type.as_str().unwrap(), field_name)
    } else if field_type.is_table_like() {
        let field = field_type.as_table_like().unwrap();
        if field.contains_key("type") {
            gen_field(
                temp_vars,
                field.get("type").unwrap().as_str().expect(&format!(
                    "'type' property of field must be string: {}",
                    field_name
                )),
                field_name,
            )
        } else {
            format!("/* UNKNOWN: {} */", field_name)
        }
    } else {
        format!("/* UNKNOWN: {} */", field_name)
    }
}

fn generate_component_def(namespace: &str, component: &Table) -> String {
    let component_name = component
        .get("_name_")
        .expect("Component must contain _name_ field")
        .as_str()
        .expect("Name field must be a string");

    let sub_namespace = component
        .get("_namespace_")
        .map(|n| n.as_str().expect("_namespace_ must be a string"))
        .unwrap_or("");

    let namespaced_name = format!(
        "{}{}{}",
        sub_namespace,
        if sub_namespace == "" { "" } else { "/" },
        component_name
    );
    let namespace = if namespace == "" { "core" } else { namespace };
    let namespace = if sub_namespace == "" {
        String::from(namespace)
    } else {
        format!("{}::{}", namespace, sub_namespace)
    };

    let fields = component
        .iter()
        .filter(|(k, _)| !(k.starts_with("_") && k.ends_with("_")));

    let mut temp_vars: Vec<String> = Vec::new();

    let accessors = fields
        .map(|x| generate_field_accessor(&mut temp_vars, x))
        .collect::<Vec<String>>()
        .join(", ");

    template_to_str(
        "component_def.cpp",
        &liquid::object!({
            "namespace": namespace,
            "component_name": namespaced_name,
            "class_name": CaseStyle::from_kebabcase(component_name).to_pascalcase(),
            "loader_vars": temp_vars.join("\n\t\t\t\t"),
            "loader_args": accessors,
            "component_attributes": "",
        }),
    )
}

fn generate_component_defs(namespace: &str, components: &ArrayOfTables) -> String {
    components
        .iter()
        .map(|x| generate_component_def(namespace, x))
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate_component<'a>(component: &'a Table) -> (&'a str, String, String, &'a Table) {
    let component_name = component
        .get("_name_")
        .expect("Component must contain _name_ field")
        .as_str()
        .expect("Name field must be a string");

    let namespace = component
        .get("_namespace_")
        .map(|n| n.as_str().expect("_namespace_ must be a string"));

    let indent = match namespace {
        Some(_) => "\t\t",
        None => "\t",
    };
    let description = match component.get("_description_") {
        Some(description) => format!(
            "{}// {}\n",
            indent,
            description
                .as_str()
                .expect("_description_ must be a string")
        ),
        None => String::from(""),
    };

    (
        namespace.unwrap_or(""),
        component_name.to_string(),
        description,
        component,
    )
}

fn generate_component_struct<'a>(
    types: &Types,
    foreign_types: &mut HashSet<String>,
    include_specifier: bool,
    component_name: &str,
    description: &str,
    indent: &str,
    component: &Table,
) -> String {
    format!(
        "\n{}{}struct {} {{\n{}\n{}}};",
        description,
        indent,
        component_name,
        component
            .iter()
            .filter(|(k, _)| !(k.starts_with("_") && k.ends_with("_")))
            .map(|x| types.generate_component_field(indent, foreign_types, include_specifier, x))
            .collect::<Vec<String>>()
            .join("\n"),
        indent,
    )
}

fn generate_components<'a>(
    types: &Types,
    foreign_types: &mut HashSet<String>,
    components: &ArrayOfTables,
    cpp_output: bool,
) -> String {
    let component_code = components.iter().map(generate_component);

    let mut code = String::from("");
    let mut component_map: MultiMap<String, (String, String, &Table)> = MultiMap::new();
    for (namespace, component_name, description, component) in component_code {
        let component_name = CaseStyle::from_kebabcase(component_name).to_pascalcase();
        if namespace == "" {
            code.push_str(
                generate_component_struct(
                    types,
                    foreign_types,
                    !cpp_output,
                    (if cpp_output {
                        component_name
                    } else {
                        format!("Component_Core_{}", component_name)
                    })
                    .as_str(),
                    if cpp_output { description.as_str() } else { "" },
                    "\t",
                    component,
                )
                .as_str(),
            );
            if cpp_output {
                code.push_str("\n");
            }
        } else {
            let namespace_name = CaseStyle::from_kebabcase(namespace).to_snakecase();
            component_map.insert(
                namespace_name.to_string(),
                (
                    if cpp_output {
                        component_name
                    } else {
                        format!("Component_{}_{}", namespace_name, component_name)
                    },
                    description,
                    component,
                ),
            );
        }
    }

    if !component_map.is_empty() {
        if !cpp_output {
            code.push_str("\n");
        }
        for (namespace_name, component_list) in component_map.iter_all() {
            if cpp_output {
                code.push_str(&format!("\n\tnamespace {} {{\n", namespace_name,));
            }
            for (component_name, description, component) in component_list {
                code.push_str(
                    generate_component_struct(
                        types,
                        foreign_types,
                        !cpp_output,
                        component_name,
                        if cpp_output { description } else { "" },
                        if cpp_output { "\t\t" } else { "\t" },
                        component,
                    )
                    .as_str(),
                );
                if cpp_output {
                    code.push_str("\n");
                }
            }
            if cpp_output {
                code.push_str(&format!("\n\t}} // {}\n", namespace_name));
            } else {
                code.push_str("\n");
            }
        }
    }
    code
}

fn generate_component_name_pair(namespace: &str, name: &str) -> String {
    format!(
        "\t[\"{0}{1}{2}\"] = \"struct Component_{3}_{4}*\",",
        namespace,
        if namespace == "" { "" } else { "/" },
        name,
        if namespace == "" { "Core" } else { namespace },
        CaseStyle::from_kebabcase(name).to_pascalcase(),
    )
}

fn generate_component_map(components: &ArrayOfTables) -> String {
    components
        .iter()
        .map(generate_component)
        .map(|(namespace, name, _, _)| generate_component_name_pair(namespace, &name))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn generate(source: &str, generate_what: GeneratorType, output_dir: &str) {
    let defs = fs::read_to_string(source)
        .expect(format!("Components file {} must exist", source).as_str())
        .parse::<Document>()
        .expect(format!("Components file {} must be valid TOML file", source).as_str());

    if !defs.contains_array_of_tables("component") {
        panic!(
            "Components file {} does not contain array of tables field: components",
            source
        );
    }

    let namespace = defs
        .get("namespace")
        .expect("Must specify a namespace")
        .as_str()
        .expect("Namespace must be a string");

    let components = defs.get("component").unwrap().as_array_of_tables().unwrap();

    let output_file = format!(
        "{}/{}",
        if output_dir.ends_with('/') {
            output_dir.trim_right_matches('/')
        } else {
            output_dir
        },
        if generate_what != GeneratorType::HeaderFile && namespace == "core" {
            "core_components"
        } else {
            namespace
        }
    );

    let mut foreign_types: HashSet<String> = HashSet::new();

    match generate_what {
        GeneratorType::LuaDefinitions => {
            let components_code =
                generate_components(&DATA_TYPES_LUA, &mut foreign_types, &components, false);
            let component_map = generate_component_map(&components);
            println!("Outputting Lua definition to: {}.lua", output_file);
            make_file(
                &format!("{}.lua", output_file),
                "components.lua",
                &liquid::object!({
                    "cdef": components_code,
                    "component_map": component_map,
                }),
            );
        }
        GeneratorType::HeaderFile => {
            let components_code =
                generate_components(&DATA_TYPES_CPP, &mut foreign_types, &components, true);
            println!("Outputting C++ header file to: {}.hpp", output_file);
            make_file(
                &format!("{}.hpp", output_file),
                "components.hpp",
                &liquid::object!({
                    "namespace": namespace,
                    "components": components_code,
                    "pointer_declarations": foreign_types.iter().map(|x| format!("struct {};", x)).collect::<Vec<String>>().join("\n")
                }),
            );
        }
        GeneratorType::CppDefinitions => {
            println!("Outputting C++ definition to: {}.cpp", output_file);
            make_file(
                &format!("{}.cpp", output_file),
                "components.cpp",
                &liquid::object!({
                    "name": namespace,
                    "components": generate_component_defs(namespace, components),
                }),
            );
        }
    }
}
