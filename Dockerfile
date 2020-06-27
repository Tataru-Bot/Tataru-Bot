FROM debian:buster-slim
COPY target/release/tataru /usr/local/bin/tataru
CMD ["tataru"]