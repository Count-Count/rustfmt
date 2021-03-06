// rustfmt-normalize_comments: true

extern crate foo;
extern crate foo as bar;

extern crate chrono;
extern crate dotenv;
extern crate futures;

extern crate bar;
extern crate foo;

// #2315
extern crate proc_macro;
extern crate proc_macro2;

// #3128
extern crate serde; // 1.0.78
extern crate serde_derive; // 1.0.78
extern crate serde_json; // 1.0.27

extern "C" {
    fn c_func(x: *mut *mut libc::c_void);

    fn c_func(
        x: XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX,
        y: YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY,
    );

    #[test123]
    fn foo() -> uint64_t;

    pub fn bar();
}

extern "C" {
    fn DMR_GetDevice(
        pHDev: *mut HDEV,
        searchMode: DeviceSearchMode,
        pSearchString: *const c_char,
        devNr: c_uint,
        wildcard: c_char,
    ) -> TDMR_ERROR;

    fn quux() -> (); // Post comment

    pub type Foo;

    type Bar;
}

extern "Rust" {
    static ext: u32;
    // Some comment.
    pub static mut var: SomeType;
}

extern "C" {
    fn syscall(
        number: libc::c_long, // comment 1
        // comm 2
        ... // sup?
    ) -> libc::c_long;

    fn foo(x: *const c_char, ...) -> libc::c_long;
}

extern "C" {
    pub fn freopen(
        filename: *const c_char,
        mode: *const c_char,
        mode2: *const c_char,
        mode3: *const c_char,
        file: *mut FILE,
    ) -> *mut FILE;


    async fn foo() -> *mut Bar;
    const fn foo() -> *mut Bar;
    unsafe fn foo() -> *mut Bar;

    pub async fn foo() -> *mut Bar;
    pub(super) const fn foo() -> *mut Bar;
    pub(crate) unsafe fn foo() -> *mut Bar;
}

extern "C" {}

// #2908 - https://github.com/rust-lang/rustfmt/issues/2908
#[wasm_bindgen(module = "child_process", version = "*")]
extern {
    #[wasm_bindgen(js_name = execSync)]
    fn exec_sync(cmd: &str) -> Buffer;

    fn foo() -> Bar;
}

// Users that have an existing explicit ABI would need to convert to implicit
// manually, as rustfmt will be conservative and not attempt to convert explicit
// to implicit in the wasm case.
#[wasm_bindgen]
extern "C" {
    fn foo() -> Bar;
}
