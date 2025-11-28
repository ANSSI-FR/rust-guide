FROM rust:1.91.1-slim-trixie
# FROM debian:latest

RUN cargo install mdbook --git https://github.com/hg-anssi/mdBook.git --rev c5a35b9296c6d5e48570e30022bd69403050a9f4 --locked && rm -rf /usr/local/cargo/git /usr/local/cargo/registry

COPY ./mdbook-checklist mdbook-checklist
RUN cargo install --path ./mdbook-checklist --locked && rm -rf mdbook-checklist && rm -rf /usr/local/cargo/git /usr/local/cargo/registry

COPY ./mdbook-code-align mdbook-code-align
RUN cargo install --path ./mdbook-code-align --locked && rm -rf mdbook-code-align && rm -rf /usr/local/cargo/git /usr/local/cargo/registry

COPY ./mdbook-extensions mdbook-extensions
RUN cargo install --path ./mdbook-extensions --locked && rm -rf mdbook-extensions && rm -rf /usr/local/cargo/git /usr/local/cargo/registry

# RUN rm -rf /usr/local/cargo/git /usr/local/cargo/registry