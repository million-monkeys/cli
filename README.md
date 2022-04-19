# cli  
Commandline development tool for working with the million-monkeys game engine projects

## To create a new project:

```sh
cargo run -- projects <name> create
```

## To build a project:

```sh
cargo run -- projects <name> build
```

This will generate an `events.lua` file in your project directory, making these events visible to Lua and the game engine.

## To run a project:

```sh
cargo run -- projects <name> run
```

Not yet implemented.

## To generate a header file from a `components.toml` components listing:

```sh
cargo run -- generate components hpp <path to components.toml> <destination>
```
This will generate the `<destination>/<namespace>.hpp` header file containing the struct definitions of the components listed in the `components.toml` file.

## To generate a Lua setup file from a `components.toml` components listing:

```sh
cargo run -- generate components lua <path to components.toml> <destination>
```
This will generate the `<destination>/<namespace>.lua` header file containing the struct definitions of the components listed in the `components.toml` file and registers them with the scripting system.
After loading this file (once only!), the components are now available in Lua scripts using the `mm_script_api` Lua module.

## To generate a C++ component definition file from a `components.toml` components listing:

```sh
cargo run -- generate components cpp <path to components.toml> <destination>
```
This will generate the `<destination>/<namespace>.cpp` header file containing the initialization function that will register the components, their accessor functions and their TOML loader functions with the engine.
This file then needs to be compiled into the engine or a module (the core components are compiled directly into the engine, all other components should be created by a module). These components may now be used with the engine ECS and can be loaded from TOML files.

## FUTURE

In the future, the CLI tool will be extended to allow:

* Running the engine with a project (setting engine commandline arguments to the necessary paths)
* Generating and building modules (including events and components)
* Development mode that runs an embedded web server with a web based editor
* Creating release packages of projects for distribution
