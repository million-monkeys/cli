// This file is autogenerated, do not edit!
#include <components/{{name}}.hpp>
#include <million/engine.hpp>
#include <entt/entity/registry.hpp>
#include <toml.hpp>

using namespace entt::literals;

namespace init_{{name}} {

    void register_components (million::api::internal::ModuleManager* engine)
    {
{{components}}
    }

} // init_{{name}}
