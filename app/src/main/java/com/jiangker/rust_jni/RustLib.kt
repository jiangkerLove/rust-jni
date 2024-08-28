package com.jiangker.rust_jni

object RustLib {
    external fun stringFromJNI(): String

    init {
        System.loadLibrary("rust_native")
    }
}