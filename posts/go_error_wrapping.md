# Wrapping Errors

*I have released a Go linter to check for the rule described in this post. You
can find it here: [wrapcheck](https://github.com/tomarrell/wrapcheck)*

<br>

> This is a small post covering error wrapping in Go that will build on the work
> of Dave Cheney [here](https://dave.cheney.net/tag/stacktrace).

So recently my team at work had a discussion about how we can improve the
debugging experience when hunting down the source of an error which is returned
up the call stack.

This is a common thing in Go, as errors are simply values fulfilling the
interface:

```go
type Error interface {
  Error() string
}
```

This is both a powerful feature and a slight limitation in this scenario, as
errors don't by default hold any information about where they are generated
within your program. This can make identifying the source method of an error
difficult when it's returned repeatedly up a call stack, which is a common good
pattern in Go. This occurs especially in code which calls into a common library,
e.g. database methods.

The good thing about Go errors is that this requirement allows you to extend
errors however you would like. This includes adding extra information to the
errors. This is known as *wrapping*.

I'll breifly describe the history of wrapping errors in Go.

### Wrapping errors prior to Go `1.13`

Go has had error wrapping since the introduction of the
[`errors`](https://godoc.org/github.com/pkg/errors) package was introduced in
early 2016.

This shortly followed with the `errors.[New|Errorf|Wrap|Wrapf]` methods
implementing the interface:

```go
type Stack interface {
        Stack() []uintptr
}
```

The functionality for adding stack traces to errors 
