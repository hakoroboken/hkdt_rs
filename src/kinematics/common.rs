use crate::common::Position2D;
use std::collections::HashMap;

pub struct UnitPosition
{
    pub data : HashMap<String, Position2D>
}

impl UnitPosition {
    pub fn new()->Self
    {
        return Self { data: HashMap::new() }
    }

    pub fn add_unit(&mut self, name : &str, position : Position2D)
    {
        self.data.insert(name.to_string(), position);
    }
}