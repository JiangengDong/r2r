R2R - Minimal ROS2 Rust bindings
=============

Minimal bindings for ROS2 that does *not* require hooking in to the ROS2 build infrastructure, in contrast to <https://github.com/ros2-rust/ros2_rust>. Message definitions are instead created by calling into the c introspection libraries to avoid the .msg/.idl pipeline.

How to use
------------
You need to source your ros installation before building/running. A couple of examples are included in examples/
```
. /opt/ros/dashing/setup.sh
cargo build
cargo run --example subscriber_with_thread
```

In order to avoid building everything, put the message types you need in msgs.txt.

What works?
--------
- Only tested with ROS2 Dashing
- Simple publish/subscribe, see examples.


TODO
------------
- The code generation is currently just a big hack. Needs cleanup and refactoring.
- There is no proper abstractions for nodes etc. Reuse/share with <https://github.com/ros2-rust/ros2_rust>?
- Implement error handling and cleanup code.
