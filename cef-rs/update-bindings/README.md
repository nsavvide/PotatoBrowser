# update-bindings

Download the prebuilt [Chromium Embedded Framework](https://github.com/chromiumembedded/cef)
archive on any supported platform and run `bindgen` on the C API for the `cef-dll-sys` crate,
then regenerate the safe bindings in the `cef` crate.
