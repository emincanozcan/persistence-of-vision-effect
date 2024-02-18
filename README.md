# Persistence of Vision Effect

## Description

This project serves as a showcase for the persistence of view phenomenon in the human brain.

Objects become visible when in motion, appear to materialize and move seamlessly on a backdrop of random black and white pixels.

Get a screenshot, or just use the stop button in the WebUI to stop the motion, and the objects will just disappear.

[Click here to see the demo](https://persistence-of-vision-effect.vercel.app/).

## Technologies Used

- Rust
- WebAssembly (WASM)
- Javascript, HTML, and CSS

## Local Setup Instructions

To run the project locally, follow these steps:

1. Have `rust` `wasm-pack`, and `node.js` installed
2. Clone the repository.
3. Run `wasm-pack build` in the root directory
4. Go to the `www` directory, and run `npm install`
5. Run `npm run webpack:start`
6. Navigate to http://localhost:8080 in your browser

## License

This project is licensed under the [MIT License](LICENSE).
