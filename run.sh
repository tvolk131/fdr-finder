# TODO - If client build fails, don't start server.
cd ./client
npm run build-dev
cd ../server
cargo run