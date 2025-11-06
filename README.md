# this is not finished yet!!!! feel free to make any suggestions in issues or pull requests.
a privacy-focused messaging protocol. crates will be provided in future for the server, the client, and the shared stuff.

## protocol:
server:
    - a server must response with a version, a status, headers, and a body. each status's form is strictly declared in ResponseKind at the end of meta.rs
    - the body is a string of any u32s, the rest is u8 only
client:
    - a client must make a request with a version, a request kind, headers and a body. each request's form is strictly defined in RequestKind
    - the body is a string of any u32s, the rest is u8 only

## plans:
- better encryption
- server <-> xmpp interop
