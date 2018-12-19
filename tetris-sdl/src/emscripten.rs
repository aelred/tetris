use libc;
use std::panic::{self, AssertUnwindSafe};

extern "C" {
    fn emscripten_set_main_loop_arg(
        func: extern "C" fn(_: *mut libc::c_void),
        arg: *mut libc::c_void,
        fps: libc::c_int,
        simulate_infinite_loop: libc::c_int,
    );
}

extern "C" fn func_wrapper<F: FnMut() + 'static>(closure: *mut libc::c_void) {
    let closure = closure as *mut F;

    // This is safe if we assume emscripten always passes the callback just what we passed in.
    let closure = unsafe { &mut *closure };

    // The closure could panic and we mustn't unwind into emscripten
    panic::catch_unwind(AssertUnwindSafe(closure)).unwrap();
}

pub fn set_main_loop<F: FnMut() + 'static>(func: F, fps: i32) -> ! {
    // It's very important that the argument passed to emscripten doesn't get dropped.
    // To do this, we allocate our argument on the heap and transform it into a raw pointer.
    // This means we have deliberately introduced a memory leak! Yay!
    let arg_on_heap = Box::into_raw(Box::new(func));

    let arg = arg_on_heap as *mut libc::c_void;

    // This is safe if we assume emscripten doesn't mess with any arguments we pass.
    unsafe {
        emscripten_set_main_loop_arg(func_wrapper::<F>, arg, fps, 1);
    }

    unreachable!("emscripten should enter an infinite loop")
}
