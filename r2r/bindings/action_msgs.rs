  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct GoalInfo {

                              pub goal_id: unique_identifier_msgs::msg::UUID,
pub stamp: builtin_interfaces::msg::Time,

                          }

                          impl WrappedTypesupport for GoalInfo { 

            type CStruct = action_msgs__msg__GoalInfo; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalInfo() }
            }

            fn create_msg() -> *mut action_msgs__msg__GoalInfo {

                unsafe { action_msgs__msg__GoalInfo__create() }

            }

            fn destroy_msg(msg: *mut action_msgs__msg__GoalInfo) -> () {

                unsafe { action_msgs__msg__GoalInfo__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> GoalInfo {
  GoalInfo {
goal_id: unique_identifier_msgs::msg::UUID::from_native(&msg.goal_id),
stamp: builtin_interfaces::msg::Time::from_native(&msg.stamp),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.goal_id.copy_to_native(&mut msg.goal_id);
self.stamp.copy_to_native(&mut msg.stamp);
}



        }


                          
                          impl Default for GoalInfo {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<GoalInfo>::new();
                                  GoalInfo::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct GoalStatus {

                              pub goal_info: action_msgs::msg::GoalInfo,
pub status: i8,

                          }

                          impl WrappedTypesupport for GoalStatus { 

            type CStruct = action_msgs__msg__GoalStatus; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalStatus() }
            }

            fn create_msg() -> *mut action_msgs__msg__GoalStatus {

                unsafe { action_msgs__msg__GoalStatus__create() }

            }

            fn destroy_msg(msg: *mut action_msgs__msg__GoalStatus) -> () {

                unsafe { action_msgs__msg__GoalStatus__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> GoalStatus {
  GoalStatus {
goal_info: action_msgs::msg::GoalInfo::from_native(&msg.goal_info),
status: msg.status,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.goal_info.copy_to_native(&mut msg.goal_info);
msg.status = self.status;
}



        }


                          
                          impl Default for GoalStatus {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<GoalStatus>::new();
                                  GoalStatus::from_native(&msg_native)
                              }
                          }
             

                          
                          #[allow(non_upper_case_globals)]
                          impl GoalStatus {
                                pub const STATUS_UNKNOWN: _bindgen_ty_1 = action_msgs__msg__GoalStatus__STATUS_UNKNOWN;
  pub const STATUS_ACCEPTED: _bindgen_ty_2 = action_msgs__msg__GoalStatus__STATUS_ACCEPTED;
  pub const STATUS_EXECUTING: _bindgen_ty_3 = action_msgs__msg__GoalStatus__STATUS_EXECUTING;
  pub const STATUS_CANCELING: _bindgen_ty_4 = action_msgs__msg__GoalStatus__STATUS_CANCELING;
  pub const STATUS_SUCCEEDED: _bindgen_ty_5 = action_msgs__msg__GoalStatus__STATUS_SUCCEEDED;
  pub const STATUS_CANCELED: _bindgen_ty_6 = action_msgs__msg__GoalStatus__STATUS_CANCELED;
  pub const STATUS_ABORTED: _bindgen_ty_7 = action_msgs__msg__GoalStatus__STATUS_ABORTED;
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct GoalStatusArray {

                              pub status_list: Vec<action_msgs::msg::GoalStatus>,

                          }

                          impl WrappedTypesupport for GoalStatusArray { 

            type CStruct = action_msgs__msg__GoalStatusArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalStatusArray() }
            }

            fn create_msg() -> *mut action_msgs__msg__GoalStatusArray {

                unsafe { action_msgs__msg__GoalStatusArray__create() }

            }

            fn destroy_msg(msg: *mut action_msgs__msg__GoalStatusArray) -> () {

                unsafe { action_msgs__msg__GoalStatusArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> GoalStatusArray {
  GoalStatusArray {
// is_upper_bound_: false
// member.array_size_ : 0
status_list : {
let mut temp = Vec::with_capacity(msg.status_list.size);
let slice = unsafe { std::slice::from_raw_parts(msg.status_list.data, msg.status_list.size)};
for s in slice { temp.push(action_msgs::msg::GoalStatus::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { action_msgs__msg__GoalStatus__Sequence__fini(&mut msg.status_list) };
unsafe { action_msgs__msg__GoalStatus__Sequence__init(&mut msg.status_list, self.status_list.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.status_list.data, msg.status_list.size)};
for (t,s) in slice.iter_mut().zip(&self.status_list) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for GoalStatusArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<GoalStatusArray>::new();
                                  GoalStatusArray::from_native(&msg_native)
                              }
                          }
             

                          


                      }
  pub mod srv {
#[allow(non_snake_case)]
    pub mod CancelGoal {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__action_msgs__srv__CancelGoal()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub goal_info: action_msgs::msg::GoalInfo,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = action_msgs__srv__CancelGoal_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_msgs__srv__CancelGoal_Request() }
            }

            fn create_msg() -> *mut action_msgs__srv__CancelGoal_Request {

                unsafe { action_msgs__srv__CancelGoal_Request__create() }

            }

            fn destroy_msg(msg: *mut action_msgs__srv__CancelGoal_Request) -> () {

                unsafe { action_msgs__srv__CancelGoal_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
goal_info: action_msgs::msg::GoalInfo::from_native(&msg.goal_info),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.goal_info.copy_to_native(&mut msg.goal_info);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub return_code: i8,
pub goals_canceling: Vec<action_msgs::msg::GoalInfo>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = action_msgs__srv__CancelGoal_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_msgs__srv__CancelGoal_Response() }
            }

            fn create_msg() -> *mut action_msgs__srv__CancelGoal_Response {

                unsafe { action_msgs__srv__CancelGoal_Response__create() }

            }

            fn destroy_msg(msg: *mut action_msgs__srv__CancelGoal_Response) -> () {

                unsafe { action_msgs__srv__CancelGoal_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
return_code: msg.return_code,
// is_upper_bound_: false
// member.array_size_ : 0
goals_canceling : {
let mut temp = Vec::with_capacity(msg.goals_canceling.size);
let slice = unsafe { std::slice::from_raw_parts(msg.goals_canceling.data, msg.goals_canceling.size)};
for s in slice { temp.push(action_msgs::msg::GoalInfo::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.return_code = self.return_code;
unsafe { action_msgs__msg__GoalInfo__Sequence__fini(&mut msg.goals_canceling) };
unsafe { action_msgs__msg__GoalInfo__Sequence__init(&mut msg.goals_canceling, self.goals_canceling.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.goals_canceling.data, msg.goals_canceling.size)};
for (t,s) in slice.iter_mut().zip(&self.goals_canceling) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             

                          
                          #[allow(non_upper_case_globals)]
                          impl Response {
                                pub const ERROR_NONE: _bindgen_ty_8 = action_msgs__srv__CancelGoal_Response__ERROR_NONE;
  pub const ERROR_REJECTED: _bindgen_ty_9 = action_msgs__srv__CancelGoal_Response__ERROR_REJECTED;
  pub const ERROR_UNKNOWN_GOAL_ID: _bindgen_ty_10 = action_msgs__srv__CancelGoal_Response__ERROR_UNKNOWN_GOAL_ID;
  pub const ERROR_GOAL_TERMINATED: _bindgen_ty_11 = action_msgs__srv__CancelGoal_Response__ERROR_GOAL_TERMINATED;
                          }
             


                        }
  }
