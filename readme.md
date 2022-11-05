# Syslib
Use the Linux ABI on Rust, without C.

Syslib tries to make using Linux syscalls as enjoyable as possible. Syscalls are wrapped to be usable from safe Rust where possible (perhaps dangerously so!).

## Motivation
This library is not portable - and it is not intended to be. A major issue with `libc` is its need to be portable, even if that means sacrificing everything else.
But despite that, applications and libraries will typically need to conditonally compile fixes for quirks on each platform. Sick of the unnecessary pain `libc` was
causing me (null-terminated strings may be the biggest mistake in computing), I decided to take the possible performance boost and just enjoy Linux's stable ABI.

Not to mention, it is pretty cool to potentially have the whole userspace stack Rust based.

## Safety
There is little-to-no testing of this library outside of my own use, currently. I wouldn't trust it without a good review first.
Further, it is under heavy development and no interfaces are stable.