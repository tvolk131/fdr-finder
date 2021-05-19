# TODO - If client build fails, don't start server.
cd ./client
npm i
npm run build-dev
cd ../server
cargo run