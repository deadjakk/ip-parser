# ips

## overview   

parses ip addresses from stdin or a provided file and takes into consideration
blacklisted and whitelisted CIDRs.

Blacklisting is done after whitelisting and thus takes precedence.

### implemented features  

- `-k` keep duplicates in output
- `-b <file>` Blacklist ips within newline-delimited File containins CIDRs or IPs
- `-w <file>` Whitelist ips within newline-delimited File containins CIDRs or IPs
- `-i` include ips within provided comma-separated CIDRs 
- `-x` eXclude ips within provided comma-separated CIDRs 

### installation

run `cargo install ip-parser` to install and `ips` to run the program.

### help output

```
ips 1.0.0

USAGE:
    ips [FLAGS] [OPTIONS] [file-name]

FLAGS:
    -h, --help               Prints help information
    -k, --keep-duplicates    non-unique output, keep any duplicates
    -V, --version            Prints version information

OPTIONS:
    -b, --blacklist-file <blacklist-file>    file that contains CIDRs & IP addrs against which to filter output (exlude)
    -i, --include <include>                  single or comma-separated CIDRs for whitelisting
    -w, --whitelist-file <whitelist-file>    file that contains CIDRs & IP addrs against which to filter output
                                             (include)
    -x, --xclude <xclude>                    single or comma-separated CIDRs for blacklisting

ARGS:
    <file-name>    file path from which to parse ip addresses. omitting this forces reading from stdin
```

### usage examples

```
$ cat testfile | ips
1.1.1.1
127.0.0.1
2.2.2.2
$ cat testfile | ips -x 2.2.2.2
1.1.1.1
127.0.0.1
$ cat testfile | ips -i 2.2.2.2
2.2.2.2
$ ips -i 2.2.2.2 testfile
2.2.2.2
$ ips -i 2.2.2.2,1.1.1.1 testfile
1.1.1.1
2.2.2.2
$
```
