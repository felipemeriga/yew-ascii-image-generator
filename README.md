# yew-ascii-image-generator

## Introduction 

This repo is supposed to be an example project, for people looking into building frontend apps using Rust,
and its framework Yew.

You can check the demo on that link: [DEMO](http://yew-ascii-generator.s3-website-us-west-2.amazonaws.com/)

Yew is a modern Rust framework for creating frontend web apps with WebAssembly. It features 
a component-based architecture similar to React or Elm, using a virtual DOM implementation for efficient rendering and 
updating of the user interface.

Yew is an excellent choice if you want to write a web application entirely in Rust while taking advantage of modern web 
features like WebAssembly and Web Workers. It allows you to benefit from Rust's performance and safety guarantees 
while building dynamic, feature-rich web applications.


### WebAssembly

WebAssembly (often abbreviated as WASM) is an open standard that defines a binary code format for web browsers. 
The goal of WebAssembly is to enable high-performance applications on web pages, but the format is designed to be
executed and integrated in other environments as well, not just web pages.

### Trunk


Trunk is a powerful build system for (but not limited to) Yew applications. It simplifies the building and bundling
process of your Yew application into a single command, eliminating the need for complex configuration files.

## Application

This is a simple application about uploading an image from local, and getting the ASCII version of this image
in the browser. It isn't meant to be a professional application, just an app for leaning how to use Rust to 
write frontend web apps, with the help of Yew.

Features that we use:

- Futures
- Functional components
- Results and Error handling
- Image manipulation

### How to Run

First build the project with cargo
```shell
cargo build
```

After that, install trunk
```shell
cargo install trunk
```

You need to install the target triple for WASM:
```shell
rustup target add wasm32-unknown-unknown
```

For running locally, for development
```shell
trunk serve
```

For distributing and building your app:
```shell
trunk build
```