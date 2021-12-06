// * Made by qqt#0001 on December 6th, 2021
mod library {
    // * Import the rand class
    use rand::Rng;

    // * Stop mangling of function names, which is required for Unity
    #[no_mangle]
    pub extern fn generate_random(min_range: i32, max_range: i32) -> i32
    {
        rand::thread_rng().gen_range(min_range..max_range)
    }
}
