use std::fmt::Debug;
use std::io::{Read, Write};

pub trait Transport: Debug + Write + Read {}
