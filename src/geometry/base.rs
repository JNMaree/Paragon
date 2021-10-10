#![allow(non_camel_case_types)]

type BaseType_Integer = i64;

pub struct BaseType_Dimension {
    pub integer: baseType_integer,
    pub decimal: baseType_integer
}

impl BaseType_Dimension {

    // Return the rounded off value of the dimension
    //  - To the floor
    pub fn floor(&self) -> BaseType_Dimension {
        return self.integer;
    }

    // Return the rounded off value of the dimension
    //  - To the ceiling
    pub fn ceil(&self) -> BaseType_Dimension {
        return self.integer + 1;
    }
}


