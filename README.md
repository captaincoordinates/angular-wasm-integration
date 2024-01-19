# WASM-to-WASM Proof of Concept

Proof of concept to demonstrate integration of a WASM component written in Rust with an Angular web application. The WASM component communicates with an HTTP API, also written in Rust using the Fermyon Spin framework and compiled to WASM. The WASM HTTP API can be executed by Docker if the required functionality is enabled, otherwise it can be executed by Fermyon Spin.

The HTTP API depends on data in a public S3 bucket [here](https://tchristian-wasm-data.s3.us-west-2.amazonaws.com/T09UXA_20231210T194821_B04_clip.tif) and [here](https://tchristian-wasm-data.s3.us-west-2.amazonaws.com/T09UXA_20231210T194821_B08_clip.tif). If you see runtime errors this is likely due to problems accessing data.

***Notes***:
- There are a number of areas that could be improved here, but are ignored for the sake of proving the capability with minimal effort.
- Uses Angular 15, as the switch to Vite in Angular 16 introduces additional complexity when loading the WASM module and addressing Vite configuration issues is not the primary focus of this exercise.


# Running
```sh
# requires Docker configured to support WASM and have runtimes installed
scripts/demo.sh
# http://localhost:8123
# kill with ctrl+c and wait
```


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
# http://localhost:4200
```
