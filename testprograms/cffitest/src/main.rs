use cty;

#[repr(C)]
pub struct CoolStruct {
    pub x: cty::c_int,
    pub y: cty::c_int,
}

extern "C" {
    pub fn cool_function(
        i: cty::c_int,
        c: cty::c_char,
        cs: *mut CoolStruct
    );
}

fn main() {
    let mut x = CoolStruct{x: 3, y: 4};
    unsafe {
        cool_function(1, 2, &mut x);
    }
}
