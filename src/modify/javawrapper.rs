use super::core;
use super::parser::file_ops::read_int_from_file;
use crate::modify::parser::nitf21::{NitfHeader21 as N, NitfHeader21::*};
use jni::objects::{JByteArray, JClass, JObject, JString};
use jni::sys::{jbyteArray, jint, jlong, jstring};
use jni::JNIEnv;
use std::fs::File;
use std::mem::ManuallyDrop;
#[cfg(unix)]
use std::os::unix::io::{FromRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{FromRawHandle, RawHandle};
#[cfg(windows)]
use winapi::um::handleapi::CloseHandle;

#[no_mangle]
pub extern "system" fn Java_nitfgnr_getVersion(env: JNIEnv, _class: JClass, fd: jlong) -> jstring {
    let file = get_java_file(fd);
    let (fhdr, ver) = core::get_version(&file);
    **env.new_string(fhdr+&ver).expect("Failed to create new string")
}

#[no_mangle]
pub extern "system" fn Java_nitfgnr_getHeaderLength(_env: JNIEnv, _class: JClass, fd: jlong) -> jint {
    let file = get_java_file(fd);
    read_int_from_file(&file, N::get_offset(HL, None), N::get_value(HL)) as jint
}

#[no_mangle]
pub extern "system" fn Java_nitfgnr_getNumImages(_env: JNIEnv, _class: JClass, fd: jlong) -> jint {
    let file = get_java_file(fd);
    read_int_from_file(&file, N::get_offset(NUMI, None), N::get_value(NUMI)) as jint
}

#[no_mangle]
pub extern "system" fn Java_nitfgnr_extractJp2Index(
    mut env: JNIEnv,
    _class: JClass,
    input_path: JString,
    index: jint,
) -> jbyteArray {
    let input_string: String = env.get_string(&input_path).expect("Couldn't get input path").into();
    let input_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(input_string)
        .expect("Failed to open file");
    ovu8_to_jbytearray(env, core::extract_jp2_index(&input_file, index as usize))
}

#[no_mangle]
pub extern "system" fn Java_nitfgnr_extractAllJp2(
    mut env: JNIEnv,
    _class: JClass,
    input_path: JString,
    output_path: JString
) {
    let input_string: String = env.get_string(&input_path).expect("Couldn't get input path").into();
    let output_string: String = env.get_string(&output_path).expect("Couldn't get input path").into();
    let input_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(input_string)
        .expect("Failed to open file");
    core::extract_jp2(&input_file, &output_string);
}

#[no_mangle]
pub extern "system" fn Java_nitfgnr_getNumDes(_env: JNIEnv, _class: JClass, fd: jlong) -> jint {
    let file = get_java_file(fd);
    read_int_from_file(&file, N::get_offset(NUMDES, Some(&file)), N::get_value(NUMDES)) as jint
}

#[no_mangle]
pub extern "system" fn Java_nitfgnr_extractDesHeader (
    env: JNIEnv,
    _class: JClass,
    fd: jlong,
    index: jint,
) -> jbyteArray {
    println!("Extracting DES Header {}", index);
    let file = get_java_file(fd);
    ovu8_to_jbytearray(env, core::extract_des_header_fields_index(&file, index as usize))
}

#[no_mangle]
pub extern "system" fn Java_nitfgnr_extractDes (
    env: JNIEnv,
    _class: JClass,
    fd: jlong,
    index: jint,
) -> jbyteArray {
    println!("Extracting DES {}", index);
    let file = get_java_file(fd);
    ovu8_to_jbytearray(env, core::extract_des_index(&file, index as usize))
}

#[no_mangle]
pub extern "system" fn Java_nitfgnr_copyDesSegmants(
    _env: JNIEnv,
    _class: JClass,
    input_fd: jlong,
    output_fd: jlong,
) {
    let mut input_file = get_java_file(input_fd);
    let mut output_file = get_java_file(output_fd);
    core::copy_des_segmants(&mut input_file, &mut output_file);
}

#[no_mangle]
pub extern "system" fn Java_nitfgnr_copyDesSegmantsFromPaths<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input_path: JString<'local>,
    output_path: JString<'local>,
) {

    let input_string: String = env.get_string(&input_path).expect("Couldn't get input path").into();
    let output_string: String = env.get_string(&output_path).expect("Couldn't get input path").into();
    let mut input_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(input_string)
        .expect("Failed to open file");
    let mut output_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(output_string)
        .expect("Failed to open file");
    core::copy_des_segmants(&mut input_file, &mut output_file);
}

