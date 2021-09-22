cargo update
wasm-pack build --target web
cp index.html docs/
cp -R pkg docs/
rm -rf docs/pkg/.gitignore
