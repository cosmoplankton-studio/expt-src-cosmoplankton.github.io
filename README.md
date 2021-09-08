*Cosmoplankton Experiments*: Source for the various webapp experiments using WebAssembly, Rust, React, Redux, etc.
   
WebApp Link: [expt.cosmoplankton.studio](https://expt.cosmoplankton.studio/)

---

Repository Structure:
```sh
expt-src-cosmoplankton.github.io
    |__ package.json # package file with dependencies and scripts
    |__ webpack.common.js # webpack config common to dev/prod
    |__ webpack.dev.js # webpack development config
    |__ webpack.prod.js # webpack production config
    |__ static # static files those will be copied to 'dist' during build
    |   |__ common.css # common css for the entire app
    |   |__ ...
    |__ app-core # webassembly module source
    |   |__ Cargo.toml # cargo manifest file of the rust/wasm library
    |   |__ src # rust source files of the library
    |   |   |__ lib.rs # wasm/rust library entry point
    |   |   |__ experiments.rs
    |   |   |__ app_client.rs # app client class abstrating the gfx backend
    |   |   |__ app_state.rs # static app states
    |   |   |__ experiments
    |   |       |__ common_utils.rs # common linear algebra functions
    |   |       |__ webgl.rs # mod file exposing the webgl mod
    |   |       |__ webgl # webgl folder (gl_program.rs, gl_utils.rs, gl_shader_*.rs)
    |   |__ pkg # temp folder with the target wasm module and Js-glue
    |
    |__ src # webapp and react-redux app source
    |   |__ app.js # main webapp entry
    |   |__ app-ux # react-redux app folder
    |   |   |__ store.js # redux store
    |   |   |__ App-ux.js # react app
    |   |   |__ ...
    |   |__ pages # website entry-points and template files
    |       |__ index.js # '/' entry
    |       |__ index-template.html # '/' template
    |       |__ webgl.js # '/webgl' entry 
    |       |__ webgl-template.html # '/webgl' template
    |       |__ ...
    |__ dist # temp folder to hold the generated static site to be deployed

```

---

© 2021 Cosmoplankton. All rights reserved.
