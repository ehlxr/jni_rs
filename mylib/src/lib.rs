use jni::objects::{GlobalRef, JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jint, jlong, jstring};
use jni::JNIEnv;
use std::{sync::mpsc, thread, time::Duration};

#[no_mangle]
pub extern "system" fn Java_me_ehlxr_HelloWorld_getFiled(
    env: JNIEnv,
    _class: JClass,
    input: JObject,
) -> jstring {
    let map = env
        .get_field(input, "map", "Ljava/util/Map;")
        .unwrap()
        .l()
        .unwrap();
    let jmap = env.get_map(map).unwrap();
    let v1 = jmap
        .get(JObject::from(env.new_string("k1").unwrap()))
        .unwrap()
        .unwrap();
    println!("get map key k1, value: {}", long_value(env, v1));

    jmap.iter().unwrap().into_iter().for_each(|jmap_iter| {
        let key: JString = jmap_iter.0.into();
        let value = jmap_iter.1;
        println!(
            "get map key: {}, value: {}",
            String::from(env.get_string(key).unwrap()),
            long_value(env, value)
        );
    });

    let jlist = env
        .get_list(
            env.get_field(input, "ls", "Ljava/util/List;")
                .unwrap()
                .l()
                .unwrap(),
        )
        .unwrap();
    jlist.iter().unwrap().into_iter().for_each(|jobj| {
        let jstr: JString = jobj.into();
        println!(
            "get list filed: {}",
            String::from(env.get_string(jstr).unwrap())
        );
    });

    let age = env.get_field(input, "age", "I").unwrap().i().unwrap();
    println!("get age field: {}", age);

    let name: JString = env
        .get_field(input, "name", "Ljava/lang/String;")
        .unwrap()
        .l()
        .unwrap()
        .into();
    println!(
        "get name field: {}",
        String::from(env.get_string(name).unwrap())
    );

    // let no = env.get_field(input, "no", "J").unwrap().j().unwrap();
    // println!("get no field: {}", no);

    let no = long_value(
        env,
        env.get_field(input, "no", "Ljava/lang/Long;")
            .unwrap()
            .l()
            .unwrap(),
    );
    println!("get no field: {}", no);

    let out_str = if let JValue::Object(result) = env
        .call_method(input, "getName", "()Ljava/lang/String;", &[])
        .unwrap()
    {
        let jstr = env.get_string(JString::from(result)).unwrap();
        // println!("call getNameStr result: {}", String::from(jstr));
        String::from(jstr)
    } else {
        "".to_string()
    };

    let output = env
        .new_string(format!("Hello {}! from Rust..", out_str))
        .expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_me_ehlxr_HelloWorld_hello(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    let output = env
        .new_string(format!("Hello, {}! from Rust..", input))
        .expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_me_ehlxr_HelloWorld_helloByte(
    env: JNIEnv,
    _class: JClass,
    input: jbyteArray,
) -> jbyteArray {
    let _input = env.convert_byte_array(input).unwrap();

    let buf = [1; 2000];
    let output = env.byte_array_from_slice(&buf).unwrap();
    output
}

#[no_mangle]
pub extern "system" fn Java_me_ehlxr_HelloWorld_factAndCallMeBack(
    env: JNIEnv,
    _class: JClass,
    n: jint,
    callback: JObject,
) {
    let i = n as i32;
    let res: jint = (2..i + 1).product();

    env.call_method(callback, "factCallback", "(I)V", &[res.into()])
        .unwrap();
}

struct Counter {
    count: i32,
    callback: GlobalRef,
}

impl Counter {
    pub fn new(callback: GlobalRef) -> Counter {
        Counter {
            count: 0,
            callback: callback,
        }
    }

    pub fn increment(&mut self, env: JNIEnv) {
        self.count = self.count + 1;
        env.call_method(
            &self.callback,
            "counterCallback",
            "(I)V",
            &[self.count.into()],
        )
        .unwrap();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_me_ehlxr_HelloWorld_counterNew(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();
    let counter = Counter::new(global_ref);

    Box::into_raw(Box::new(counter)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_me_ehlxr_HelloWorld_counterIncrement(
    env: JNIEnv,
    _class: JClass,
    counter_ptr: jlong,
) {
    let counter = &mut *(counter_ptr as *mut Counter);

    counter.increment(env);
}

#[no_mangle]
pub unsafe extern "system" fn Java_me_ehlxr_HelloWorld_counterDestroy(
    _env: JNIEnv,
    _class: JClass,
    counter_ptr: jlong,
) {
    let _boxed_counter = Box::from_raw(counter_ptr as *mut Counter);
}

#[no_mangle]
pub extern "system" fn Java_me_ehlxr_HelloWorld_asyncComputation(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
    let jvm = env.get_java_vm().unwrap();

    let callback = env.new_global_ref(callback).unwrap();

    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        tx.send(()).unwrap();

        let env = jvm.attach_current_thread().unwrap();

        for i in 0..11 {
            let progress = (i * 10) as jint;
            env.call_method(&callback, "asyncCallback", "(I)V", &[progress.into()])
                .unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    rx.recv().unwrap();
}

fn long_value(env: JNIEnv, jobj: JObject) -> i64 {
    env.call_method(jobj, "longValue", "()J", &[])
        .unwrap()
        .j()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
