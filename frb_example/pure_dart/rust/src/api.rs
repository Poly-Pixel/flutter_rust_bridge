#![allow(unused_variables)]

use flutter_rust_bridge::ZeroCopyBuffer;

use anyhow::{anyhow, Result};

pub fn simple_adder(a: i32, b: i32) -> Result<i32> {
    Ok(a + b)
}

pub fn primitive_types(my_i32: i32, my_i64: i64, my_f64: f64, my_bool: bool) -> Result<i32> {
    println!(
        "primitive_types({}, {}, {}, {})",
        my_i32, my_i64, my_f64, my_bool
    );
    Ok(42)
}

pub fn handle_string(s: String) -> Result<String> {
    println!("handle_string({})", &s);
    let s2 = s.clone();
    Ok(s + &s2)
}

pub fn handle_vec_u8(v: Vec<u8>) -> Result<Vec<u8>> {
    println!("handle_vec_u8(first few elements: {:?})", &v[..5]);
    Ok(v.repeat(2))
}

pub fn handle_zero_copy_result(n: i32) -> Result<ZeroCopyBuffer<Vec<u8>>> {
    println!("handle_zero_copy_result(n: {})", n);
    Ok(ZeroCopyBuffer(vec![42u8; n as usize]))
}

#[derive(Debug, Clone)]
pub struct MySize {
    pub width: i32,
    pub height: i32,
}

pub fn handle_struct(arg: MySize, boxed: Box<MySize>) -> Result<MySize> {
    println!("handle_struct({:?}, {:?})", &arg, &boxed);
    Ok(MySize {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    })
}

#[derive(Debug)]
pub struct NewTypeInt(pub i64);

pub fn handle_newtype(arg: NewTypeInt) -> Result<NewTypeInt> {
    println!("handle_newtype({:?})", &arg);
    Ok(NewTypeInt(arg.0 * 2))
}

pub fn handle_list_of_struct(mut l: Vec<MySize>) -> Result<Vec<MySize>> {
    println!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    Ok(ans)
}

#[derive(Debug, Clone)]
pub struct MyTreeNode {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub children: Vec<MyTreeNode>,
}

pub fn handle_complex_struct(s: MyTreeNode) -> Result<MyTreeNode> {
    println!("handle_complex_struct({:?})", &s);
    let s_cloned = s.clone();
    Ok(s)
}

pub fn return_err() -> Result<i32> {
    Err(anyhow!(
        "return_err() is called, thus deliberately return Err"
    ))
}

pub fn return_panic() -> Result<i32> {
    panic!("return_panic() is called, thus deliberately panic")
}

pub fn handle_optional_return(left: f64, right: f64) -> Result<Option<f64>> {
    if right == 0. {
        Ok(None)
    } else {
        Ok(Some(left / right))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Element {
    pub tag: Option<String>,
    pub text: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub children: Option<Vec<Element>>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

pub fn handle_optional_struct(document: Option<String>) -> Result<Option<Element>> {
    Ok(document.map(|inner| Element {
        tag: Some("div".to_owned()),
        attributes: Some(vec![Attribute {
            key: "id".to_owned(),
            value: "root".to_owned(),
        }]),
        children: Some(vec![Element {
            tag: Some("p".to_owned()),
            children: Some(vec![Element {
                text: Some(inner),
                ..Default::default()
            }]),
            ..Default::default()
        }]),
        ..Default::default()
    }))
}

#[derive(Default)]
pub struct ExoticOptionals {
    pub int32: Option<i32>,
    pub int64: Option<i64>,
    pub float64: Option<f64>,
    pub boolean: Option<bool>,
    pub zerocopy: Option<ZeroCopyBuffer<Vec<u8>>>,
    pub int8list: Option<Vec<i8>>,
    pub uint8list: Option<Vec<u8>>,
    pub float64list: Option<Vec<f64>>,
    pub attributes: Option<Vec<Attribute>>,
    pub attributes_nullable: Vec<Option<Attribute>>,
    pub nullable_attributes: Option<Vec<Option<Attribute>>>,
    pub newtypeint: Option<NewTypeInt>,
}

pub fn increment(opt: Option<ExoticOptionals>) -> Result<Option<ExoticOptionals>> {
    Ok(opt.map(|opt| ExoticOptionals {
        int32: Some(opt.int32.unwrap_or(0) + 1),
        int64: Some(opt.int64.unwrap_or(0) + 1),
        float64: Some(opt.float64.unwrap_or(0.) + 1.),
        boolean: Some(!opt.boolean.unwrap_or(false)),
        zerocopy: opt
            .zerocopy
            .map(|mut e| {
                e.0.push(42);
                e
            })
            .or_else(|| Some(ZeroCopyBuffer(vec![]))),
        int8list: opt
            .int8list
            .map(|mut e| {
                e.push(42);
                e
            })
            .or_else(|| Some(vec![])),
        uint8list: opt
            .uint8list
            .map(|mut e| {
                e.push(42);
                e
            })
            .or_else(|| Some(vec![])),
        // float64list: opt.float64list.map(|mut e| {
        // e.push(42.);
        // e
        // }),
        attributes: opt
            .attributes
            .map(|mut e| {
                e.push(Attribute {
                    key: "some-attrib".to_owned(),
                    value: "some-value".to_owned(),
                });
                e
            })
            .or_else(|| Some(vec![])),
        newtypeint: opt
            .newtypeint
            .map(|mut e| {
                e.0 += 1;
                e
            })
            .or_else(|| Some(NewTypeInt(0))),
        ..Default::default()
    }))
}

pub fn handle_boxed_optional(opt: Option<Box<f64>>) -> Result<f64> {
    match opt {
        Some(e) => Ok(*e + 1.),
        None => Ok(42.),
    }
}
