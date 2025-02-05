  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct BatteryState {

                              pub header: std_msgs::msg::Header,
pub voltage: f32,
pub temperature: f32,
pub current: f32,
pub charge: f32,
pub capacity: f32,
pub design_capacity: f32,
pub percentage: f32,
pub power_supply_status: u8,
pub power_supply_health: u8,
pub power_supply_technology: u8,
pub present: bool,
pub cell_voltage: Vec<f32>,
pub cell_temperature: Vec<f32>,
pub location: std::string::String,
pub serial_number: std::string::String,

                          }

                          impl WrappedTypesupport for BatteryState { 

            type CStruct = sensor_msgs__msg__BatteryState; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__BatteryState() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__BatteryState {

                unsafe { sensor_msgs__msg__BatteryState__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__BatteryState) -> () {

                unsafe { sensor_msgs__msg__BatteryState__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> BatteryState {
  BatteryState {
header: std_msgs::msg::Header::from_native(&msg.header),
voltage: msg.voltage,
temperature: msg.temperature,
current: msg.current,
charge: msg.charge,
capacity: msg.capacity,
design_capacity: msg.design_capacity,
percentage: msg.percentage,
power_supply_status: msg.power_supply_status,
power_supply_health: msg.power_supply_health,
power_supply_technology: msg.power_supply_technology,
present: msg.present,
// is_upper_bound_: false
// member.array_size_ : 0
cell_voltage: msg.cell_voltage.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
cell_temperature: msg.cell_temperature.to_vec(),
location: msg.location.to_str().to_owned(),
serial_number: msg.serial_number.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.voltage = self.voltage;
msg.temperature = self.temperature;
msg.current = self.current;
msg.charge = self.charge;
msg.capacity = self.capacity;
msg.design_capacity = self.design_capacity;
msg.percentage = self.percentage;
msg.power_supply_status = self.power_supply_status;
msg.power_supply_health = self.power_supply_health;
msg.power_supply_technology = self.power_supply_technology;
msg.present = self.present;
msg.cell_voltage.update(&self.cell_voltage);
msg.cell_temperature.update(&self.cell_temperature);
msg.location.assign(&self.location);
msg.serial_number.assign(&self.serial_number);
}



        }


                          
                          impl Default for BatteryState {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<BatteryState>::new();
                                  BatteryState::from_native(&msg_native)
                              }
                          }
             

                          
                          #[allow(non_upper_case_globals)]
                          impl BatteryState {
                                pub const POWER_SUPPLY_STATUS_UNKNOWN: _bindgen_ty_92 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_STATUS_UNKNOWN;
  pub const POWER_SUPPLY_STATUS_CHARGING: _bindgen_ty_93 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_STATUS_CHARGING;
  pub const POWER_SUPPLY_STATUS_DISCHARGING: _bindgen_ty_94 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_STATUS_DISCHARGING;
  pub const POWER_SUPPLY_STATUS_NOT_CHARGING: _bindgen_ty_95 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_STATUS_NOT_CHARGING;
  pub const POWER_SUPPLY_STATUS_FULL: _bindgen_ty_96 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_STATUS_FULL;
  pub const POWER_SUPPLY_HEALTH_UNKNOWN: _bindgen_ty_97 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_HEALTH_UNKNOWN;
  pub const POWER_SUPPLY_HEALTH_GOOD: _bindgen_ty_98 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_HEALTH_GOOD;
  pub const POWER_SUPPLY_HEALTH_OVERHEAT: _bindgen_ty_99 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_HEALTH_OVERHEAT;
  pub const POWER_SUPPLY_HEALTH_DEAD: _bindgen_ty_100 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_HEALTH_DEAD;
  pub const POWER_SUPPLY_HEALTH_OVERVOLTAGE: _bindgen_ty_101 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_HEALTH_OVERVOLTAGE;
  pub const POWER_SUPPLY_HEALTH_UNSPEC_FAILURE: _bindgen_ty_102 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_HEALTH_UNSPEC_FAILURE;
  pub const POWER_SUPPLY_HEALTH_COLD: _bindgen_ty_103 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_HEALTH_COLD;
  pub const POWER_SUPPLY_HEALTH_WATCHDOG_TIMER_EXPIRE: _bindgen_ty_104 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_HEALTH_WATCHDOG_TIMER_EXPIRE;
  pub const POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE: _bindgen_ty_105 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE;
  pub const POWER_SUPPLY_TECHNOLOGY_UNKNOWN: _bindgen_ty_106 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_TECHNOLOGY_UNKNOWN;
  pub const POWER_SUPPLY_TECHNOLOGY_NIMH: _bindgen_ty_107 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_TECHNOLOGY_NIMH;
  pub const POWER_SUPPLY_TECHNOLOGY_LION: _bindgen_ty_108 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_TECHNOLOGY_LION;
  pub const POWER_SUPPLY_TECHNOLOGY_LIPO: _bindgen_ty_109 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_TECHNOLOGY_LIPO;
  pub const POWER_SUPPLY_TECHNOLOGY_LIFE: _bindgen_ty_110 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_TECHNOLOGY_LIFE;
  pub const POWER_SUPPLY_TECHNOLOGY_NICD: _bindgen_ty_111 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_TECHNOLOGY_NICD;
  pub const POWER_SUPPLY_TECHNOLOGY_LIMN: _bindgen_ty_112 = sensor_msgs__msg__BatteryState__POWER_SUPPLY_TECHNOLOGY_LIMN;
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct CameraInfo {

                              pub header: std_msgs::msg::Header,
pub height: u32,
pub width: u32,
pub distortion_model: std::string::String,
pub d: Vec<f64>,
pub k: Vec<f64>,
pub r: Vec<f64>,
pub p: Vec<f64>,
pub binning_x: u32,
pub binning_y: u32,
pub roi: sensor_msgs::msg::RegionOfInterest,

                          }

                          impl WrappedTypesupport for CameraInfo { 

            type CStruct = sensor_msgs__msg__CameraInfo; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__CameraInfo() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__CameraInfo {

                unsafe { sensor_msgs__msg__CameraInfo__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__CameraInfo) -> () {

                unsafe { sensor_msgs__msg__CameraInfo__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> CameraInfo {
  CameraInfo {
header: std_msgs::msg::Header::from_native(&msg.header),
height: msg.height,
width: msg.width,
distortion_model: msg.distortion_model.to_str().to_owned(),
// is_upper_bound_: false
// member.array_size_ : 0
d: msg.d.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 9
k: msg.k.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 9
r: msg.r.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 12
p: msg.p.to_vec(),
binning_x: msg.binning_x,
binning_y: msg.binning_y,
roi: sensor_msgs::msg::RegionOfInterest::from_native(&msg.roi),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.height = self.height;
msg.width = self.width;
msg.distortion_model.assign(&self.distortion_model);
msg.d.update(&self.d);
assert_eq!(self.k.len(), 9, "Field {} is fixed size of {}!", "k", 9);
msg.k.copy_from_slice(&self.k[..9]);
assert_eq!(self.r.len(), 9, "Field {} is fixed size of {}!", "r", 9);
msg.r.copy_from_slice(&self.r[..9]);
assert_eq!(self.p.len(), 12, "Field {} is fixed size of {}!", "p", 12);
msg.p.copy_from_slice(&self.p[..12]);
msg.binning_x = self.binning_x;
msg.binning_y = self.binning_y;
self.roi.copy_to_native(&mut msg.roi);
}



        }


                          
                          impl Default for CameraInfo {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<CameraInfo>::new();
                                  CameraInfo::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ChannelFloat32 {

                              pub name: std::string::String,
pub values: Vec<f32>,

                          }

                          impl WrappedTypesupport for ChannelFloat32 { 

            type CStruct = sensor_msgs__msg__ChannelFloat32; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__ChannelFloat32() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__ChannelFloat32 {

                unsafe { sensor_msgs__msg__ChannelFloat32__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__ChannelFloat32) -> () {

                unsafe { sensor_msgs__msg__ChannelFloat32__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ChannelFloat32 {
  ChannelFloat32 {
name: msg.name.to_str().to_owned(),
// is_upper_bound_: false
// member.array_size_ : 0
values: msg.values.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.name.assign(&self.name);
msg.values.update(&self.values);
}



        }


                          
                          impl Default for ChannelFloat32 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ChannelFloat32>::new();
                                  ChannelFloat32::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct CompressedImage {

                              pub header: std_msgs::msg::Header,
pub format: std::string::String,
pub data: Vec<u8>,

                          }

                          impl WrappedTypesupport for CompressedImage { 

            type CStruct = sensor_msgs__msg__CompressedImage; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__CompressedImage() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__CompressedImage {

                unsafe { sensor_msgs__msg__CompressedImage__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__CompressedImage) -> () {

                unsafe { sensor_msgs__msg__CompressedImage__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> CompressedImage {
  CompressedImage {
header: std_msgs::msg::Header::from_native(&msg.header),
format: msg.format.to_str().to_owned(),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.format.assign(&self.format);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for CompressedImage {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<CompressedImage>::new();
                                  CompressedImage::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct FluidPressure {

                              pub header: std_msgs::msg::Header,
pub fluid_pressure: f64,
pub variance: f64,

                          }

                          impl WrappedTypesupport for FluidPressure { 

            type CStruct = sensor_msgs__msg__FluidPressure; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__FluidPressure() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__FluidPressure {

                unsafe { sensor_msgs__msg__FluidPressure__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__FluidPressure) -> () {

                unsafe { sensor_msgs__msg__FluidPressure__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> FluidPressure {
  FluidPressure {
header: std_msgs::msg::Header::from_native(&msg.header),
fluid_pressure: msg.fluid_pressure,
variance: msg.variance,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.fluid_pressure = self.fluid_pressure;
msg.variance = self.variance;
}



        }


                          
                          impl Default for FluidPressure {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<FluidPressure>::new();
                                  FluidPressure::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Illuminance {

                              pub header: std_msgs::msg::Header,
pub illuminance: f64,
pub variance: f64,

                          }

                          impl WrappedTypesupport for Illuminance { 

            type CStruct = sensor_msgs__msg__Illuminance; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Illuminance() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__Illuminance {

                unsafe { sensor_msgs__msg__Illuminance__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__Illuminance) -> () {

                unsafe { sensor_msgs__msg__Illuminance__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Illuminance {
  Illuminance {
header: std_msgs::msg::Header::from_native(&msg.header),
illuminance: msg.illuminance,
variance: msg.variance,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.illuminance = self.illuminance;
msg.variance = self.variance;
}



        }


                          
                          impl Default for Illuminance {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Illuminance>::new();
                                  Illuminance::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Image {

                              pub header: std_msgs::msg::Header,
pub height: u32,
pub width: u32,
pub encoding: std::string::String,
pub is_bigendian: u8,
pub step: u32,
pub data: Vec<u8>,

                          }

                          impl WrappedTypesupport for Image { 

            type CStruct = sensor_msgs__msg__Image; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Image() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__Image {

                unsafe { sensor_msgs__msg__Image__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__Image) -> () {

                unsafe { sensor_msgs__msg__Image__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Image {
  Image {
header: std_msgs::msg::Header::from_native(&msg.header),
height: msg.height,
width: msg.width,
encoding: msg.encoding.to_str().to_owned(),
is_bigendian: msg.is_bigendian,
step: msg.step,
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.height = self.height;
msg.width = self.width;
msg.encoding.assign(&self.encoding);
msg.is_bigendian = self.is_bigendian;
msg.step = self.step;
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Image {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Image>::new();
                                  Image::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Imu {

                              pub header: std_msgs::msg::Header,
pub orientation: geometry_msgs::msg::Quaternion,
pub orientation_covariance: Vec<f64>,
pub angular_velocity: geometry_msgs::msg::Vector3,
pub angular_velocity_covariance: Vec<f64>,
pub linear_acceleration: geometry_msgs::msg::Vector3,
pub linear_acceleration_covariance: Vec<f64>,

                          }

                          impl WrappedTypesupport for Imu { 

            type CStruct = sensor_msgs__msg__Imu; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Imu() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__Imu {

                unsafe { sensor_msgs__msg__Imu__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__Imu) -> () {

                unsafe { sensor_msgs__msg__Imu__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Imu {
  Imu {
header: std_msgs::msg::Header::from_native(&msg.header),
orientation: geometry_msgs::msg::Quaternion::from_native(&msg.orientation),
// is_upper_bound_: false
// member.array_size_ : 9
orientation_covariance: msg.orientation_covariance.to_vec(),
angular_velocity: geometry_msgs::msg::Vector3::from_native(&msg.angular_velocity),
// is_upper_bound_: false
// member.array_size_ : 9
angular_velocity_covariance: msg.angular_velocity_covariance.to_vec(),
linear_acceleration: geometry_msgs::msg::Vector3::from_native(&msg.linear_acceleration),
// is_upper_bound_: false
// member.array_size_ : 9
linear_acceleration_covariance: msg.linear_acceleration_covariance.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.orientation.copy_to_native(&mut msg.orientation);
assert_eq!(self.orientation_covariance.len(), 9, "Field {} is fixed size of {}!", "orientation_covariance", 9);
msg.orientation_covariance.copy_from_slice(&self.orientation_covariance[..9]);
self.angular_velocity.copy_to_native(&mut msg.angular_velocity);
assert_eq!(self.angular_velocity_covariance.len(), 9, "Field {} is fixed size of {}!", "angular_velocity_covariance", 9);
msg.angular_velocity_covariance.copy_from_slice(&self.angular_velocity_covariance[..9]);
self.linear_acceleration.copy_to_native(&mut msg.linear_acceleration);
assert_eq!(self.linear_acceleration_covariance.len(), 9, "Field {} is fixed size of {}!", "linear_acceleration_covariance", 9);
msg.linear_acceleration_covariance.copy_from_slice(&self.linear_acceleration_covariance[..9]);
}



        }


                          
                          impl Default for Imu {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Imu>::new();
                                  Imu::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct JointState {

                              pub header: std_msgs::msg::Header,
pub name: Vec<std::string::String>,
pub position: Vec<f64>,
pub velocity: Vec<f64>,
pub effort: Vec<f64>,

                          }

                          impl WrappedTypesupport for JointState { 

            type CStruct = sensor_msgs__msg__JointState; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__JointState() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__JointState {

                unsafe { sensor_msgs__msg__JointState__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__JointState) -> () {

                unsafe { sensor_msgs__msg__JointState__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> JointState {
  JointState {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
name: msg.name.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
position: msg.position.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
velocity: msg.velocity.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
effort: msg.effort.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.name.update(&self.name);
msg.position.update(&self.position);
msg.velocity.update(&self.velocity);
msg.effort.update(&self.effort);
}



        }


                          
                          impl Default for JointState {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<JointState>::new();
                                  JointState::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Joy {

                              pub header: std_msgs::msg::Header,
pub axes: Vec<f32>,
pub buttons: Vec<i32>,

                          }

                          impl WrappedTypesupport for Joy { 

            type CStruct = sensor_msgs__msg__Joy; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Joy() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__Joy {

                unsafe { sensor_msgs__msg__Joy__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__Joy) -> () {

                unsafe { sensor_msgs__msg__Joy__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Joy {
  Joy {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
axes: msg.axes.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
buttons: msg.buttons.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.axes.update(&self.axes);
msg.buttons.update(&self.buttons);
}



        }


                          
                          impl Default for Joy {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Joy>::new();
                                  Joy::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct JoyFeedback {

                              #[serde(rename = "type")]
pub type_: u8,
pub id: u8,
pub intensity: f32,

                          }

                          impl WrappedTypesupport for JoyFeedback { 

            type CStruct = sensor_msgs__msg__JoyFeedback; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__JoyFeedback() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__JoyFeedback {

                unsafe { sensor_msgs__msg__JoyFeedback__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__JoyFeedback) -> () {

                unsafe { sensor_msgs__msg__JoyFeedback__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> JoyFeedback {
  JoyFeedback {
type_: msg.type_,
id: msg.id,
intensity: msg.intensity,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_ = self.type_;
msg.id = self.id;
msg.intensity = self.intensity;
}



        }


                          
                          impl Default for JoyFeedback {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<JoyFeedback>::new();
                                  JoyFeedback::from_native(&msg_native)
                              }
                          }
             

                          
                          #[allow(non_upper_case_globals)]
                          impl JoyFeedback {
                                pub const TYPE_LED: _bindgen_ty_113 = sensor_msgs__msg__JoyFeedback__TYPE_LED;
  pub const TYPE_RUMBLE: _bindgen_ty_114 = sensor_msgs__msg__JoyFeedback__TYPE_RUMBLE;
  pub const TYPE_BUZZER: _bindgen_ty_115 = sensor_msgs__msg__JoyFeedback__TYPE_BUZZER;
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct JoyFeedbackArray {

                              pub array: Vec<sensor_msgs::msg::JoyFeedback>,

                          }

                          impl WrappedTypesupport for JoyFeedbackArray { 

            type CStruct = sensor_msgs__msg__JoyFeedbackArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__JoyFeedbackArray() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__JoyFeedbackArray {

                unsafe { sensor_msgs__msg__JoyFeedbackArray__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__JoyFeedbackArray) -> () {

                unsafe { sensor_msgs__msg__JoyFeedbackArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> JoyFeedbackArray {
  JoyFeedbackArray {
// is_upper_bound_: false
// member.array_size_ : 0
array : {
let mut temp = Vec::with_capacity(msg.array.size);
let slice = unsafe { std::slice::from_raw_parts(msg.array.data, msg.array.size)};
for s in slice { temp.push(sensor_msgs::msg::JoyFeedback::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { sensor_msgs__msg__JoyFeedback__Sequence__fini(&mut msg.array) };
unsafe { sensor_msgs__msg__JoyFeedback__Sequence__init(&mut msg.array, self.array.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.array.data, msg.array.size)};
for (t,s) in slice.iter_mut().zip(&self.array) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for JoyFeedbackArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<JoyFeedbackArray>::new();
                                  JoyFeedbackArray::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct LaserEcho {

                              pub echoes: Vec<f32>,

                          }

                          impl WrappedTypesupport for LaserEcho { 

            type CStruct = sensor_msgs__msg__LaserEcho; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__LaserEcho() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__LaserEcho {

                unsafe { sensor_msgs__msg__LaserEcho__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__LaserEcho) -> () {

                unsafe { sensor_msgs__msg__LaserEcho__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> LaserEcho {
  LaserEcho {
// is_upper_bound_: false
// member.array_size_ : 0
echoes: msg.echoes.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.echoes.update(&self.echoes);
}



        }


                          
                          impl Default for LaserEcho {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<LaserEcho>::new();
                                  LaserEcho::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct LaserScan {

                              pub header: std_msgs::msg::Header,
pub angle_min: f32,
pub angle_max: f32,
pub angle_increment: f32,
pub time_increment: f32,
pub scan_time: f32,
pub range_min: f32,
pub range_max: f32,
pub ranges: Vec<f32>,
pub intensities: Vec<f32>,

                          }

                          impl WrappedTypesupport for LaserScan { 

            type CStruct = sensor_msgs__msg__LaserScan; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__LaserScan() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__LaserScan {

                unsafe { sensor_msgs__msg__LaserScan__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__LaserScan) -> () {

                unsafe { sensor_msgs__msg__LaserScan__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> LaserScan {
  LaserScan {
header: std_msgs::msg::Header::from_native(&msg.header),
angle_min: msg.angle_min,
angle_max: msg.angle_max,
angle_increment: msg.angle_increment,
time_increment: msg.time_increment,
scan_time: msg.scan_time,
range_min: msg.range_min,
range_max: msg.range_max,
// is_upper_bound_: false
// member.array_size_ : 0
ranges: msg.ranges.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
intensities: msg.intensities.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.angle_min = self.angle_min;
msg.angle_max = self.angle_max;
msg.angle_increment = self.angle_increment;
msg.time_increment = self.time_increment;
msg.scan_time = self.scan_time;
msg.range_min = self.range_min;
msg.range_max = self.range_max;
msg.ranges.update(&self.ranges);
msg.intensities.update(&self.intensities);
}



        }


                          
                          impl Default for LaserScan {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<LaserScan>::new();
                                  LaserScan::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MagneticField {

                              pub header: std_msgs::msg::Header,
pub magnetic_field: geometry_msgs::msg::Vector3,
pub magnetic_field_covariance: Vec<f64>,

                          }

                          impl WrappedTypesupport for MagneticField { 

            type CStruct = sensor_msgs__msg__MagneticField; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MagneticField() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__MagneticField {

                unsafe { sensor_msgs__msg__MagneticField__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__MagneticField) -> () {

                unsafe { sensor_msgs__msg__MagneticField__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MagneticField {
  MagneticField {
header: std_msgs::msg::Header::from_native(&msg.header),
magnetic_field: geometry_msgs::msg::Vector3::from_native(&msg.magnetic_field),
// is_upper_bound_: false
// member.array_size_ : 9
magnetic_field_covariance: msg.magnetic_field_covariance.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.magnetic_field.copy_to_native(&mut msg.magnetic_field);
assert_eq!(self.magnetic_field_covariance.len(), 9, "Field {} is fixed size of {}!", "magnetic_field_covariance", 9);
msg.magnetic_field_covariance.copy_from_slice(&self.magnetic_field_covariance[..9]);
}



        }


                          
                          impl Default for MagneticField {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MagneticField>::new();
                                  MagneticField::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MultiDOFJointState {

                              pub header: std_msgs::msg::Header,
pub joint_names: Vec<std::string::String>,
pub transforms: Vec<geometry_msgs::msg::Transform>,
pub twist: Vec<geometry_msgs::msg::Twist>,
pub wrench: Vec<geometry_msgs::msg::Wrench>,

                          }

                          impl WrappedTypesupport for MultiDOFJointState { 

            type CStruct = sensor_msgs__msg__MultiDOFJointState; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MultiDOFJointState() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__MultiDOFJointState {

                unsafe { sensor_msgs__msg__MultiDOFJointState__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__MultiDOFJointState) -> () {

                unsafe { sensor_msgs__msg__MultiDOFJointState__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MultiDOFJointState {
  MultiDOFJointState {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
joint_names: msg.joint_names.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
transforms : {
let mut temp = Vec::with_capacity(msg.transforms.size);
let slice = unsafe { std::slice::from_raw_parts(msg.transforms.data, msg.transforms.size)};
for s in slice { temp.push(geometry_msgs::msg::Transform::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
twist : {
let mut temp = Vec::with_capacity(msg.twist.size);
let slice = unsafe { std::slice::from_raw_parts(msg.twist.data, msg.twist.size)};
for s in slice { temp.push(geometry_msgs::msg::Twist::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
wrench : {
let mut temp = Vec::with_capacity(msg.wrench.size);
let slice = unsafe { std::slice::from_raw_parts(msg.wrench.data, msg.wrench.size)};
for s in slice { temp.push(geometry_msgs::msg::Wrench::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.joint_names.update(&self.joint_names);
unsafe { geometry_msgs__msg__Transform__Sequence__fini(&mut msg.transforms) };
unsafe { geometry_msgs__msg__Transform__Sequence__init(&mut msg.transforms, self.transforms.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.transforms.data, msg.transforms.size)};
for (t,s) in slice.iter_mut().zip(&self.transforms) { s.copy_to_native(t);}
unsafe { geometry_msgs__msg__Twist__Sequence__fini(&mut msg.twist) };
unsafe { geometry_msgs__msg__Twist__Sequence__init(&mut msg.twist, self.twist.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.twist.data, msg.twist.size)};
for (t,s) in slice.iter_mut().zip(&self.twist) { s.copy_to_native(t);}
unsafe { geometry_msgs__msg__Wrench__Sequence__fini(&mut msg.wrench) };
unsafe { geometry_msgs__msg__Wrench__Sequence__init(&mut msg.wrench, self.wrench.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.wrench.data, msg.wrench.size)};
for (t,s) in slice.iter_mut().zip(&self.wrench) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for MultiDOFJointState {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MultiDOFJointState>::new();
                                  MultiDOFJointState::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MultiEchoLaserScan {

                              pub header: std_msgs::msg::Header,
pub angle_min: f32,
pub angle_max: f32,
pub angle_increment: f32,
pub time_increment: f32,
pub scan_time: f32,
pub range_min: f32,
pub range_max: f32,
pub ranges: Vec<sensor_msgs::msg::LaserEcho>,
pub intensities: Vec<sensor_msgs::msg::LaserEcho>,

                          }

                          impl WrappedTypesupport for MultiEchoLaserScan { 

            type CStruct = sensor_msgs__msg__MultiEchoLaserScan; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MultiEchoLaserScan() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__MultiEchoLaserScan {

                unsafe { sensor_msgs__msg__MultiEchoLaserScan__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__MultiEchoLaserScan) -> () {

                unsafe { sensor_msgs__msg__MultiEchoLaserScan__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MultiEchoLaserScan {
  MultiEchoLaserScan {
header: std_msgs::msg::Header::from_native(&msg.header),
angle_min: msg.angle_min,
angle_max: msg.angle_max,
angle_increment: msg.angle_increment,
time_increment: msg.time_increment,
scan_time: msg.scan_time,
range_min: msg.range_min,
range_max: msg.range_max,
// is_upper_bound_: false
// member.array_size_ : 0
ranges : {
let mut temp = Vec::with_capacity(msg.ranges.size);
let slice = unsafe { std::slice::from_raw_parts(msg.ranges.data, msg.ranges.size)};
for s in slice { temp.push(sensor_msgs::msg::LaserEcho::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
intensities : {
let mut temp = Vec::with_capacity(msg.intensities.size);
let slice = unsafe { std::slice::from_raw_parts(msg.intensities.data, msg.intensities.size)};
for s in slice { temp.push(sensor_msgs::msg::LaserEcho::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.angle_min = self.angle_min;
msg.angle_max = self.angle_max;
msg.angle_increment = self.angle_increment;
msg.time_increment = self.time_increment;
msg.scan_time = self.scan_time;
msg.range_min = self.range_min;
msg.range_max = self.range_max;
unsafe { sensor_msgs__msg__LaserEcho__Sequence__fini(&mut msg.ranges) };
unsafe { sensor_msgs__msg__LaserEcho__Sequence__init(&mut msg.ranges, self.ranges.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.ranges.data, msg.ranges.size)};
for (t,s) in slice.iter_mut().zip(&self.ranges) { s.copy_to_native(t);}
unsafe { sensor_msgs__msg__LaserEcho__Sequence__fini(&mut msg.intensities) };
unsafe { sensor_msgs__msg__LaserEcho__Sequence__init(&mut msg.intensities, self.intensities.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.intensities.data, msg.intensities.size)};
for (t,s) in slice.iter_mut().zip(&self.intensities) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for MultiEchoLaserScan {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MultiEchoLaserScan>::new();
                                  MultiEchoLaserScan::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct NavSatFix {

                              pub header: std_msgs::msg::Header,
pub status: sensor_msgs::msg::NavSatStatus,
pub latitude: f64,
pub longitude: f64,
pub altitude: f64,
pub position_covariance: Vec<f64>,
pub position_covariance_type: u8,

                          }

                          impl WrappedTypesupport for NavSatFix { 

            type CStruct = sensor_msgs__msg__NavSatFix; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__NavSatFix() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__NavSatFix {

                unsafe { sensor_msgs__msg__NavSatFix__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__NavSatFix) -> () {

                unsafe { sensor_msgs__msg__NavSatFix__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> NavSatFix {
  NavSatFix {
header: std_msgs::msg::Header::from_native(&msg.header),
status: sensor_msgs::msg::NavSatStatus::from_native(&msg.status),
latitude: msg.latitude,
longitude: msg.longitude,
altitude: msg.altitude,
// is_upper_bound_: false
// member.array_size_ : 9
position_covariance: msg.position_covariance.to_vec(),
position_covariance_type: msg.position_covariance_type,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.status.copy_to_native(&mut msg.status);
msg.latitude = self.latitude;
msg.longitude = self.longitude;
msg.altitude = self.altitude;
assert_eq!(self.position_covariance.len(), 9, "Field {} is fixed size of {}!", "position_covariance", 9);
msg.position_covariance.copy_from_slice(&self.position_covariance[..9]);
msg.position_covariance_type = self.position_covariance_type;
}



        }


                          
                          impl Default for NavSatFix {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<NavSatFix>::new();
                                  NavSatFix::from_native(&msg_native)
                              }
                          }
             

                          
                          #[allow(non_upper_case_globals)]
                          impl NavSatFix {
                                pub const COVARIANCE_TYPE_UNKNOWN: _bindgen_ty_116 = sensor_msgs__msg__NavSatFix__COVARIANCE_TYPE_UNKNOWN;
  pub const COVARIANCE_TYPE_APPROXIMATED: _bindgen_ty_117 = sensor_msgs__msg__NavSatFix__COVARIANCE_TYPE_APPROXIMATED;
  pub const COVARIANCE_TYPE_DIAGONAL_KNOWN: _bindgen_ty_118 = sensor_msgs__msg__NavSatFix__COVARIANCE_TYPE_DIAGONAL_KNOWN;
  pub const COVARIANCE_TYPE_KNOWN: _bindgen_ty_119 = sensor_msgs__msg__NavSatFix__COVARIANCE_TYPE_KNOWN;
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct NavSatStatus {

                              pub status: i8,
pub service: u16,

                          }

                          impl WrappedTypesupport for NavSatStatus { 

            type CStruct = sensor_msgs__msg__NavSatStatus; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__NavSatStatus() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__NavSatStatus {

                unsafe { sensor_msgs__msg__NavSatStatus__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__NavSatStatus) -> () {

                unsafe { sensor_msgs__msg__NavSatStatus__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> NavSatStatus {
  NavSatStatus {
status: msg.status,
service: msg.service,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.status = self.status;
msg.service = self.service;
}



        }


                          
                          impl Default for NavSatStatus {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<NavSatStatus>::new();
                                  NavSatStatus::from_native(&msg_native)
                              }
                          }
             

                          
                          #[allow(non_upper_case_globals)]
                          impl NavSatStatus {
                                pub const STATUS_NO_FIX: _bindgen_ty_120 = sensor_msgs__msg__NavSatStatus__STATUS_NO_FIX;
  pub const STATUS_FIX: _bindgen_ty_121 = sensor_msgs__msg__NavSatStatus__STATUS_FIX;
  pub const STATUS_SBAS_FIX: _bindgen_ty_122 = sensor_msgs__msg__NavSatStatus__STATUS_SBAS_FIX;
  pub const STATUS_GBAS_FIX: _bindgen_ty_123 = sensor_msgs__msg__NavSatStatus__STATUS_GBAS_FIX;
  pub const SERVICE_GPS: _bindgen_ty_124 = sensor_msgs__msg__NavSatStatus__SERVICE_GPS;
  pub const SERVICE_GLONASS: _bindgen_ty_125 = sensor_msgs__msg__NavSatStatus__SERVICE_GLONASS;
  pub const SERVICE_COMPASS: _bindgen_ty_126 = sensor_msgs__msg__NavSatStatus__SERVICE_COMPASS;
  pub const SERVICE_GALILEO: _bindgen_ty_127 = sensor_msgs__msg__NavSatStatus__SERVICE_GALILEO;
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PointCloud {

                              pub header: std_msgs::msg::Header,
pub points: Vec<geometry_msgs::msg::Point32>,
pub channels: Vec<sensor_msgs::msg::ChannelFloat32>,

                          }

                          impl WrappedTypesupport for PointCloud { 

            type CStruct = sensor_msgs__msg__PointCloud; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__PointCloud() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__PointCloud {

                unsafe { sensor_msgs__msg__PointCloud__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__PointCloud) -> () {

                unsafe { sensor_msgs__msg__PointCloud__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PointCloud {
  PointCloud {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
points : {
let mut temp = Vec::with_capacity(msg.points.size);
let slice = unsafe { std::slice::from_raw_parts(msg.points.data, msg.points.size)};
for s in slice { temp.push(geometry_msgs::msg::Point32::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
channels : {
let mut temp = Vec::with_capacity(msg.channels.size);
let slice = unsafe { std::slice::from_raw_parts(msg.channels.data, msg.channels.size)};
for s in slice { temp.push(sensor_msgs::msg::ChannelFloat32::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
unsafe { geometry_msgs__msg__Point32__Sequence__fini(&mut msg.points) };
unsafe { geometry_msgs__msg__Point32__Sequence__init(&mut msg.points, self.points.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.points.data, msg.points.size)};
for (t,s) in slice.iter_mut().zip(&self.points) { s.copy_to_native(t);}
unsafe { sensor_msgs__msg__ChannelFloat32__Sequence__fini(&mut msg.channels) };
unsafe { sensor_msgs__msg__ChannelFloat32__Sequence__init(&mut msg.channels, self.channels.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.channels.data, msg.channels.size)};
for (t,s) in slice.iter_mut().zip(&self.channels) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for PointCloud {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PointCloud>::new();
                                  PointCloud::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PointCloud2 {

                              pub header: std_msgs::msg::Header,
pub height: u32,
pub width: u32,
pub fields: Vec<sensor_msgs::msg::PointField>,
pub is_bigendian: bool,
pub point_step: u32,
pub row_step: u32,
pub data: Vec<u8>,
pub is_dense: bool,

                          }

                          impl WrappedTypesupport for PointCloud2 { 

            type CStruct = sensor_msgs__msg__PointCloud2; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__PointCloud2() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__PointCloud2 {

                unsafe { sensor_msgs__msg__PointCloud2__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__PointCloud2) -> () {

                unsafe { sensor_msgs__msg__PointCloud2__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PointCloud2 {
  PointCloud2 {
header: std_msgs::msg::Header::from_native(&msg.header),
height: msg.height,
width: msg.width,
// is_upper_bound_: false
// member.array_size_ : 0
fields : {
let mut temp = Vec::with_capacity(msg.fields.size);
let slice = unsafe { std::slice::from_raw_parts(msg.fields.data, msg.fields.size)};
for s in slice { temp.push(sensor_msgs::msg::PointField::from_native(s)); }
temp },
is_bigendian: msg.is_bigendian,
point_step: msg.point_step,
row_step: msg.row_step,
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
is_dense: msg.is_dense,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.height = self.height;
msg.width = self.width;
unsafe { sensor_msgs__msg__PointField__Sequence__fini(&mut msg.fields) };
unsafe { sensor_msgs__msg__PointField__Sequence__init(&mut msg.fields, self.fields.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.fields.data, msg.fields.size)};
for (t,s) in slice.iter_mut().zip(&self.fields) { s.copy_to_native(t);}
msg.is_bigendian = self.is_bigendian;
msg.point_step = self.point_step;
msg.row_step = self.row_step;
msg.data.update(&self.data);
msg.is_dense = self.is_dense;
}



        }


                          
                          impl Default for PointCloud2 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PointCloud2>::new();
                                  PointCloud2::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PointField {

                              pub name: std::string::String,
pub offset: u32,
pub datatype: u8,
pub count: u32,

                          }

                          impl WrappedTypesupport for PointField { 

            type CStruct = sensor_msgs__msg__PointField; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__PointField() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__PointField {

                unsafe { sensor_msgs__msg__PointField__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__PointField) -> () {

                unsafe { sensor_msgs__msg__PointField__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PointField {
  PointField {
name: msg.name.to_str().to_owned(),
offset: msg.offset,
datatype: msg.datatype,
count: msg.count,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.name.assign(&self.name);
msg.offset = self.offset;
msg.datatype = self.datatype;
msg.count = self.count;
}



        }


                          
                          impl Default for PointField {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PointField>::new();
                                  PointField::from_native(&msg_native)
                              }
                          }
             

                          
                          #[allow(non_upper_case_globals)]
                          impl PointField {
                                pub const INT8: _bindgen_ty_128 = sensor_msgs__msg__PointField__INT8;
  pub const UINT8: _bindgen_ty_129 = sensor_msgs__msg__PointField__UINT8;
  pub const INT16: _bindgen_ty_130 = sensor_msgs__msg__PointField__INT16;
  pub const UINT16: _bindgen_ty_131 = sensor_msgs__msg__PointField__UINT16;
  pub const INT32: _bindgen_ty_132 = sensor_msgs__msg__PointField__INT32;
  pub const UINT32: _bindgen_ty_133 = sensor_msgs__msg__PointField__UINT32;
  pub const FLOAT32: _bindgen_ty_134 = sensor_msgs__msg__PointField__FLOAT32;
  pub const FLOAT64: _bindgen_ty_135 = sensor_msgs__msg__PointField__FLOAT64;
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Range {

                              pub header: std_msgs::msg::Header,
pub radiation_type: u8,
pub field_of_view: f32,
pub min_range: f32,
pub max_range: f32,
pub range: f32,

                          }

                          impl WrappedTypesupport for Range { 

            type CStruct = sensor_msgs__msg__Range; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Range() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__Range {

                unsafe { sensor_msgs__msg__Range__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__Range) -> () {

                unsafe { sensor_msgs__msg__Range__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Range {
  Range {
header: std_msgs::msg::Header::from_native(&msg.header),
radiation_type: msg.radiation_type,
field_of_view: msg.field_of_view,
min_range: msg.min_range,
max_range: msg.max_range,
range: msg.range,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.radiation_type = self.radiation_type;
msg.field_of_view = self.field_of_view;
msg.min_range = self.min_range;
msg.max_range = self.max_range;
msg.range = self.range;
}



        }


                          
                          impl Default for Range {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Range>::new();
                                  Range::from_native(&msg_native)
                              }
                          }
             

                          
                          #[allow(non_upper_case_globals)]
                          impl Range {
                                pub const ULTRASOUND: _bindgen_ty_136 = sensor_msgs__msg__Range__ULTRASOUND;
  pub const INFRARED: _bindgen_ty_137 = sensor_msgs__msg__Range__INFRARED;
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct RegionOfInterest {

                              pub x_offset: u32,
pub y_offset: u32,
pub height: u32,
pub width: u32,
pub do_rectify: bool,

                          }

                          impl WrappedTypesupport for RegionOfInterest { 

            type CStruct = sensor_msgs__msg__RegionOfInterest; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__RegionOfInterest() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__RegionOfInterest {

                unsafe { sensor_msgs__msg__RegionOfInterest__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__RegionOfInterest) -> () {

                unsafe { sensor_msgs__msg__RegionOfInterest__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> RegionOfInterest {
  RegionOfInterest {
x_offset: msg.x_offset,
y_offset: msg.y_offset,
height: msg.height,
width: msg.width,
do_rectify: msg.do_rectify,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.x_offset = self.x_offset;
msg.y_offset = self.y_offset;
msg.height = self.height;
msg.width = self.width;
msg.do_rectify = self.do_rectify;
}



        }


                          
                          impl Default for RegionOfInterest {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<RegionOfInterest>::new();
                                  RegionOfInterest::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct RelativeHumidity {

                              pub header: std_msgs::msg::Header,
pub relative_humidity: f64,
pub variance: f64,

                          }

                          impl WrappedTypesupport for RelativeHumidity { 

            type CStruct = sensor_msgs__msg__RelativeHumidity; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__RelativeHumidity() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__RelativeHumidity {

                unsafe { sensor_msgs__msg__RelativeHumidity__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__RelativeHumidity) -> () {

                unsafe { sensor_msgs__msg__RelativeHumidity__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> RelativeHumidity {
  RelativeHumidity {
header: std_msgs::msg::Header::from_native(&msg.header),
relative_humidity: msg.relative_humidity,
variance: msg.variance,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.relative_humidity = self.relative_humidity;
msg.variance = self.variance;
}



        }


                          
                          impl Default for RelativeHumidity {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<RelativeHumidity>::new();
                                  RelativeHumidity::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Temperature {

                              pub header: std_msgs::msg::Header,
pub temperature: f64,
pub variance: f64,

                          }

                          impl WrappedTypesupport for Temperature { 

            type CStruct = sensor_msgs__msg__Temperature; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Temperature() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__Temperature {

                unsafe { sensor_msgs__msg__Temperature__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__Temperature) -> () {

                unsafe { sensor_msgs__msg__Temperature__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Temperature {
  Temperature {
header: std_msgs::msg::Header::from_native(&msg.header),
temperature: msg.temperature,
variance: msg.variance,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.temperature = self.temperature;
msg.variance = self.variance;
}



        }


                          
                          impl Default for Temperature {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Temperature>::new();
                                  Temperature::from_native(&msg_native)
                              }
                          }
             

                          


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct TimeReference {

                              pub header: std_msgs::msg::Header,
pub time_ref: builtin_interfaces::msg::Time,
pub source: std::string::String,

                          }

                          impl WrappedTypesupport for TimeReference { 

            type CStruct = sensor_msgs__msg__TimeReference; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__TimeReference() }
            }

            fn create_msg() -> *mut sensor_msgs__msg__TimeReference {

                unsafe { sensor_msgs__msg__TimeReference__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__msg__TimeReference) -> () {

                unsafe { sensor_msgs__msg__TimeReference__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> TimeReference {
  TimeReference {
header: std_msgs::msg::Header::from_native(&msg.header),
time_ref: builtin_interfaces::msg::Time::from_native(&msg.time_ref),
source: msg.source.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.time_ref.copy_to_native(&mut msg.time_ref);
msg.source.assign(&self.source);
}



        }


                          
                          impl Default for TimeReference {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<TimeReference>::new();
                                  TimeReference::from_native(&msg_native)
                              }
                          }
             

                          


                      }
  pub mod srv {
#[allow(non_snake_case)]
    pub mod SetCameraInfo {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__sensor_msgs__srv__SetCameraInfo()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub camera_info: sensor_msgs::msg::CameraInfo,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = sensor_msgs__srv__SetCameraInfo_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__srv__SetCameraInfo_Request() }
            }

            fn create_msg() -> *mut sensor_msgs__srv__SetCameraInfo_Request {

                unsafe { sensor_msgs__srv__SetCameraInfo_Request__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__srv__SetCameraInfo_Request) -> () {

                unsafe { sensor_msgs__srv__SetCameraInfo_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
camera_info: sensor_msgs::msg::CameraInfo::from_native(&msg.camera_info),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.camera_info.copy_to_native(&mut msg.camera_info);
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
pub status_message: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = sensor_msgs__srv__SetCameraInfo_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__srv__SetCameraInfo_Response() }
            }

            fn create_msg() -> *mut sensor_msgs__srv__SetCameraInfo_Response {

                unsafe { sensor_msgs__srv__SetCameraInfo_Response__create() }

            }

            fn destroy_msg(msg: *mut sensor_msgs__srv__SetCameraInfo_Response) -> () {

                unsafe { sensor_msgs__srv__SetCameraInfo_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
status_message: msg.status_message.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
msg.status_message.assign(&self.status_message);
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
