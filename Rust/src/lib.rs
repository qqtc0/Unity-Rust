mod library {
    use rand::Rng;

    // Stop mangling of function names, which is required for DllImport (unless you somehow enjoy passing the mangled name too).
    // Do note though that `no_mangle` will prevent generic types [ `T` ] from working.
    #[no_mangle]
    pub extern "C" fn generate_random(min_range: i32, max_range: i32) -> i32
    {
        rand::thread_rng().gen_range(min_range..max_range)
    }
}
