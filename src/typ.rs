use std::{any::type_name, u8, mem::size_of};

pub fn deftype<T>(_x:T)->(&'static str,usize){
    let mut t:&str = "";
    if type_name::<T>().contains("::"){
        let split = type_name::<T>().split("::");
        t = split.last().unwrap().trim_end();
    }
    else if type_name::<T>().contains("&"){
        let split = type_name::<T>().split("&");
        t = split.last().unwrap().trim_end();
    }
    println!("{}",type_name::<T>());
    match t{
        "u8"=>{return ("u8",size_of::<u8>())},
        "i8"=>{return ("i8",size_of::<i8>())},
        "u16"=>{return ("u16",size_of::<u16>())},
        "i16"=>{return ("i16",size_of::<i16>())},
        "u32"=>{return ("u32",size_of::<u32>())},
        "i32"=>{return ("i32",size_of::<i32>())},
        "char"=>{return ("char",size_of::<char>())},
        "String"=>{return ("String",size_of::<String>())},
        _=>{return (panic!("Type non reconnu"))}
     }   
}