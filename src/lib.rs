//! A library for handling money in a typesafe way

pub mod parse;

pub mod currencies;
pub mod printer;

#[derive(PartialEq,Debug)]
pub struct Transaction<A>{
    from_id:i32,
    to_id:i32,
    amount:A,
}

pub trait Account{
    fn id(&self)->i32;
}


