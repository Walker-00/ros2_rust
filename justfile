alias c := controller
alias s := subscriber
alias e := service
alias t := turtlesim

# run turtlesim with ros2
turtlesim:
  ros2 run turtlesim turtlesim_node

# run turtlesim controller with rust
controller:
  cargo run --bin turtlesim_controller

# run turtlesim pose subscriber with rust
subscriber:
  cargo run --bin turtlesim_pose_subscriber

# run turtlesim SetPen service client with rust
service:
  cargo run --bin turtlesim_setpen_service
