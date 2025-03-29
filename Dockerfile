FROM alpine:3.20

ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG TARGETOS
ARG TARGETARCH
ARG TARGETVARIANT
RUN echo "PKDNS Build Starting...$TARGETPLATFORM"
RUN echo "$TARGETOS : $TARGETARCH : $TARGETVARIANT"

# CMD ["/bin/sh"]

# Conditional RUN command based on the target platform
# RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
#         echo "Building for AMD64" && \
#         COPY ./target/x86_64-unknown-linux-gnu/release/pkdns /usr/local/bin && \
#         COPY ./target/x86_64-unknown-linux-gnu/release/pkdns-cli /usr/local/bin; \
#     elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
#         echo "Building for ARM64" && \
#         COPY ./target/release/pkdns /usr/local/bin && \
#         COPY ./target/release/pkdns-cli /usr/local/bin; \
#     else \
#         echo "Unknown $TARGETPLATFORM"; \
#     fi


# RUN case "${TARGETPLATFORM}" in \
#         "linux/amd64") ENV TARGET_DIR='x86_64-unknown-linux-gnu/' ;; \
#         "linux/arm64") ENV TARGET_DIR='/' ;; \
#         *) exit 1 ;; \
#     esac;

# COPY target/$TARGET_DIR/release/pkdns /usr/local/bin
# COPY target/$TARGET_DIR/release/pkdns-cli /usr/local/bin  

COPY ./target/x86_64-unknown-linux-gnu/release/pkdns /usr/local/bin 
COPY ./target/x86_64-unknown-linux-gnu/release/pkdns-cli /usr/local/bin 

EXPOSE 5353
# EXPOSE map[5300/tcp:{} 53/tcp:{}]
# EXPOSE map[53/tcp:{}]

CMD ["pkdns"]