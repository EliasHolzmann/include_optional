fn main() {
    /* This does not fail only when executing `include_str`, but already when executing
     * `include_str_optional` – we don't have permissions to look inside the `fd` directory, so
     * `Path::try_exists` returns `Error(_)`
     */
    include_optional::include_str_optional!("/proc/1/fd/doesnt_exist");
}