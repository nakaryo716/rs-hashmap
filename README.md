# rs-hashmap
## Purpose
This library is intended for educational purposes.  
Let's implement a custom `HashMap` in Rust.
## Run
### Example
```bash
docker image build --file Dockerfile --tag rs_hashmap:latest . \
&& docker container run --rm  rs_hashmap:latest ./target/release/examples/main
```
### Test
```bash
docker image build --file Dockerfile --tag rs_hashmap:latest . \
&& docker container run --rm  rs_hashmap:latest
```
## License
`rs-hashmap` is provided under the MIT license. See [LICENSE](LICENSE)
