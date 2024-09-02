pub mod log_map;

extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use crate::log_map::{write_log};

#[no_mangle]
pub extern "C" fn Java_com_jiangker_rust_1jni_RustLib_writeLog(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
    log: JString,
) {
    let path_str: String = env.get_string(&path).unwrap().into();
    let log_str: String = env.get_string(&log).unwrap().into();
    write_log(path_str, log_str)
}
