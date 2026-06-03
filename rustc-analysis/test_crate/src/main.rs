/*
// cycle test:
/*
dependency tree:
main -> [f_recursion_0]
f_recursion_0 -> [f_recursion_1, f_0]
f_recursion_1 -> [f_recursion_0, f_0]
f_0 -> []

ordering:
f_0
(f_recursion_0 | f_recursion_1)
main
*/ 

fn f_0(c: u32) -> u32 {
    c - 1
}

fn f_recursion_0(c: u32) -> u32 {
    let c_new = f_0(c);
    f_recursion_1(c_new)
}

fn f_recursion_1(c: u32) -> u32 {
    let c_new = f_0(c);
    f_recursion_0(c_new)
}

fn main() {
    let _ = f_recursion_0(4);
}
*/


// async filter test:
use tokio;

async fn async_fn_0() { 
    async_fn_1().await;
}

async fn async_fn_1() {
    let _ = tokio::fs::read_to_string("/dev/null").await;
}

fn main() {
    let closure = async || {
        let _ = async_fn_0().await;
    };

    closure();
}


// TODO: SHOULD test async fn1 -> fn2 -> async fn3
// fn1 , fn2 now with no dependency in arbitrary order is fine

/*
async_fn_0 -> [async_fn_1]
async_fn_1 -> []
main -> [async_fn_0]

*/

// FIXME: MUST include all functions (also synchronous) calling async functions

// TODO: SHOULD test external async functions

// TODO: COULD improve tool later to also somehow watch for `.await` statements?

