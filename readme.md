# fun little project with tshark

## dependencies
- wireshark


run it like
```
tshark -T ek 2>/dev/null | cargo run
```
if you made a binary use the location instead of `cargo run` and if you want to filter anything out or do anything with the input, you can pip `jq` in between, you can also ofcourse read the json from a file instead of directly from tshark

have a nice day
