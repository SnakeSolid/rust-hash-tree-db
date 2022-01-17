use htdb_sys::Config;
use htdb_sys::Database;
use jni::objects::JClass;
use jni::objects::JString;
use jni::sys::jboolean;
use jni::sys::jbyteArray;
use jni::sys::jint;
use jni::sys::jlong;
use jni::JNIEnv;
use std::ptr;

const ILLEGAL_ARGUMENT: &str = "java/lang/IllegalArgumentException";

macro_rules! illegal_argument {
    ($env:ident, $message:expr) => {{
        if let Err(error) = $env.throw_new(ILLEGAL_ARGUMENT, $message) {
            eprint!("{}", error);
        }

        return;
    }};
    ($env:ident, $message:expr, $result:expr) => {{
        if let Err(error) = $env.throw_new(ILLEGAL_ARGUMENT, $message) {
            eprint!("{}", error);
        }

        return $result;
    }};
}

macro_rules! database {
    ($env:ident, $handle:ident) => {
        match unsafe { ($handle as *mut JavaDatabase).as_mut() } {
            Some(database) => database,
            None => illegal_argument!($env, "Invalid database handle"),
        }
    };
    ($env:ident, $handle:ident, $result:expr) => {
        match unsafe { ($handle as *mut JavaDatabase).as_mut() } {
            Some(database) => database,
            None => illegal_argument!($env, "Invalid database handle", $result),
        }
    };
}

macro_rules! check {
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
        config = config.set_storage_path(check!(
            env,
            check!(env, env.get_string(storage_path), 0).to_str(),
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
    hash: jbyteArray,
    key: jbyteArray,
) -> jbyteArray {
    let database = database!(env, handle, ptr::null_mut());
    let hash = check!(env, env.convert_byte_array(hash), ptr::null_mut());
    let key = check!(env, env.convert_byte_array(key), ptr::null_mut());
    let value = database.get(&hash, &key).expect("Failed to get data");

    match value {
        Some(value) => env
            .byte_array_from_slice(value)
            .expect("Failed to create new byte array"),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_put(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    hash: jbyteArray,
    key: jbyteArray,
    value: jbyteArray,
) -> jboolean {
    let database = database!(env, handle, 0);
    let hash = check!(env, env.convert_byte_array(hash), 0);
    let key = check!(env, env.convert_byte_array(key), 0);
    let value = check!(env, env.convert_byte_array(value), 0);

    database.put(hash, key, value).expect("Failed to put data") as jboolean
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_contains(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    hash: jbyteArray,
    key: jbyteArray,
) -> jboolean {
    let database = database!(env, handle, 0);
    let hash = check!(env, env.convert_byte_array(hash), 0);
    let key = check!(env, env.convert_byte_array(key), 0);

    database
        .contains(&hash, &key)
        .expect("Failed to check data") as jboolean
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_delete(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    hash: jbyteArray,
    key: jbyteArray,
) -> jboolean {
    let database = database!(env, handle, 0);
    let hash = check!(env, env.convert_byte_array(hash), 0);
    let key = check!(env, env.convert_byte_array(key), 0);

    database.delete(&hash, &key).expect("Failed to delete data") as jboolean
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
    let database = database!(env, handle);

    database.save().expect("Failed to save data");
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_HTDBNative_load(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    let database = database!(env, handle);

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
