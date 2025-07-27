export RUSTDOCFLAGS := "--html-in-header katex/katex.html"

doc:
    cargo doc --document-private-items --no-deps --open
