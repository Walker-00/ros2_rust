alias c := controller
alias s := subscriber
alias ts := turtlesim_service
alias t := turtlesim
alias as := addtwoints_service
alias ac := addtwoints_client

# run turtlesim with ros2
turtlesim:
  ros2 run turtlesim turtlesim_node --release

# run turtlesim controller with rust
controller:
  cargo run --bin turtlesim_controller --release

# run turtlesim pose subscriber with rust
subscriber:
  cargo run --bin turtlesim_pose_subscriber --release

# run turtlesim SetPen service client with rust
turtlesim_service:
  cargo run --bin turtlesim_setpen_service --release

# run addtwoints service with rust
addtwoints_service:
  cargo run --bin addtwoints_service --release

# run addtwoints client with rust
addtwoints_client:
  cargo run --bin addtwoints_client --release
