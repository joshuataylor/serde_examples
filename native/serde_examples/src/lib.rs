use std::collections::HashMap;
use rustler::{Encoder, Env, Term};
use rustler::NifUntaggedEnum;
rustler::init!("Elixir.SerdeExamples.Native", [vec_deserialize, hashmap_deserialize, example_snowflake_arrow]);

#[rustler::nif(schedule = "DirtyCpu")]
#[inline(always)]
fn vec_deserialize<'a>(env: Env<'a>, rust_type: String, size: usize) -> Term<'a> {
    // Not the greatest rust code, just a quick example
    if rust_type == "str" {
        return vec!["xxx"; size].encode(env);
    }
    if rust_type == "string" {
        return vec!["xxx".to_string(); size].encode(env);
    }
    if rust_type == "integer" {
        return vec![42; size].encode(env);
    }

    return "unmatched".encode(env);
}

#[rustler::nif(schedule = "DirtyCpu")]
#[inline(always)]
fn hashmap_deserialize<'a>(env: Env<'a>, rust_type: String, hashmap_size: usize, size: usize) -> Term<'a> {
    // Not the greatest rust code, just a quick example
    if rust_type == "str" {
        // Create a hashmap with the given size
        let mut hashmap = HashMap::new();
        for i in 0..hashmap_size {
            hashmap.insert(i, vec!["xxx"; size]);
        }
        return hashmap.encode(env);
    }
    if rust_type == "string" {
        // Create a hashmap with the given size
        let mut hashmap = HashMap::new();
        for i in 0..hashmap_size {
            hashmap.insert(i, vec!["xxx".to_string(); size]);
        }
        return hashmap.encode(env);
    }
    if rust_type == "integer" {
        // Create a hashmap with the given size
        let mut hashmap = HashMap::new();
        for i in 0..hashmap_size {
            hashmap.insert(i, vec![fastrand::i32(..); size]);
        }
        return hashmap.encode(env);
    }

    return "unmatched".encode(env);
}

#[rustler::nif(schedule = "DirtyCpu")]
#[inline(always)]
fn example_snowflake_arrow<'a>(env: Env<'a>) -> Term<'a> {
    // We return a vec of vecs, laid out like columns
    let mut return_vec = vec![];

    for i in 0..100000 {
        let mut vec: Vec<SnowflakeReturnType> = vec![
            SnowflakeReturnType::Float64(Some(32.0)),
            SnowflakeReturnType::Boolean(Some(false)),
            SnowflakeReturnType::Boolean(None),
            SnowflakeReturnType::Int8(Some(1)),
            SnowflakeReturnType::Int32(Some(100000000)),
            SnowflakeReturnType::String(Some("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string())),
            SnowflakeReturnType::String(Some("er3fgre32gf3r".to_string())),
            SnowflakeReturnType::Binary(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])),

        ];
        return_vec.push(vec);
    }

    return return_vec.encode(env);
}

#[derive(Debug, Clone, NifUntaggedEnum)]
pub enum SnowflakeReturnType {
    Int32(Option<i32>),
    Float64(Option<f64>),
    Float642(Option<f64>),
    Int8(Option<i8>),
    Boolean(Option<bool>),
    Binary(Option<Vec<u8>>),
    String(Option<String>)
}
