A 3D field visualizer that will use openscad models for defining the
geometry of volumetric charges. This will run in the browser at some
point using three.js for vizualization and rust for numerical
calculations.

It doesn't work yet.


a quick check to see how the runtime performance stacks up between
WASM running under wasmtime and native code show that native code runs
roughly 25x faster at the moment. But it runs! Doing file I/O without 

-----

20:49 $ time wasmtime --dir=. fieldsim.wasm 
π: 3.978

real	0m0.746s
user	0m0.700s
sys	0m0.044s

-----

20:50 $ time cargo run --release 
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/fieldsim`
π: 3.966

real	0m0.030s
user	0m0.016s
sys	0m0.012s
~/rustcourse/fieldsim [master|…1] 
