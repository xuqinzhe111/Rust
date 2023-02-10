// -------------------------------------------
//           	- Simplifying structures
// ------------------------------------------- 

/*

struct A {
    f1: i32,
    f2: i32,
    f3: i32,
}

fn func_1(a: &mut A) -> &u32 { &a.f2 }
fn func_2(a: &mut A) -> u32 { a.f1 + a.f3 } 

fn func_3(a: &mut A) {
    let x = func_1(a);
    let y = func_2(a); 
    println!("{}", x);
}

struct A {
    b: B,
    c: C,
}
struct B {
    f2: u32,
}
struct C {
    f1: u32,
    f3: u32,
}

*/ 


// These functions take a B or C, rather than A.
fn func_1(b: &mut B) -> &u32 { &b.f2 }
fn func_2(c: &mut C) -> u32 { c.f1 + c.f3 }

fn func_3(a: &mut A) {
    let x = func_1(&mut a.b);
    let y = func_2(&mut a.c);
    println!("{}", x);
}

fn main() {}



