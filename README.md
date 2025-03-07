Join a user namespace (optional) and a network namespace, open a TUN/TAP device, then "send" the file descriptor via an inherited UNIX domain socket.
\
Example:
\
`./namespace-fd-helper -sock-fd 3 -pid 3000 -if-name tun0`
