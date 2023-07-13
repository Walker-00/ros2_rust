alias c := controller
alias s := subscriber
alias e := service
alias t := turtlesim

turtlesim:
  ros2 run turtlesim turtlesim_node

controller:
  cargo run --bin turtlesim_controller

subscriber:
  cargo run --bin turtlesim_pose_subscriber

service:
  cargo run --bin turtlesim_setpen_service
