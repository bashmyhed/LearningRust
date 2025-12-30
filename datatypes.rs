fn main() {
    let num:u32 = 5400;     // unsigned 32 bit int
    let num:i32 = 5400;     // signed 32 bit int
    let num= 5400i64;       // signed 64 bit int
    let num= 5_400i64;      // _ is purely cosmetinc ...
    let num= 5_400_i64;     // _ is purely cosmetinc ...
    let num = 0xff;          // hex  0o octadecimal   0b is binary  b is bit(u8)
    
    let x = 254u8;
    let y = 30+x;  // will panic in regular/debug compile and wrap in release build
    let y = x.wrapping_add(30);  // returns 28
    let y = x.checked_add(30);  // returns none
    let (overflowed , y) = x.overflowing_add(30);  // returns y = 28 and overflowed = true
    //   ^ this is a tuple size set at compile time.. multi type..

    let y = x.saturating_add(30);  // returns 256 (ceil or floor)
    
    let tup = (6,54.3,true);
    println!("value {} , {} , {} ",tup.1,tup.2,tup.0);

    println!("Hello, world!");
}

