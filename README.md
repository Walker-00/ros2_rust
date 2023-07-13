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
    controller # run turtlesim controller with rust
    c          # alias for `controller`
    service    # run turtlesim SetPen service client with rust
    e          # alias for `service`
    subscriber # run turtlesim pose subscriber with rust
    s          # alias for `subscriber`
    turtlesim  # run turtlesim with ros2
    t          # alias for `turtlesim
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
$ just e # or just service
```


