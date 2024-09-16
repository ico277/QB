# QB
Quick Bench - Tool for meassuring speed of executables
# Compiling from source
## Dependencies
 rust, cargo

More instructions as to how to install rust [*here*](https://www.rust-lang.org/tools/install).

### Building
```bash
$ make build
```
### Installing
```bash
# make install
```
Note: You can use `make install PREFIX=<prefix>` to change the prefix (default = /usr).

# Usage Examples
### Single threaded
Run [*nyafetch*](https://github.com/ico277/nyafetch/tree/rewrite) 5 times
```bash
qb --cmd "nyafetch" --iters 5
```
### Multi threaded
Run [*nyafetch*](https://github.com/ico277/nyafetch/tree/rewrite) 8 times using 2 threads
```bash
qb --cmd "nyafetch" --iters 8 --threads 2
```
### Multi threaded with shell
run a shell command 8 times using 2 threads
```bash
qb --shell --cmd 'echo "$((5**10))"' --iters 8 --threads 2
```
