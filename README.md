<div align="center">

[![Rust](https://github.com/Walker-00/ros2_rust/actions/workflows/rust.yml/badge.svg)](https://github.com/Walker-00/ros2_rust/actions/workflows/rust.yml)

</div>

# ros2_rust
Just examples to get start with ros2 with rust

# PreRequired

- [Rust installed](https://rust-lang.org)
- [Ros2 Foxy, Galactic or Humble](https://ros.org)
- [just](https://github.com/casey/just)

# Tested

- All of the examples are tested with Ros2 Humble with Arch and Docker with Ubuntu lastest version.
- We use rust crate [r2r](https://docs.rs/r2r) for ros2 client.
- 'Cuz it's the best crate I've found.
- For now [r2r](https://docs.rs/r2r) only support for Ros2 [Foxy, Galactic and Humble] Versions I think.

# Usage

### help message

```
$ just -l
Available recipes:
    action_client      # run action client with rust
    ca                 # alias for `action_client`
    action_server      # run action server with rust
    aa                 # alias for `action_server`
    addtwoints_client  # run addtwoints client with rust
    ac                 # alias for `addtwoints_client`
    addtwoints_service # run addtwoints service with rust
    as                 # alias for `addtwoints_service`
    controller         # run turtlesim controller with rust
    c                  # alias for `controller`
    subscriber         # run turtlesim pose subscriber with rust
    s                  # alias for `subscriber`
    turtlesim          # run turtlesim with ros2
    t                  # alias for `turtlesim`
    turtlesim_service  # run turtlesim SetPen service client with rust
    ts                 # alias for `turtlesim_service`
```

### run turtlesim

```
$ just t # or just turtlesim
```

### run turtlesim controller

```
$ just c # or just controller
```

### run turtlesim pose subscriber

```
$ just s # or just subscriber
```

### run turtlesim SetPen service client

```
$ just ts # or just turtlesim_service
```

### run addtwoints service client

```
$ just as # or just addtwoints_service
```

### run addtwoints service client

```
$ just ac # or just addtwoints_client
```

### run ros parameters

```
$ cargo run --bin ros_parameters --release -- --ros-args -p name:=[Linus, Walker] -p age:=15 -r __ns:=/namespace -r __node:=node
```

- we use `--ros-args` cli argument to add ros additional arguments
- `-p` ros argument to specify the parameters of our program 
- you can change the parameters by running this

```
$ ros2 param set /{namespace_name}/{node_name} {parameter}:={value} # ros2 param set /namespace/node name:=Walker
```

### run action server

```
$ just aa # just action_server
```

### run action client

```
$ just ca # just action_client
```

