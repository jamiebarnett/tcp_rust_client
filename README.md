# TCP client example

An example TCP client that writes input from stdin to a remote server.

## Examples

Start `netcat` in a terminal window to listen for TCP connections on a port. 

```
$ netcat -l 4444
```

In a new terminal window run this binary.

```
$ cargo run
```

Any input to stdin will be printed in the netcat output.