#[no_mangle]
pub extern "system" fn Java_nitfgnr_addDesBytes (
    env: JNIEnv,
    _class: JClass,
    nitf_bytes: jbyteArray,
    des_header_bytes: jbyteArray,
    des_bytes: jbyteArray
) -> jbyteArray {
    println!("Adding DES bytes to NITF");
    let mut nitf_bytes = jbytearray_to_vec(&env, nitf_bytes);
    println!("NITF bytes conversion finished.");
    let des_header_bytes = jbytearray_to_vec(&env, des_header_bytes);
    println!("DES Header bytes conversion finished.");
    let des_bytes = jbytearray_to_vec(&env, des_bytes);
    println!("DES bytes conversion finished.");
    core::add_des_bytes(&mut nitf_bytes, &des_header_bytes, &des_bytes);
    println!("DES bytes added to NITF.");
    ovu8_to_jbytearray(env, Some(nitf_bytes))
}

//Private helper functions
fn get_java_file(fd: jlong) -> ManuallyDrop<File> {
    #[cfg(unix)]
    let file = unsafe { File::from_raw_fd(fd as RawFd) };
    #[cfg(windows)]
    let file = unsafe { File::from_raw_handle(fd as RawHandle) };
    ManuallyDrop::new(file)
}

fn get_java_file_owned(fd: jlong) -> File {
    #[cfg(unix)]
    unsafe { File::from_raw_fd(fd as RawFd) }
    #[cfg(windows)]
    unsafe { File::from_raw_handle(fd as RawHandle) }
}

fn get_java_file_from_path(env: &mut JNIEnv, path: jstring) -> File {
    let path = jstring_to_string(env, path);
    println!("Opening file: {}", path);
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("Failed to open file");
    file
}

/// Converts an `Option<Vec<u8>>` to a `Vec<i8>`, panics if None.
fn ovu8_to_vi8(vec: Option<Vec<u8>>) -> Vec<i8> {
    vec.expect("Vec was None!").iter().map(|&b| b as i8).collect()
}

/// Converts a `Vec<i8>` to a `jbyteArray`.
fn vi8_to_jbytearray(env: JNIEnv, vec: Vec<i8>) -> jbyteArray {
    let byte_array = env
        .new_byte_array(vec.len() as jint)
        .expect("Failed to create byte array");
    env.set_byte_array_region(&byte_array, 0, &vec)
        .expect("Failed to set byte array region");
    **byte_array
}

/// Converts an Option<Vec<u8>> to a jbyteArray, panics if None.
fn ovu8_to_jbytearray(env: JNIEnv, vec: Option<Vec<u8>>) -> jbyteArray {
    vi8_to_jbytearray(env, ovu8_to_vi8(vec))
}

/// Converts a `jbyteArray` to a `Vec<u8>`.
fn jbytearray_to_vec(env: &JNIEnv, array: jbyteArray) -> Vec<u8> {
    let obj: JObject = unsafe { JObject::from_raw(array) };
    let byte_array = JByteArray::from(obj);
    env.convert_byte_array(byte_array)
        .expect("Failed to convert byte array")
}

fn jstring_to_string(env:&mut JNIEnv, jstr: jstring) -> String {
    let jstr = unsafe {
        JString::from_raw(jstr)
    };
    let path = env.get_string(&jstr).expect("Failed to get string");
    path.to_str().expect("Failed to convert to str").to_string()
}

