  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct State {

                              pub id: u8,
pub label: std::string::String,

                          }

                          impl WrappedTypesupport for State { 

            type CStruct = lifecycle_msgs__msg__State; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__msg__State() }
            }

            fn create_msg() -> *mut lifecycle_msgs__msg__State {

                unsafe { lifecycle_msgs__msg__State__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__msg__State) -> () {

                unsafe { lifecycle_msgs__msg__State__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> State {
  State {
id: msg.id,
label: msg.label.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.id = self.id;
msg.label.assign(&self.label);
}



        }


                          
                          impl Default for State {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<State>::new();
                                  State::from_native(&msg_native)
                              }
                          }
             

                          
                          #[allow(non_upper_case_globals)]
                          impl State {
                                pub const PRIMARY_STATE_UNKNOWN: _bindgen_ty_26 = lifecycle_msgs__msg__State__PRIMARY_STATE_UNKNOWN;
  pub const PRIMARY_STATE_UNCONFIGURED: _bindgen_ty_27 = lifecycle_msgs__msg__State__PRIMARY_STATE_UNCONFIGURED;
  pub const PRIMARY_STATE_INACTIVE: _bindgen_ty_28 = lifecycle_msgs__msg__State__PRIMARY_STATE_INACTIVE;
  pub const PRIMARY_STATE_ACTIVE: _bindgen_ty_29 = lifecycle_msgs__msg__State__PRIMARY_STATE_ACTIVE;
  pub const PRIMARY_STATE_FINALIZED: _bindgen_ty_30 = lifecycle_msgs__msg__State__PRIMARY_STATE_FINALIZED;
  pub const TRANSITION_STATE_CONFIGURING: _bindgen_ty_31 = lifecycle_msgs__msg__State__TRANSITION_STATE_CONFIGURING;
  pub const TRANSITION_STATE_CLEANINGUP: _bindgen_ty_32 = lifecycle_msgs__msg__State__TRANSITION_STATE_CLEANINGUP;
  pub const TRANSITION_STATE_SHUTTINGDOWN: _bindgen_ty_33 = lifecycle_msgs__msg__State__TRANSITION_STATE_SHUTTINGDOWN;
  pub const TRANSITION_STATE_ACTIVATING: _bindgen_ty_34 = lifecycle_msgs__msg__State__TRANSITION_STATE_ACTIVATING;
  pub const TRANSITION_STATE_DEACTIVATING: _bindgen_ty_35 = lifecycle_msgs__msg__State__TRANSITION_STATE_DEACTIVATING;
  pub const TRANSITION_STATE_ERRORPROCESSING: _bindgen_ty_36 = lifecycle_msgs__msg__State__TRANSITION_STATE_ERRORPROCESSING;
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Transition {

                              pub id: u8,
pub label: std::string::String,

                          }

                          impl WrappedTypesupport for Transition { 

            type CStruct = lifecycle_msgs__msg__Transition; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__msg__Transition() }
            }

            fn create_msg() -> *mut lifecycle_msgs__msg__Transition {

                unsafe { lifecycle_msgs__msg__Transition__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__msg__Transition) -> () {

                unsafe { lifecycle_msgs__msg__Transition__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Transition {
  Transition {
id: msg.id,
label: msg.label.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.id = self.id;
msg.label.assign(&self.label);
}



        }


                          
                          impl Default for Transition {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Transition>::new();
                                  Transition::from_native(&msg_native)
                              }
                          }
             

                          
                          #[allow(non_upper_case_globals)]
                          impl Transition {
                                pub const TRANSITION_CREATE: _bindgen_ty_37 = lifecycle_msgs__msg__Transition__TRANSITION_CREATE;
  pub const TRANSITION_CONFIGURE: _bindgen_ty_38 = lifecycle_msgs__msg__Transition__TRANSITION_CONFIGURE;
  pub const TRANSITION_CLEANUP: _bindgen_ty_39 = lifecycle_msgs__msg__Transition__TRANSITION_CLEANUP;
  pub const TRANSITION_ACTIVATE: _bindgen_ty_40 = lifecycle_msgs__msg__Transition__TRANSITION_ACTIVATE;
  pub const TRANSITION_DEACTIVATE: _bindgen_ty_41 = lifecycle_msgs__msg__Transition__TRANSITION_DEACTIVATE;
  pub const TRANSITION_UNCONFIGURED_SHUTDOWN: _bindgen_ty_42 = lifecycle_msgs__msg__Transition__TRANSITION_UNCONFIGURED_SHUTDOWN;
  pub const TRANSITION_INACTIVE_SHUTDOWN: _bindgen_ty_43 = lifecycle_msgs__msg__Transition__TRANSITION_INACTIVE_SHUTDOWN;
  pub const TRANSITION_ACTIVE_SHUTDOWN: _bindgen_ty_44 = lifecycle_msgs__msg__Transition__TRANSITION_ACTIVE_SHUTDOWN;
  pub const TRANSITION_DESTROY: _bindgen_ty_45 = lifecycle_msgs__msg__Transition__TRANSITION_DESTROY;
  pub const TRANSITION_ON_CONFIGURE_SUCCESS: _bindgen_ty_46 = lifecycle_msgs__msg__Transition__TRANSITION_ON_CONFIGURE_SUCCESS;
  pub const TRANSITION_ON_CONFIGURE_FAILURE: _bindgen_ty_47 = lifecycle_msgs__msg__Transition__TRANSITION_ON_CONFIGURE_FAILURE;
  pub const TRANSITION_ON_CONFIGURE_ERROR: _bindgen_ty_48 = lifecycle_msgs__msg__Transition__TRANSITION_ON_CONFIGURE_ERROR;
  pub const TRANSITION_ON_CLEANUP_SUCCESS: _bindgen_ty_49 = lifecycle_msgs__msg__Transition__TRANSITION_ON_CLEANUP_SUCCESS;
  pub const TRANSITION_ON_CLEANUP_FAILURE: _bindgen_ty_50 = lifecycle_msgs__msg__Transition__TRANSITION_ON_CLEANUP_FAILURE;
  pub const TRANSITION_ON_CLEANUP_ERROR: _bindgen_ty_51 = lifecycle_msgs__msg__Transition__TRANSITION_ON_CLEANUP_ERROR;
  pub const TRANSITION_ON_ACTIVATE_SUCCESS: _bindgen_ty_52 = lifecycle_msgs__msg__Transition__TRANSITION_ON_ACTIVATE_SUCCESS;
  pub const TRANSITION_ON_ACTIVATE_FAILURE: _bindgen_ty_53 = lifecycle_msgs__msg__Transition__TRANSITION_ON_ACTIVATE_FAILURE;
  pub const TRANSITION_ON_ACTIVATE_ERROR: _bindgen_ty_54 = lifecycle_msgs__msg__Transition__TRANSITION_ON_ACTIVATE_ERROR;
  pub const TRANSITION_ON_DEACTIVATE_SUCCESS: _bindgen_ty_55 = lifecycle_msgs__msg__Transition__TRANSITION_ON_DEACTIVATE_SUCCESS;
  pub const TRANSITION_ON_DEACTIVATE_FAILURE: _bindgen_ty_56 = lifecycle_msgs__msg__Transition__TRANSITION_ON_DEACTIVATE_FAILURE;
  pub const TRANSITION_ON_DEACTIVATE_ERROR: _bindgen_ty_57 = lifecycle_msgs__msg__Transition__TRANSITION_ON_DEACTIVATE_ERROR;
  pub const TRANSITION_ON_SHUTDOWN_SUCCESS: _bindgen_ty_58 = lifecycle_msgs__msg__Transition__TRANSITION_ON_SHUTDOWN_SUCCESS;
  pub const TRANSITION_ON_SHUTDOWN_FAILURE: _bindgen_ty_59 = lifecycle_msgs__msg__Transition__TRANSITION_ON_SHUTDOWN_FAILURE;
  pub const TRANSITION_ON_SHUTDOWN_ERROR: _bindgen_ty_60 = lifecycle_msgs__msg__Transition__TRANSITION_ON_SHUTDOWN_ERROR;
  pub const TRANSITION_ON_ERROR_SUCCESS: _bindgen_ty_61 = lifecycle_msgs__msg__Transition__TRANSITION_ON_ERROR_SUCCESS;
  pub const TRANSITION_ON_ERROR_FAILURE: _bindgen_ty_62 = lifecycle_msgs__msg__Transition__TRANSITION_ON_ERROR_FAILURE;
  pub const TRANSITION_ON_ERROR_ERROR: _bindgen_ty_63 = lifecycle_msgs__msg__Transition__TRANSITION_ON_ERROR_ERROR;
  pub const TRANSITION_CALLBACK_SUCCESS: _bindgen_ty_64 = lifecycle_msgs__msg__Transition__TRANSITION_CALLBACK_SUCCESS;
  pub const TRANSITION_CALLBACK_FAILURE: _bindgen_ty_65 = lifecycle_msgs__msg__Transition__TRANSITION_CALLBACK_FAILURE;
  pub const TRANSITION_CALLBACK_ERROR: _bindgen_ty_66 = lifecycle_msgs__msg__Transition__TRANSITION_CALLBACK_ERROR;
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct TransitionDescription {

                              pub transition: lifecycle_msgs::msg::Transition,
pub start_state: lifecycle_msgs::msg::State,
pub goal_state: lifecycle_msgs::msg::State,

                          }

                          impl WrappedTypesupport for TransitionDescription { 

            type CStruct = lifecycle_msgs__msg__TransitionDescription; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__msg__TransitionDescription() }
            }

            fn create_msg() -> *mut lifecycle_msgs__msg__TransitionDescription {

                unsafe { lifecycle_msgs__msg__TransitionDescription__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__msg__TransitionDescription) -> () {

                unsafe { lifecycle_msgs__msg__TransitionDescription__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> TransitionDescription {
  TransitionDescription {
transition: lifecycle_msgs::msg::Transition::from_native(&msg.transition),
start_state: lifecycle_msgs::msg::State::from_native(&msg.start_state),
goal_state: lifecycle_msgs::msg::State::from_native(&msg.goal_state),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.transition.copy_to_native(&mut msg.transition);
self.start_state.copy_to_native(&mut msg.start_state);
self.goal_state.copy_to_native(&mut msg.goal_state);
}



        }


                          
                          impl Default for TransitionDescription {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<TransitionDescription>::new();
                                  TransitionDescription::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct TransitionEvent {

                              pub timestamp: u64,
pub transition: lifecycle_msgs::msg::Transition,
pub start_state: lifecycle_msgs::msg::State,
pub goal_state: lifecycle_msgs::msg::State,

                          }

                          impl WrappedTypesupport for TransitionEvent { 

            type CStruct = lifecycle_msgs__msg__TransitionEvent; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__msg__TransitionEvent() }
            }

            fn create_msg() -> *mut lifecycle_msgs__msg__TransitionEvent {

                unsafe { lifecycle_msgs__msg__TransitionEvent__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__msg__TransitionEvent) -> () {

                unsafe { lifecycle_msgs__msg__TransitionEvent__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> TransitionEvent {
  TransitionEvent {
timestamp: msg.timestamp,
transition: lifecycle_msgs::msg::Transition::from_native(&msg.transition),
start_state: lifecycle_msgs::msg::State::from_native(&msg.start_state),
goal_state: lifecycle_msgs::msg::State::from_native(&msg.goal_state),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.timestamp = self.timestamp;
self.transition.copy_to_native(&mut msg.transition);
self.start_state.copy_to_native(&mut msg.start_state);
self.goal_state.copy_to_native(&mut msg.goal_state);
}



        }


                          
                          impl Default for TransitionEvent {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<TransitionEvent>::new();
                                  TransitionEvent::from_native(&msg_native)
                              }
                          }
             

                          


                      }
  pub mod srv {
#[allow(non_snake_case)]
    pub mod ChangeState {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__lifecycle_msgs__srv__ChangeState()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub transition: lifecycle_msgs::msg::Transition,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = lifecycle_msgs__srv__ChangeState_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__ChangeState_Request() }
            }

            fn create_msg() -> *mut lifecycle_msgs__srv__ChangeState_Request {

                unsafe { lifecycle_msgs__srv__ChangeState_Request__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__srv__ChangeState_Request) -> () {

                unsafe { lifecycle_msgs__srv__ChangeState_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
transition: lifecycle_msgs::msg::Transition::from_native(&msg.transition),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.transition.copy_to_native(&mut msg.transition);
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

                              pub success: bool,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = lifecycle_msgs__srv__ChangeState_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__ChangeState_Response() }
            }

            fn create_msg() -> *mut lifecycle_msgs__srv__ChangeState_Response {

                unsafe { lifecycle_msgs__srv__ChangeState_Response__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__srv__ChangeState_Response) -> () {

                unsafe { lifecycle_msgs__srv__ChangeState_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             

                          


                        }
#[allow(non_snake_case)]
    pub mod GetAvailableStates {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__lifecycle_msgs__srv__GetAvailableStates()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = lifecycle_msgs__srv__GetAvailableStates_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__GetAvailableStates_Request() }
            }

            fn create_msg() -> *mut lifecycle_msgs__srv__GetAvailableStates_Request {

                unsafe { lifecycle_msgs__srv__GetAvailableStates_Request__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__srv__GetAvailableStates_Request) -> () {

                unsafe { lifecycle_msgs__srv__GetAvailableStates_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub available_states: Vec<lifecycle_msgs::msg::State>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = lifecycle_msgs__srv__GetAvailableStates_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__GetAvailableStates_Response() }
            }

            fn create_msg() -> *mut lifecycle_msgs__srv__GetAvailableStates_Response {

                unsafe { lifecycle_msgs__srv__GetAvailableStates_Response__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__srv__GetAvailableStates_Response) -> () {

                unsafe { lifecycle_msgs__srv__GetAvailableStates_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
available_states : {
let mut temp = Vec::with_capacity(msg.available_states.size);
let slice = unsafe { std::slice::from_raw_parts(msg.available_states.data, msg.available_states.size)};
for s in slice { temp.push(lifecycle_msgs::msg::State::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { lifecycle_msgs__msg__State__Sequence__fini(&mut msg.available_states) };
unsafe { lifecycle_msgs__msg__State__Sequence__init(&mut msg.available_states, self.available_states.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.available_states.data, msg.available_states.size)};
for (t,s) in slice.iter_mut().zip(&self.available_states) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             

                          


                        }
#[allow(non_snake_case)]
    pub mod GetAvailableTransitions {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__lifecycle_msgs__srv__GetAvailableTransitions()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = lifecycle_msgs__srv__GetAvailableTransitions_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__GetAvailableTransitions_Request() }
            }

            fn create_msg() -> *mut lifecycle_msgs__srv__GetAvailableTransitions_Request {

                unsafe { lifecycle_msgs__srv__GetAvailableTransitions_Request__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__srv__GetAvailableTransitions_Request) -> () {

                unsafe { lifecycle_msgs__srv__GetAvailableTransitions_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub available_transitions: Vec<lifecycle_msgs::msg::TransitionDescription>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = lifecycle_msgs__srv__GetAvailableTransitions_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__GetAvailableTransitions_Response() }
            }

            fn create_msg() -> *mut lifecycle_msgs__srv__GetAvailableTransitions_Response {

                unsafe { lifecycle_msgs__srv__GetAvailableTransitions_Response__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__srv__GetAvailableTransitions_Response) -> () {

                unsafe { lifecycle_msgs__srv__GetAvailableTransitions_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
available_transitions : {
let mut temp = Vec::with_capacity(msg.available_transitions.size);
let slice = unsafe { std::slice::from_raw_parts(msg.available_transitions.data, msg.available_transitions.size)};
for s in slice { temp.push(lifecycle_msgs::msg::TransitionDescription::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { lifecycle_msgs__msg__TransitionDescription__Sequence__fini(&mut msg.available_transitions) };
unsafe { lifecycle_msgs__msg__TransitionDescription__Sequence__init(&mut msg.available_transitions, self.available_transitions.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.available_transitions.data, msg.available_transitions.size)};
for (t,s) in slice.iter_mut().zip(&self.available_transitions) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             

                          


                        }
#[allow(non_snake_case)]
    pub mod GetState {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__lifecycle_msgs__srv__GetState()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = lifecycle_msgs__srv__GetState_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__GetState_Request() }
            }

            fn create_msg() -> *mut lifecycle_msgs__srv__GetState_Request {

                unsafe { lifecycle_msgs__srv__GetState_Request__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__srv__GetState_Request) -> () {

                unsafe { lifecycle_msgs__srv__GetState_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub current_state: lifecycle_msgs::msg::State,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = lifecycle_msgs__srv__GetState_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__GetState_Response() }
            }

            fn create_msg() -> *mut lifecycle_msgs__srv__GetState_Response {

                unsafe { lifecycle_msgs__srv__GetState_Response__create() }

            }

            fn destroy_msg(msg: *mut lifecycle_msgs__srv__GetState_Response) -> () {

                unsafe { lifecycle_msgs__srv__GetState_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
current_state: lifecycle_msgs::msg::State::from_native(&msg.current_state),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.current_state.copy_to_native(&mut msg.current_state);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             

                          


                        }
  }
