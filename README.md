# rust-vs-go-prime-numbers

Source: https://x.com/kai_fall/status/1934944986740425084

Thread says:

rust = 5.41s
go = 0.35s

Constraints:
- must be concurrent
- must not use third party library

## Remarks

I think it is a bit unfair for rust since the Go version uses
goroutine whereas in Rust, its equivalent is to use an async runtime
like tokio. You can probably do it with a thread pool but it would be an overkill.

Not sure what's going on with the threading and stuff but I tried to
run Rust using a non-threaded version and suprisingly it is a lot faster than Go.
It is possible that I got this result because I only have 1 CPU in the VM.
Will try with multiple cores.

Point is, the task is too trivial that Go will own Rust easily but once
the big boys steps in (tokio, rayon, etc), Rust with surely be much faster.
