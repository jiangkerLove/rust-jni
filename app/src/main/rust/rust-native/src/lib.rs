extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JString};

#[no_mangle]
pub extern "C" fn Java_com_jiangker_rust_1jni_RustLib_stringFromJNI<'a>(
    env: JNIEnv<'a>,
    _class: JClass<'a>,
) -> JString<'a> {
    return env.new_string("Rust").unwrap();
}
