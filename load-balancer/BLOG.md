# Load Balancer

Problem Statement: https://codingchallenges.fyi/challenges/challenge-load-balancer

## Blog

**17-01-2026**:

Research:

- The problem statement states the use of a program named `lb`. This indicates that its most likely going to be a CLI tool.
    - Found this Rust CLI book to read about building the CLI: https://rust-cli.github.io/book
    - It mentions about this package called "Clap" which would be useful for implee

**19-01-2026**

- I got this resource which talks about binding to a port and actively listening: https://rust-lang-nursery.github.io/rust-cookbook/net/server.html

- The current `lb` program has some issues, so gotta fix that

**20-01-2026**

- I just realised the following is incorrect:

```rs
loop {
    run_server()
}
```

What's actually happening is that there is an infinite bind/unbind process that is occuring, whenever we accepting any connection. 

Ideally, the bind should happen only once and the acceptance of new connection should happen forever.

Hence, instead of looping the `run_server()`, we instead loop `match listener.accept()` block.

- Today, I completed the first half of Step 1, which was to essentially log out the information about incoming connection. Tested with both `curl` (from Powershell) and `Postman` and my output was something like following:

```
GET / HTTP/1.1
User-Agent: Mozilla/5.0 (Windows NT; Windows NT 10.0; en-IN) WindowsPowerShell/5.1.26100.7462
Host: localhost
Connection: Keep-Alive
GET / HTTP/1.1
User-Agent: PostmanRuntime/7.51.0
Accept: */*
Postman-Token: 8d52070d-63bc-40e9-ac7e-fcbf1f8770d6
Host: localhost
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
```

