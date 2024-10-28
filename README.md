# `zip_downloader`

> A simple Rust crate for one-line download and read of ZIP files.
> Provide downloaded content as a String or as bytes.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Usage

Library usage example

```rust
use zip_downloader::ZipDownloader;

let url = "https://some.url/file.zip";

// Download and get ZIP file as String
let string = ZipDownloader::get(url).unwrap().text().unwrap();

// Download and get ZIP file as bytes
let bytes = ZipDownloader::get(url).unwrap().bytes();

// Download and get 3rd line from ZIP-packed text
let line = ZipDownloader::get(url).unwrap().line(3).unwrap();

// Download and get 3rd byte from ZIP packed byte file
let byte = ZipDownloader::get(url).unwrap().bytes()[3];
```
