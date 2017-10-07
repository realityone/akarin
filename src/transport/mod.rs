use std::fmt::Debug;
use std::io::{Write, Read};

pub trait Transport: Debug + Write + Read {}
