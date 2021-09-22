cargo update
wasm-pack build --target web
cp index.html docs/gh-pages
cp -R pkg docs/gh-pages/
