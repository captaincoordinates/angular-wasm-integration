# Angular WASM Integration

Proof of concept to demonstrate integration of a WASM component, written in Rust, with an Angular web application.

***Note***: Uses Angular 15, as switch to Vite in Angular 16 introduces additional complexity when loading the WASM module and addressing Vite configuration issues is not the primary focus of this exercise.


# Development

Requires npm, Cargo, wasm-pack, and Fermyon Spin

```sh
cd image-processor
wasm-pack build
cd ../api
spin build --up
# new tab
cd ../app
npm install
ng serve
```
