use libloading::{Library, library_filename, Symbol};

fn main() {
    unsafe {
        // Will attempt to load `libhello_world.so` on Linux, `libhello_world.dylib`
        // on macOS and `libhello_world.dll` on Windows.
        let lib = Library::new(library_filename("hello_world")).unwrap();

        let func: Symbol<fn()> = lib.get(b"execute").unwrap();
        func()
    }
}
