#[macro_use]
mod util;

use htdb_sys::Config;
use htdb_sys::Database;
use jni::objects::JClass;
use jni::objects::JObject;
use jni::objects::JString;
use jni::objects::JValue;
use jni::signature::JavaType;
use jni::signature::Primitive;
use jni::sys::jboolean;
use jni::sys::jbyteArray;
use jni::sys::jint;
use jni::sys::jlong;
use jni::sys::jobject;
use jni::JNIEnv;
use std::ptr::null_mut;

const ILLEGAL_ARGUMENT: &str = "java/lang/IllegalArgumentException";
const CLASS_ENTRY: &str = "ru/snake/htdb/entry/RawEntry";
const METHOD_ENTRY_INIT: &str = "<init>";
const METHOD_CALLBACL_ACCEPT: &str = "accept";
const SIGNATURE_ENTRY_INIT: &str = "([B[B)V";
const SIGNATURE_CALLBACL_ACCEPT: &str = "([B[B)Z";

type JavaDatabase = Database<Vec<u8>, Vec<u8>, Vec<u8>>;

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_create(
    env: JNIEnv,
    _class: JClass,
    page_size: jint,
    n_pages: jint,
    storage_path: JString,
) -> jlong {
    let mut config = Config::default();

    match page_size {
        page_size if page_size > 0 => config = config.set_max_page_size(page_size as usize),
        _ => illegal_argument!(env, "`page_size` must be greater then zero.", 0),
    }

    match n_pages {
        0 => config = config.set_max_pages(None),
        n_pages if n_pages > 0 => config = config.set_max_pages(Some(n_pages as usize)),
        _ => illegal_argument!(env, "`n_pages` must be greater or equals to zero.", 0),
    }

    if !storage_path.is_null() {
        config = config.set_storage_path(unwrap!(
            env,
            unwrap!(env, env.get_string(storage_path), 0).to_str(),
            0
        ));
    }

    let database: JavaDatabase = Database::new(config);

    Box::into_raw(Box::new(database)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_get(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    partition: jbyteArray,
    key: jbyteArray,
) -> jbyteArray {
    if partition.is_null() {
        illegal_argument!(env, "Parameter `partition` must not be null.", null_mut());
    }

    if key.is_null() {
        illegal_argument!(env, "Parameter `key` must not be null.", null_mut());
    }

    let database = database!(env, handle, null_mut());
    let partition = unwrap!(env, env.convert_byte_array(partition), null_mut());
    let key = unwrap!(env, env.convert_byte_array(key), null_mut());
    let value = unwrap!(env, database.get(&partition, &key), null_mut());

    match value {
        Some(value) => unwrap!(env, env.byte_array_from_slice(value), null_mut()),
        None => null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_put(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    partition: jbyteArray,
    key: jbyteArray,
    value: jbyteArray,
) -> jboolean {
    if partition.is_null() {
        illegal_argument!(env, "Parameter `partition` must not be null.", 0);
    }

    if key.is_null() {
        illegal_argument!(env, "Parameter `key` must not be null.", 0);
    }

    if value.is_null() {
        illegal_argument!(env, "Parameter `value` must not be null.", 0);
    }

    let database = database!(env, handle, 0);
    let partition = unwrap!(env, env.convert_byte_array(partition), 0);
    let key = unwrap!(env, env.convert_byte_array(key), 0);
    let value = unwrap!(env, env.convert_byte_array(value), 0);

    unwrap!(env, database.put(partition, key, value), 0) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_contains(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    partition: jbyteArray,
    key: jbyteArray,
) -> jboolean {
    if partition.is_null() {
        illegal_argument!(env, "Parameter `partition` must not be null.", 0);
    }

    if key.is_null() {
        illegal_argument!(env, "Parameter `key` must not be null.", 0);
    }

    let database = database!(env, handle, 0);
    let partition = unwrap!(env, env.convert_byte_array(partition), 0);
    let key = unwrap!(env, env.convert_byte_array(key), 0);

    unwrap!(env, database.contains(&partition, &key), 0) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_delete(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    partition: jbyteArray,
    key: jbyteArray,
) -> jboolean {
    if partition.is_null() {
        illegal_argument!(env, "Parameter `partition` must not be null.", 0);
    }

    if key.is_null() {
        illegal_argument!(env, "Parameter `key` must not be null.", 0);
    }

    let database = database!(env, handle, 0);
    let partition = unwrap!(env, env.convert_byte_array(partition), 0);
    let key = unwrap!(env, env.convert_byte_array(key), 0);

    unwrap!(env, database.delete(&partition, &key), 0) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_range(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    partition: jbyteArray,
    key_first: jbyteArray,
    key_last: jbyteArray,
    callback: jobject,
) {
    if partition.is_null() {
        illegal_argument!(env, "Parameter `partition` must not be null.");
    }

    if key_first.is_null() {
        illegal_argument!(env, "Parameter `key_first` must not be null.");
    }

    if key_last.is_null() {
        illegal_argument!(env, "Parameter `key_last` must not be null.");
    }

    if callback.is_null() {
        illegal_argument!(env, "Parameter `callback` must not be null.");
    }

    let database = database!(env, handle);
    let partition = unwrap!(env, env.convert_byte_array(partition));
    let key_first = unwrap!(env, env.convert_byte_array(key_first));
    let key_last = unwrap!(env, env.convert_byte_array(key_last));
    let callback = JObject::from(callback);
    let method_accept = unwrap!(
        env,
        env.get_method_id(callback, METHOD_CALLBACL_ACCEPT, SIGNATURE_CALLBACL_ACCEPT)
    );

    unwrap!(
        env,
        database.range(&partition, &key_first, &key_last, |key, value| {
            let key = unwrap!(env, env.byte_array_from_slice(key), false);
            let value = unwrap!(env, env.byte_array_from_slice(value), false);
            let result = unwrap!(
                env,
                env.call_method_unchecked(
                    callback,
                    method_accept,
                    JavaType::Primitive(Primitive::Boolean),
                    &[JValue::from(key), JValue::from(value)]
                ),
                false
            );

            !unwrap!(env, env.exception_check(), false) && unwrap!(env, result.z(), false)
        })
    );
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_succ(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    partition: jbyteArray,
    key: jbyteArray,
) -> jobject {
    if partition.is_null() {
        illegal_argument!(env, "Parameter `partition` must not be null.", null_mut());
    }

    if key.is_null() {
        illegal_argument!(env, "Parameter `key` must not be null.", null_mut());
    }

    let database = database!(env, handle, null_mut());
    let partition = unwrap!(env, env.convert_byte_array(partition), null_mut());
    let key = unwrap!(env, env.convert_byte_array(key), null_mut());
    let constructor = unwrap!(
        env,
        env.get_method_id(CLASS_ENTRY, METHOD_ENTRY_INIT, SIGNATURE_ENTRY_INIT),
        null_mut()
    );

    if let Some((key, value)) = unwrap!(env, database.succ(&partition, &key), null_mut()) {
        let key = unwrap!(env, env.byte_array_from_slice(key), null_mut());
        let value = unwrap!(env, env.byte_array_from_slice(value), null_mut());
        let result = unwrap!(
            env,
            env.new_object_unchecked(
                CLASS_ENTRY,
                constructor,
                &[JValue::from(key), JValue::from(value)]
            ),
            null_mut()
        );

        result.into_inner()
    } else {
        null_mut()
    }
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_pred(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    partition: jbyteArray,
    key: jbyteArray,
) -> jobject {
    if partition.is_null() {
        illegal_argument!(env, "Parameter `partition` must not be null.", null_mut());
    }

    if key.is_null() {
        illegal_argument!(env, "Parameter `key` must not be null.", null_mut());
    }

    let database = database!(env, handle, null_mut());
    let partition = unwrap!(env, env.convert_byte_array(partition), null_mut());
    let key = unwrap!(env, env.convert_byte_array(key), null_mut());
    let constructor = unwrap!(
        env,
        env.get_method_id(CLASS_ENTRY, METHOD_ENTRY_INIT, SIGNATURE_ENTRY_INIT),
        null_mut()
    );

    if let Some((key, value)) = unwrap!(env, database.pred(&partition, &key), null_mut()) {
        let key = unwrap!(env, env.byte_array_from_slice(key), null_mut());
        let value = unwrap!(env, env.byte_array_from_slice(value), null_mut());
        let result = unwrap!(
            env,
            env.new_object_unchecked(
                CLASS_ENTRY,
                constructor,
                &[JValue::from(key), JValue::from(value)]
            ),
            null_mut()
        );

        result.into_inner()
    } else {
        null_mut()
    }
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_count(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jlong {
    let database = database!(env, handle, 0);

    unwrap!(env, database.count(), 0) as jlong
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_save(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    let database = database!(env, handle);

    unwrap!(env, database.save());
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_load(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    let database = database!(env, handle);

    unwrap!(env, database.load());
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_destroy(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    drop(unsafe { Box::from_raw(handle as *mut JavaDatabase) });
}
