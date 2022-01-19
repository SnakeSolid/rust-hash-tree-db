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

macro_rules! illegal_argument {
    ($env:ident, $message:expr, $result:expr) => {{
        if let Err(error) = $env.throw_new(ILLEGAL_ARGUMENT, $message) {
            eprint!("{}", error);
        }

        return $result;
    }};
    ($env:ident, $predicate:expr, $message:expr, $result:expr) => {{
        if $predicate {
            if let Err(error) = $env.throw_new(ILLEGAL_ARGUMENT, $message) {
                eprint!("{}", error);
            }

            return $result;
        }
    }};
}

macro_rules! database {
    ($env:ident, $handle:ident, $result:expr) => {
        match unsafe { ($handle as *mut JavaDatabase).as_mut() } {
            Some(database) => database,
            None => illegal_argument!($env, "Invalid database handle", $result),
        }
    };
}

macro_rules! unwrap {
    ($env:ident, $expression:expr, $result:expr) => {
        match $expression {
            Ok(value) => value,
            Err(error) => {
                let message = format!("{}", error);

                if let Err(error) = $env.throw(message) {
                    eprint!("{}", error);
                }

                return $result;
            }
        }
    };
}

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
    illegal_argument!(
        env,
        partition.is_null(),
        "Parameter `partition` must not be null.",
        null_mut()
    );
    illegal_argument!(
        env,
        key.is_null(),
        "Parameter `key` must not be null.",
        null_mut()
    );

    let database = database!(env, handle, null_mut());
    let partition = unwrap!(env, env.convert_byte_array(partition), null_mut());
    let key = unwrap!(env, env.convert_byte_array(key), null_mut());
    let value = database.get(&partition, &key).expect("Failed to get data");

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
    illegal_argument!(
        env,
        partition.is_null(),
        "Parameter `partition` must not be null.",
        0
    );
    illegal_argument!(env, key.is_null(), "Parameter `key` must not be null.", 0);
    illegal_argument!(
        env,
        value.is_null(),
        "Parameter `value` must not be null.",
        0
    );

    let database = database!(env, handle, 0);
    let partition = unwrap!(env, env.convert_byte_array(partition), 0);
    let key = unwrap!(env, env.convert_byte_array(key), 0);
    let value = unwrap!(env, env.convert_byte_array(value), 0);

    database
        .put(partition, key, value)
        .expect("Failed to put data") as jboolean
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_contains(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    partition: jbyteArray,
    key: jbyteArray,
) -> jboolean {
    illegal_argument!(
        env,
        partition.is_null(),
        "Parameter `partition` must not be null.",
        0
    );
    illegal_argument!(env, key.is_null(), "Parameter `key` must not be null.", 0);

    let database = database!(env, handle, 0);
    let partition = unwrap!(env, env.convert_byte_array(partition), 0);
    let key = unwrap!(env, env.convert_byte_array(key), 0);

    database
        .contains(&partition, &key)
        .expect("Failed to check data") as jboolean
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_delete(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    partition: jbyteArray,
    key: jbyteArray,
) -> jboolean {
    illegal_argument!(
        env,
        partition.is_null(),
        "Parameter `partition` must not be null.",
        0
    );
    illegal_argument!(env, key.is_null(), "Parameter `key` must not be null.", 0);

    let database = database!(env, handle, 0);
    let partition = unwrap!(env, env.convert_byte_array(partition), 0);
    let key = unwrap!(env, env.convert_byte_array(key), 0);

    database
        .delete(&partition, &key)
        .expect("Failed to delete data") as jboolean
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
    illegal_argument!(
        env,
        partition.is_null(),
        "Parameter `partition` must not be null.",
        ()
    );
    illegal_argument!(
        env,
        key_first.is_null(),
        "Parameter `key_first` must not be null.",
        ()
    );
    illegal_argument!(
        env,
        key_last.is_null(),
        "Parameter `key_last` must not be null.",
        ()
    );
    illegal_argument!(
        env,
        callback.is_null(),
        "Parameter `callback` must not be null.",
        ()
    );

    let database = database!(env, handle, ());
    let partition = unwrap!(env, env.convert_byte_array(partition), ());
    let key_first = unwrap!(env, env.convert_byte_array(key_first), ());
    let key_last = unwrap!(env, env.convert_byte_array(key_last), ());
    let callback = JObject::from(callback);
    let method_accept = unwrap!(
        env,
        env.get_method_id(callback, METHOD_CALLBACL_ACCEPT, SIGNATURE_CALLBACL_ACCEPT),
        ()
    );

    database
        .range(&partition, &key_first, &key_last, |key, value| {
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
        .expect("Failed to select data range");
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_succ(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    partition: jbyteArray,
    key: jbyteArray,
) -> jobject {
    illegal_argument!(
        env,
        partition.is_null(),
        "Parameter `partition` must not be null.",
        null_mut()
    );
    illegal_argument!(
        env,
        key.is_null(),
        "Parameter `key` must not be null.",
        null_mut()
    );

    let database = database!(env, handle, null_mut());
    let partition = unwrap!(env, env.convert_byte_array(partition), null_mut());
    let key = unwrap!(env, env.convert_byte_array(key), null_mut());
    let constructor = unwrap!(
        env,
        env.get_method_id(CLASS_ENTRY, METHOD_ENTRY_INIT, SIGNATURE_ENTRY_INIT),
        null_mut()
    );

    if let Some((key, value)) = database
        .succ(&partition, &key)
        .expect("Failed to select successor key")
    {
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
    illegal_argument!(
        env,
        partition.is_null(),
        "Parameter `partition` must not be null.",
        null_mut()
    );
    illegal_argument!(
        env,
        key.is_null(),
        "Parameter `key` must not be null.",
        null_mut()
    );

    let database = database!(env, handle, null_mut());
    let partition = unwrap!(env, env.convert_byte_array(partition), null_mut());
    let key = unwrap!(env, env.convert_byte_array(key), null_mut());
    let constructor = unwrap!(
        env,
        env.get_method_id(CLASS_ENTRY, METHOD_ENTRY_INIT, SIGNATURE_ENTRY_INIT),
        null_mut()
    );

    if let Some((key, value)) = database
        .pred(&partition, &key)
        .expect("Failed to select predecessor key")
    {
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

    database.count().expect("Failed to count data") as jlong
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_save(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    let database = database!(env, handle, ());

    database.save().expect("Failed to save data");
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_load(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    let database = database!(env, handle, ());

    database.load().expect("Failed to load data");
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_destroy(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    drop(unsafe { Box::from_raw(handle as *mut JavaDatabase) });
}
