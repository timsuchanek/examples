#[macro_use]
extern crate neon;

use neon::prelude::*;

fn convert_vec_to_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    let vec: Vec<String> = Vec::with_capacity(100);

    // Create the JS array
    let js_array = JsArray::new(&mut cx, vec.len() as u32);

    // Iterate over the rust Vec and map each value in the Vec to the JS array
    vec.iter().enumerate().for_each(|e| {
        let (i, obj) = e;
        let js_string = cx.string(obj);
        let _ = js_array.set(&mut cx, i as u32, js_string);
    });

    Ok(js_array)
}

fn convert_js_array_to_vec(mut cx: FunctionContext) -> JsResult<JsNumber> {
    // Take the first argument, which must be an array
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    // Convert a JsArray to a Rust Vec
    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    // Return the length of the Vec to JS
    Ok(cx.number(vec.len() as f64))
}

fn return_empty_js_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    Ok(cx.empty_array())
}

fn return_js_array_with_number(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array: Handle<JsArray> = JsArray::new(&mut cx, 1);
    let n = cx.number(9000.0);
    array.set(&mut cx, 0, n)?;
    Ok(array)
}

fn return_js_array_with_string(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array: Handle<JsArray> = JsArray::new(&mut cx, 1);
    let s = cx.string("hello node");
    array.set(&mut cx, 0, s)?;
    Ok(array)
}

struct User {
    pub name: String,
    pub email: String,
    pub age: u64,
}

fn get_array_with_object(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array: Handle<JsArray> = JsArray::new(&mut cx, 1);

    let user = User {
        name: "Hans".to_string(),
        email: "a@a.de".to_string(),
        age: 30,
    };

    let object = JsObject::new(&mut cx);
    let name = cx.string(&user.name);
    let email = cx.string(&user.email);
    let age = cx.number(user.age as f64);

    object.set(&mut cx, "name", name).unwrap();
    object.set(&mut cx, "email", email).unwrap();
    object.set(&mut cx, "age", age).unwrap();

    array.set(&mut cx, 0, object).unwrap();

    Ok(array)
}

register_module!(mut m, {
    m.export_function("convertVecToArray", convert_vec_to_array)?;
    m.export_function("convertJsArrayToVec", convert_js_array_to_vec)?;
    m.export_function("returnJsArray", return_empty_js_array)?;
    m.export_function("returnJsArrayWithNumber", return_js_array_with_number)?;
    m.export_function("returnJsArrayWithString", return_js_array_with_string)?;
    m.export_function("getArrayWithObject", get_array_with_object)?;
    Ok(())
});
