FROM node:15.14.0 AS client-base
COPY ./ ./app
WORKDIR /app
RUN cd client && npm ci && npm run build-prod

FROM rust:1.51.0 as server-base
COPY --from=client-base ./app ./app
WORKDIR /app
RUN cd server && cargo build --release && mkdir -p /build-out && cp target/release/fdr-show-indexer-server /build-out/

FROM debian:10-slim
COPY --from=server-base /build-out/fdr-show-indexer-server /
EXPOSE 80
CMD /fdr-show-indexer-server