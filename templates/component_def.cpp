		{ // components::{{namespace}}::{{class_name}}
			monkeys::api::definitions::Component component_def {"{{component_name}}"_hs, entt::type_hash<components::{{namespace}}::{{class_name}}>::value(), "{{namespace}}", "{{class_name}}"};
			component_def.size_in_bytes = sizeof(components::{{namespace}}::{{class_name}});
			component_def.loader = [](monkeys::api::Engine* engine, entt::registry& registry, const void* tableptr, entt::entity entity) {
				{% if loader_args != "" %}const auto& table = *reinterpret_cast<const toml::value*>(tableptr);
				{{loader_vars}}
				registry.emplace_or_replace<components::{{namespace}}::{{class_name}}>(entity, {{loader_args}});{% else %}registry.emplace_or_replace<components::{{namespace}}::{{class_name}}>(entity);{% endif %}
			};
            {{component_attributes}}
			{% if loader_args == "" %}component_def.getter = nullptr;{% else %}component_def.getter = [](entt::registry& registry, entt::entity entity){ return (char*)&(registry.get<components::{{namespace}}::{{class_name}}>(entity)); };{% endif %}
			component_def.attached_to_entity = [](entt::registry& registry, entt::entity entity){ return registry.any_of<components::{{namespace}}::{{class_name}}>(entity); };
			component_def.manage = [](entt::registry& registry, entt::entity entity, monkeys::api::definitions::ManageOperation op){
				switch (op) {
					case monkeys::api::definitions::ManageOperation::Add:
						registry.emplace_or_replace<components::{{namespace}}::{{class_name}}>(entity);
						break;
					case monkeys::api::definitions::ManageOperation::Remove:
						registry.remove<components::{{namespace}}::{{class_name}}>(entity);
						break;
					default: break;
				}
			};
			engine->registerComponent<components::{{namespace}}::{{class_name}}>(component_def);
		}