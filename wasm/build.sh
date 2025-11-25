wasm-pack build --release --target web --out-dir ./js/src/wasm --out-name duckity --no-pack
rm ./js/src/wasm/.gitignore
cd js
bunx tsc
cp ./src/wasm ./dist -r
cd ../