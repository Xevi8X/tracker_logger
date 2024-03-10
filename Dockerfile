FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/tracker_logger
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /usr/local/cargo/bin/tracker_logger /usr/local/bin/tracker_logger
CMD ["tracker_logger"]
EXPOSE 3333