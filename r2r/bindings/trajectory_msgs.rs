  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct JointTrajectory {

                              pub header: std_msgs::msg::Header,
pub joint_names: Vec<std::string::String>,
pub points: Vec<trajectory_msgs::msg::JointTrajectoryPoint>,

                          }

                          impl WrappedTypesupport for JointTrajectory { 

            type CStruct = trajectory_msgs__msg__JointTrajectory; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__trajectory_msgs__msg__JointTrajectory() }
            }

            fn create_msg() -> *mut trajectory_msgs__msg__JointTrajectory {

                unsafe { trajectory_msgs__msg__JointTrajectory__create() }

            }

            fn destroy_msg(msg: *mut trajectory_msgs__msg__JointTrajectory) -> () {

                unsafe { trajectory_msgs__msg__JointTrajectory__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> JointTrajectory {
  JointTrajectory {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
joint_names: msg.joint_names.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
points : {
let mut temp = Vec::with_capacity(msg.points.size);
let slice = unsafe { std::slice::from_raw_parts(msg.points.data, msg.points.size)};
for s in slice { temp.push(trajectory_msgs::msg::JointTrajectoryPoint::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.joint_names.update(&self.joint_names);
unsafe { trajectory_msgs__msg__JointTrajectoryPoint__Sequence__fini(&mut msg.points) };
unsafe { trajectory_msgs__msg__JointTrajectoryPoint__Sequence__init(&mut msg.points, self.points.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.points.data, msg.points.size)};
for (t,s) in slice.iter_mut().zip(&self.points) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for JointTrajectory {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<JointTrajectory>::new();
                                  JointTrajectory::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct JointTrajectoryPoint {

                              pub positions: Vec<f64>,
pub velocities: Vec<f64>,
pub accelerations: Vec<f64>,
pub effort: Vec<f64>,
pub time_from_start: builtin_interfaces::msg::Duration,

                          }

                          impl WrappedTypesupport for JointTrajectoryPoint { 

            type CStruct = trajectory_msgs__msg__JointTrajectoryPoint; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__trajectory_msgs__msg__JointTrajectoryPoint() }
            }

            fn create_msg() -> *mut trajectory_msgs__msg__JointTrajectoryPoint {

                unsafe { trajectory_msgs__msg__JointTrajectoryPoint__create() }

            }

            fn destroy_msg(msg: *mut trajectory_msgs__msg__JointTrajectoryPoint) -> () {

                unsafe { trajectory_msgs__msg__JointTrajectoryPoint__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> JointTrajectoryPoint {
  JointTrajectoryPoint {
// is_upper_bound_: false
// member.array_size_ : 0
positions: msg.positions.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
velocities: msg.velocities.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
accelerations: msg.accelerations.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
effort: msg.effort.to_vec(),
time_from_start: builtin_interfaces::msg::Duration::from_native(&msg.time_from_start),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.positions.update(&self.positions);
msg.velocities.update(&self.velocities);
msg.accelerations.update(&self.accelerations);
msg.effort.update(&self.effort);
self.time_from_start.copy_to_native(&mut msg.time_from_start);
}



        }


                          
                          impl Default for JointTrajectoryPoint {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<JointTrajectoryPoint>::new();
                                  JointTrajectoryPoint::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MultiDOFJointTrajectory {

                              pub header: std_msgs::msg::Header,
pub joint_names: Vec<std::string::String>,
pub points: Vec<trajectory_msgs::msg::MultiDOFJointTrajectoryPoint>,

                          }

                          impl WrappedTypesupport for MultiDOFJointTrajectory { 

            type CStruct = trajectory_msgs__msg__MultiDOFJointTrajectory; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__trajectory_msgs__msg__MultiDOFJointTrajectory() }
            }

            fn create_msg() -> *mut trajectory_msgs__msg__MultiDOFJointTrajectory {

                unsafe { trajectory_msgs__msg__MultiDOFJointTrajectory__create() }

            }

            fn destroy_msg(msg: *mut trajectory_msgs__msg__MultiDOFJointTrajectory) -> () {

                unsafe { trajectory_msgs__msg__MultiDOFJointTrajectory__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MultiDOFJointTrajectory {
  MultiDOFJointTrajectory {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
joint_names: msg.joint_names.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
points : {
let mut temp = Vec::with_capacity(msg.points.size);
let slice = unsafe { std::slice::from_raw_parts(msg.points.data, msg.points.size)};
for s in slice { temp.push(trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.joint_names.update(&self.joint_names);
unsafe { trajectory_msgs__msg__MultiDOFJointTrajectoryPoint__Sequence__fini(&mut msg.points) };
unsafe { trajectory_msgs__msg__MultiDOFJointTrajectoryPoint__Sequence__init(&mut msg.points, self.points.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.points.data, msg.points.size)};
for (t,s) in slice.iter_mut().zip(&self.points) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for MultiDOFJointTrajectory {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MultiDOFJointTrajectory>::new();
                                  MultiDOFJointTrajectory::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MultiDOFJointTrajectoryPoint {

                              pub transforms: Vec<geometry_msgs::msg::Transform>,
pub velocities: Vec<geometry_msgs::msg::Twist>,
pub accelerations: Vec<geometry_msgs::msg::Twist>,
pub time_from_start: builtin_interfaces::msg::Duration,

                          }

                          impl WrappedTypesupport for MultiDOFJointTrajectoryPoint { 

            type CStruct = trajectory_msgs__msg__MultiDOFJointTrajectoryPoint; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__trajectory_msgs__msg__MultiDOFJointTrajectoryPoint() }
            }

            fn create_msg() -> *mut trajectory_msgs__msg__MultiDOFJointTrajectoryPoint {

                unsafe { trajectory_msgs__msg__MultiDOFJointTrajectoryPoint__create() }

            }

            fn destroy_msg(msg: *mut trajectory_msgs__msg__MultiDOFJointTrajectoryPoint) -> () {

                unsafe { trajectory_msgs__msg__MultiDOFJointTrajectoryPoint__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MultiDOFJointTrajectoryPoint {
  MultiDOFJointTrajectoryPoint {
// is_upper_bound_: false
// member.array_size_ : 0
transforms : {
let mut temp = Vec::with_capacity(msg.transforms.size);
let slice = unsafe { std::slice::from_raw_parts(msg.transforms.data, msg.transforms.size)};
for s in slice { temp.push(geometry_msgs::msg::Transform::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
velocities : {
let mut temp = Vec::with_capacity(msg.velocities.size);
let slice = unsafe { std::slice::from_raw_parts(msg.velocities.data, msg.velocities.size)};
for s in slice { temp.push(geometry_msgs::msg::Twist::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
accelerations : {
let mut temp = Vec::with_capacity(msg.accelerations.size);
let slice = unsafe { std::slice::from_raw_parts(msg.accelerations.data, msg.accelerations.size)};
for s in slice { temp.push(geometry_msgs::msg::Twist::from_native(s)); }
temp },
time_from_start: builtin_interfaces::msg::Duration::from_native(&msg.time_from_start),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { geometry_msgs__msg__Transform__Sequence__fini(&mut msg.transforms) };
unsafe { geometry_msgs__msg__Transform__Sequence__init(&mut msg.transforms, self.transforms.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.transforms.data, msg.transforms.size)};
for (t,s) in slice.iter_mut().zip(&self.transforms) { s.copy_to_native(t);}
unsafe { geometry_msgs__msg__Twist__Sequence__fini(&mut msg.velocities) };
unsafe { geometry_msgs__msg__Twist__Sequence__init(&mut msg.velocities, self.velocities.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.velocities.data, msg.velocities.size)};
for (t,s) in slice.iter_mut().zip(&self.velocities) { s.copy_to_native(t);}
unsafe { geometry_msgs__msg__Twist__Sequence__fini(&mut msg.accelerations) };
unsafe { geometry_msgs__msg__Twist__Sequence__init(&mut msg.accelerations, self.accelerations.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.accelerations.data, msg.accelerations.size)};
for (t,s) in slice.iter_mut().zip(&self.accelerations) { s.copy_to_native(t);}
self.time_from_start.copy_to_native(&mut msg.time_from_start);
}



        }


                          
                          impl Default for MultiDOFJointTrajectoryPoint {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MultiDOFJointTrajectoryPoint>::new();
                                  MultiDOFJointTrajectoryPoint::from_native(&msg_native)
                              }
                          }
             

                          


                      }
