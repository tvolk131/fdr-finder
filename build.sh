# TODO - If client build fails, fail and don't build server.
cd ./client
npm i
npm run build-dev
cd ../server
cargo build