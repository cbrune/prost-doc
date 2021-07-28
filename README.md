This shows a simple example of what I believe to be a bug with the
documentation for the prost generated output for the enum type.

The attribute `#![warn(missing_docs)]` is set before including the
generated protobufs.

All the fields in the source `test.proto` file are documented.

With the `missing_docs` lint enabled `cargo build` gives this result:

    photon:~/src/prost-doc$ cargo build
       Compiling prost-doc v0.1.0 (/home/curt/src/prost-doc)
    warning: missing documentation for an associated function
      --> /home/curt/src/prost-doc/target/debug/build/prost-doc-890d2974c9ae4c69/out/test.rs:11:32
       |
    11 |     #[derive(Clone, PartialEq, ::prost::Oneof)]
       |                                ^^^^^^^^^^^^^^
       |
    note: the lint level is defined here
      --> src/lib.rs:3:9
       |
    3  | #![warn(missing_docs)]
       |         ^^^^^^^^^^^^
       = note: this warning originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

