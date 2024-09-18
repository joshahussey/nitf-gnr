#include <jni.h>
#include <string.h>
#include "native_lib.h"

// Declare the Rust function (same signature as in Rust)
extern void extract_jp2(const char *path, const char *outputPath);

JNIEXPORT void JNICALL Java_NativeLib_extractJp2(JNIEnv *env, jobject obj, jstring path, jstring outputPath) {
    // Convert Java strings (jstring) to C strings
    const char *input_cstr = (*env)->GetStringUTFChars(env, path, 0);
    const char *output_cstr = (*env)->GetStringUTFChars(env, outputPath, 0);

    // Call the Rust function via the C FFI
    extract_jp2(input_cstr, output_cstr);

    // Release the Java strings
    (*env)->ReleaseStringUTFChars(env, path, input_cstr);
    (*env)->ReleaseStringUTFChars(env, outputPath, output_cstr);
}

