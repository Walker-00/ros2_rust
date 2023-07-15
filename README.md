<div align="center">

[![Rust](https://github.com/Walker-00/ros2_rust/actions/workflows/rust.yml/badge.svg)](https://github.com/Walker-00/ros2_rust/actions/workflows/rust.yml)

</div>

# ros2_rust
Just examples to get start with ros2 with rust

# PreRequired

- [Rust installed](https://rust-lang.org)
- [Ros2 Foxy, Galactic or Humble](https://ros.org)
- [just](https://github.com/casey/just)

# Usage

### help message

```
$ just -l
Available recipes:
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

