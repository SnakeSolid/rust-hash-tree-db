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

type JavaDatabase = Database<Vec<u8>, Vec<u8>, Vec<u8>>;

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_NativeDatabase_create(
    env: JNIEnv,
    _class: JClass,
    page_size: jint,
    n_pages: jint,
    storage_path: JString,
) -> jlong {
    let mut config = Config::default();

    match page_size {
        page_size if page_size > 0 => {
            config = config.set_max_page_size(page_size as usize);
        }
        page_size => {
            let message = format!(
                "Parameter `page_size` must be greater then zero, but {} found",
                page_size
            );
            let _ = env.throw_new("java/lang/IllegalArgumentException", message);

            return 0;
        }
    }

    match n_pages {
        0 => {
            config = config.set_max_pages(None);
        }
        n_pages if n_pages > 0 => {
            config = config.set_max_pages(Some(n_pages as usize));
        }
        n_pages => {
            let message = format!(
                "Parameter `n_pages` must be greater or equals to zero, but {} found",
                n_pages
            );
            let _ = env.throw_new("java/lang/IllegalArgumentException", message);

            return 0;
        }
    }

    if !storage_path.is_null() {
        config = config.set_storage_path(
            env.get_string(storage_path)
                .expect("Couldn't get java string!")
                .to_str()
                .expect("Failed to convert path to string"),
        );
    }

    let database: JavaDatabase = Database::new(config);

    Box::into_raw(Box::new(database)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_NativeDatabase_get(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    hash: jbyteArray,
    key: jbyteArray,
) -> jbyteArray {
    let database = unsafe {
        (handle as *mut JavaDatabase)
            .as_mut()
            .expect("Failed to unwrap handle")
    };
    let hash = env
        .convert_byte_array(hash)
        .expect("Failed to get hash data");
    let key = env.convert_byte_array(key).expect("Failed to get key data");
    let value = database.get(&hash, &key).expect("Failed to get data");

    match value {
        Some(value) => env
            .byte_array_from_slice(value)
            .expect("Failed to create new byte array"),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_NativeDatabase_put(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    hash: jbyteArray,
    key: jbyteArray,
    value: jbyteArray,
) -> jboolean {
    let database = unsafe {
        (handle as *mut JavaDatabase)
            .as_mut()
            .expect("Failed to unwrap handle")
    };
    let hash = env
        .convert_byte_array(hash)
        .expect("Failed to get hash data");
    let key = env.convert_byte_array(key).expect("Failed to get key data");
    let value = env
        .convert_byte_array(value)
        .expect("Failed to get value data");

    database.put(hash, key, value).expect("Failed to put data") as jboolean
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_NativeDatabase_contains(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    hash: jbyteArray,
    key: jbyteArray,
) -> jboolean {
    let database = unsafe {
        (handle as *mut JavaDatabase)
            .as_mut()
            .expect("Failed to unwrap handle")
    };
    let hash = env
        .convert_byte_array(hash)
        .expect("Failed to get hash data");
    let key = env.convert_byte_array(key).expect("Failed to get key data");

    database
        .contains(&hash, &key)
        .expect("Failed to check data") as jboolean
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_NativeDatabase_delete(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    hash: jbyteArray,
    key: jbyteArray,
) -> jboolean {
    let database = unsafe {
        (handle as *mut JavaDatabase)
            .as_mut()
            .expect("Failed to unwrap handle")
    };
    let hash = env
        .convert_byte_array(hash)
        .expect("Failed to get hash data");
    let key = env.convert_byte_array(key).expect("Failed to get key data");

    database.delete(&hash, &key).expect("Failed to delete data") as jboolean
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_NativeDatabase_count(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jlong {
    let database = unsafe {
        (handle as *mut JavaDatabase)
            .as_mut()
            .expect("Failed to unwrap handle")
    };

    database.count().expect("Failed to count data") as jlong
}

#[no_mangle]
pub extern "system" fn Java_ru_snake_htdb_NativeDatabase_destroy(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    drop(unsafe { Box::from_raw(handle as *mut JavaDatabase) });
}
