  pub mod srv {
#[allow(non_snake_case)]
    pub mod Empty {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__Empty()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = std_srvs__srv__Empty_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__Empty_Request() }
            }

            fn create_msg() -> *mut std_srvs__srv__Empty_Request {

                unsafe { std_srvs__srv__Empty_Request__create() }

            }

            fn destroy_msg(msg: *mut std_srvs__srv__Empty_Request) -> () {

                unsafe { std_srvs__srv__Empty_Request__destroy(msg) };

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

                              
                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = std_srvs__srv__Empty_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__Empty_Response() }
            }

            fn create_msg() -> *mut std_srvs__srv__Empty_Response {

                unsafe { std_srvs__srv__Empty_Response__create() }

            }

            fn destroy_msg(msg: *mut std_srvs__srv__Empty_Response) -> () {

                unsafe { std_srvs__srv__Empty_Response__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Response {
  Response {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             

                          


                        }
#[allow(non_snake_case)]
    pub mod SetBool {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__SetBool()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub data: bool,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = std_srvs__srv__SetBool_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__SetBool_Request() }
            }

            fn create_msg() -> *mut std_srvs__srv__SetBool_Request {

                unsafe { std_srvs__srv__SetBool_Request__create() }

            }

            fn destroy_msg(msg: *mut std_srvs__srv__SetBool_Request) -> () {

                unsafe { std_srvs__srv__SetBool_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
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
pub message: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = std_srvs__srv__SetBool_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__SetBool_Response() }
            }

            fn create_msg() -> *mut std_srvs__srv__SetBool_Response {

                unsafe { std_srvs__srv__SetBool_Response__create() }

            }

            fn destroy_msg(msg: *mut std_srvs__srv__SetBool_Response) -> () {

                unsafe { std_srvs__srv__SetBool_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
message: msg.message.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
msg.message.assign(&self.message);
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
    pub mod Trigger {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__Trigger()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = std_srvs__srv__Trigger_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__Trigger_Request() }
            }

            fn create_msg() -> *mut std_srvs__srv__Trigger_Request {

                unsafe { std_srvs__srv__Trigger_Request__create() }

            }

            fn destroy_msg(msg: *mut std_srvs__srv__Trigger_Request) -> () {

                unsafe { std_srvs__srv__Trigger_Request__destroy(msg) };

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

                              pub success: bool,
pub message: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = std_srvs__srv__Trigger_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__Trigger_Response() }
            }

            fn create_msg() -> *mut std_srvs__srv__Trigger_Response {

                unsafe { std_srvs__srv__Trigger_Response__create() }

            }

            fn destroy_msg(msg: *mut std_srvs__srv__Trigger_Response) -> () {

                unsafe { std_srvs__srv__Trigger_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
message: msg.message.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
msg.message.assign(&self.message);
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
