# to-and-from

The easiest command line tool for converting files from one type to
another. It has a plain english syntax to be as memorable as possible

## Status: Planning / Prototyping

## Idea planning

 - 3 binaries: `to`, `from`, `toandfrom` (and maybe `toandfrom-autocomplete`)
   - `<./a.json to csv > a.csv`
   - `<./a.wrongextenson from tsv to csv > a.csv`
   - `from ./a.wrongextenson from tsv to csv > a.csv`
   - `to a.csv from a.json`
   - `<./a.json to csv with quotes \" # outputs to stdout`
   - `from a.csv to csv without headers | sponge a.csv`
   - `from a.csv to csv without headers | sponge a.csv`
 - Things to convert:
   - Any standard plaintext types: json, csv, tsv,
   - Images, videos (imagemagick and/or ffmpeg)
   - Rich documents (pdf, doc(x), odt)
   - Archives:
     - `from a.zip to a/`
     - `from a/ to a.tgz`
     - `from data.json.gz to csv`
   - Data conversion:
     - Units `from 3 inches to mm` (`seq 1 12 | from inches to mm`)
     - Dates `from timestamps to date:"%Y-%m-%d"`
     - Encodings `from utf8 to utf16`
     - Line endings?! `from CRLF to `
 - Implementation ideas
   - Get up and going ASAP by just plugging together as many
   - Get into all the top package managers
   - Have as-smart-as-possible autocomplete and excellent error messages
   - Have docs for what every type is.
   - If we don't know how to convert something, offer a util that does
     know how to.
 - Other random ideas:
   - Read straight from URLs: `from https://example.com/a.json to b.csv`
   - Is there a reasonable means of arbitrary extension for this? Sort
     of like extending git by just adding `git-something` to PATH;
     maybe add `toandfrom--abc-to-csv` and we'll automatically learn
     how to handle csv
     - And then automatically setup transitive pipes for any other
       conversion types we know! Just supply us with `abc-to-csv` and
       we'll enable `from abc to json`, and anything else know how to
       convert to/from csv

## License

Copyright © 2018 George Pittarelli

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
