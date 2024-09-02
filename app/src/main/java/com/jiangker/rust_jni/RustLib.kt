package com.jiangker.rust_jni

object RustLib {
    external fun writeLog(path: String, log: String)

    init {
        System.loadLibrary("rust_native")
    }
}