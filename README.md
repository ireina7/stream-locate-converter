# stream-locate-converter
Reader offset and line-column location converter.  
A Stream reader which can convert between byte offset and line-column numbers.
Support any type which implements `io::Read`.

# Example
```rust
use stream_locate_converter::Stream;
use stream_locate_converter::location;
use std::fs;

fn main() -> io::Result<()> {
  let file = fs::File::open("foo.rs")?;
  let mut stream = Stream::from(file);

  let offset = location::Offset::new(20);
  let line_index = stream.line_index(offset)?;

  let (line, col) = line_index.one_based();
  println!("The offset is on line {line}, column {col}.");
}
```
