# RustyTokenManipulation
just manipulatin these here tokens yes sir nothing weird

## Compile
```
$ cargo build --target x86_64-pc-windows-gnu
```
## CreateProcessWithLogonW
This API call takes WCSTR username, domain, and password values to make a process with the new token.

```
Token impersonation

USAGE:
    rusty_impersonate_token.exe [OPTIONS]

OPTIONS:
    -c, --command <COMMAND>      Command to run
    -d, --domain <DOMAIN>        Domain name
    -h, --help                   Print help information
    -p, --password <PASSWORD>    Password
    -u, --username <USERNAME>    Username
```
