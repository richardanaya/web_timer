#![no_std]
use js_ffi::*;

pub struct Timer {
    fn_set_timeout: JSValue,
    fn_set_interval: JSValue,
    fn_request_animation_frame: JSValue,
    fn_request_animation_loop: JSValue,
    fn_clear_timeout: JSValue,
    fn_clear_interval: JSValue,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            fn_set_timeout: register("window.setTimeout"),
            fn_set_interval: register("window.setInterval"),
            fn_request_animation_frame: register("window.requestAnimationFrame"),
            fn_request_animation_loop: register(
                "(cb)=>{
                    let time = Date.now();
                    function run(){
                        let new_time = Date.now();
                        let delta = new_time-time;
                        time = new_time;
                        window.requestAnimationFrame(run);
                        cb(delta);
                    }
                    window.requestAnimationFrame(run);
                }",
            ),
            fn_clear_timeout: register("window.clearTimeout"),
            fn_clear_interval: register("window.clearInterval"),
        }
    }
}

impl Timer {
    pub fn set_timeout(&self, callback: JSValue, milliseconds: usize) -> JSValue {
        call_2(
            UNDEFINED,
            self.fn_set_timeout,
            TYPE_FUNCTION,
            callback,
            TYPE_NUM,
            milliseconds as JSValue,
        )
    }

    pub fn sleep(&self, milliseconds: usize) -> CallbackFuture {
        let (future, cb) = CallbackFuture::new();
        call_2(
            UNDEFINED,
            self.fn_set_timeout,
            TYPE_FUNCTION,
            cb,
            TYPE_NUM,
            milliseconds as JSValue,
        );
        future
    }

    pub fn set_interval(&self, callback: JSValue, milliseconds: usize) -> JSValue {
        call_2(
            UNDEFINED,
            self.fn_set_interval,
            TYPE_FUNCTION,
            callback,
            TYPE_NUM,
            milliseconds as JSValue,
        )
    }

    pub fn request_animation_frame(&self, callback: JSValue) {
        call_1(
            UNDEFINED,
            self.fn_request_animation_frame,
            TYPE_FUNCTION,
            callback,
        );
    }

    pub fn request_animation_loop(&self, callback: JSValue) {
        call_1(
            UNDEFINED,
            self.fn_request_animation_loop,
            TYPE_FUNCTION,
            callback,
        );
    }

    pub fn clear_timeout(&self, handle: JSValue) {
        call_1(UNDEFINED, self.fn_clear_timeout, TYPE_NUM, handle);
    }

    pub fn clear_interval(&self, handle: JSValue) {
        call_1(UNDEFINED, self.fn_clear_interval, TYPE_NUM, handle);
    }
}
