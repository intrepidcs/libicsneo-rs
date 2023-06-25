libicsneo-sys
===

System level crate for [libicsneo](https://github.com/intrepidcs/libicsneo). This crate utilizes cbindgen for automatic bindings. This crate is currently not ready for production and in very early stages.


How to use
===

Add to your cargo project:
```
$ cargo add libicsneo-sys
```

Example use case of `icsneo_findAllDevices()`:
```rust
use libicsneo_sys::*;

let mut device_count = 255;
let mut devices = [
    neodevice_t {
        device: 0 as *mut std::os::raw::c_void,
        handle: 0i32,
        serial: [0i8; 7],
        type_: 0,
    }; 255];
unsafe {
    icsneo_findAllDevices(devices.as_mut_ptr(), &mut device_count);
}
println!("Found {} device(s)", device_count);
```

License - MIT
===
Copyright Intrepid Control Systems, Inc.

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.