# parser-performance

This is my playground to evaluate the performance of various frameworks and programming languages. In the first
iteration, I will focus on the Json parser as this is an area people have done good optimization in their
perspective frameworks and languages so we can have a fair comparison.

The test data is a 5000 objects Json file generated from  json-generator.com and is around 5.2 MB in size. The time use measured using
macOS/Linux `time` command. Memory is measured using the [memusg script](https://gist.github.com/netj/526585) which is copied to the script folder.

## Results

| Parser |  Time(s) |  Peak memory(MB) |
|---|---|---|
| Python/lark | 5.127 | 59.344 |
| Rust/nom | 0.215 | 27.828 |

Rust parser is 23+ times faster and uses < 47% of peak memory.
## Python

### python-lark-json

I just follow [this](https://github.com/lark-parser/lark/blob/master/docs/json_tutorial.md).

Change to `python-lark-json` directory. Create a Python virtual environment, activate and then run `python -m pip install -requirements.txt`.

Then run:

```
time python json_parser.py ../data/5000_objects.json
```

Results: 

```
5.03s user 0.06s system 99% cpu 5.127 total
```

```
../scripts/memusg python json_parser.py ../data/5000_objects.json
```

Results:

```
memusg: peak=59344
```

## Rust

### rust-nom-json

The parser is adapter from [nom json example](https://github.com/Geal/nom/blob/main/examples/json.rs)

To compiler, from rust-nom-json directory, run:

```
cargo build --release
```

Time test:

```
time ./target/release/rust-nom-json ../data/5000_objects.json
```

Results:

```
0.19s user 0.02s system 97% cpu 0.215 total
```

Memory test:

```
../scripts/memusg ./target/release/rust-nom-json ../data/5000_objects.json
```

Results:

```
 memusg: peak=27828
```