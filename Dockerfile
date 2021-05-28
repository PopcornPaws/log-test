FROM rust

COPY log-test /bin/log-test

CMD = ["/bin/log-test"]
