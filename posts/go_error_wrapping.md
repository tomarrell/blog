# Extending Errors in Go

*I've released a Go linter to check for the rule described in this post. You can
find it here: [wrapcheck](https://github.com/tomarrell/wrapcheck)*

<hr />

> This is a small post covering error wrapping in Go that will build on the work
> of Dave Cheney [here](https://dave.cheney.net/tag/stacktrace).

So recently, after spending more time than I should hunting down the source of
an error, I sparked the discussion within my team at work about how we can cut
down on the time to debug, specifically, by making sure that errors aren't
handed back to the caller without forgetting to attach extra information.

We already have a pretty good practice in place to wrap errors with additional
info before we hand them up the stack, however this was missing in this
particular case.

Essentially, we were seeing something akin to:

```log
time="2020-08-04T11:36:27+02:00" level=error error="sql: no rows in result set"
```

...not *particularly* useful.

To help with debugging here, it's important that additional information is
attached to the error to give the developer more context.

### Extending errors

By now you've probably already heard the popular Go proverb of:

> Don’t just check errors, handle them gracefully

Handling an error in this case would be to add this additional information and
pass it back up the stack. However we're not just limited to that. If we take a
closer look at what an error is in Go, we see the following interface.

```go
type Error interface {
  Error() string
}
```

Very simple, yet very powerful. However this power comes with many
possibilities, and unfortunately many possibilities comes with no standard way
of handling things. Ultimately, it is up to you to define what form of extension
is most suitable for your application.

An example is in Rob Pike's Upspin project. They use a custom error struct which looks
like:

```go
type Error struct {
    Path upspin.PathName
    User upspin.UserName
    Op  Op
    Kind Kind
    Err error
}
```

You can read more about how exactly they use this error definition in a blog
post they
[published](https://commandcenter.blogspot.com/2017/12/error-handling-in-upspin.html).

This is a bit intensive for many programs however, so we'll take a look at a
less intense, more widely used method for adding information to errors, called
wrapping.

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

The functionality for adding stack traces to existing errors came with the
introduction of the
[`errors.WithStack()`](https://pkg.go.dev/github.com/pkg/errors?tab=doc#WithStack)
method on the same package.

This returns an error with a format method which will print the stack trace when
used with the `%+v` verb.

```go
import "github.com/pkg/errors"

...

func (db *DB) getTansactionByID(tranID string) (Transaction, error) {
	sql := `SELECT * FROM transaction WHERE id = $1;`

	var t Transaction
	if err := db.conn.Get(&t, sql, tranID); err != nil {
		return Transaction{}, errors.Wrap(err, "failed to get transaction")
	}

	return t, nil
}

...
```

Which, when logging the error with the `%+v` verb, prodcues something along the
lines of:

```log
sql: no rows in result set
main.main
        /Users/tom/Documents/tmp/main.go:10
runtime.main
        /usr/local/Cellar/go/1.14.6/libexec/src/runtime/proc.go:203
runtime.goexit
        /usr/local/Cellar/go/1.14.6/libexec/src/runtime/asm_amd64.s:1373
```

We can use this to our advantage when creating errors to include a stack trace
in them. This can then be logged along with the error in order to aid locating
the source of the error.

One downside of this approach is the performance penalty. The necessary call
into `runtime.Stack(buf []byte, all bool) int` incurs a non-negligible cost, and
should be avoided in hot paths.

Stack traces are also not particularly human friendly. They will only identify
*where* the error occurred, not *why*.

Another minor nitpick is that when wrapping errors multiple times with
`github.com/pkg/errors`, it will leave you with multiple stack traces, one for
each time you wrap the error. This creates a rather large set of logs when you
finally print the error if your stack if deep 🃏.

### Wrapping errors in Go `1.13`

As of Go `1.13`, the introduction of the `%w` verb, as well as a few methods on
the `errors` package, have given us another option.

By using `fmt.Errorf()` with the `%w` verb, we're able to enhance errors with
additional information using only the standard library.

A similar stack tracing effect can be achieved by wrapping with `fmt.Errorf()`.
This allows you to attach more information to the error without requiring the
external package.

```go
...

func (db *DB) getTansactionByID(tranID string) (Transaction, error) {
	sql := `SELECT * FROM transaction WHERE id = $1;`

	var t Transaction
	if err := db.conn.Get(&t, sql, tranID); err != nil {
		return Transaction{}, err // wrapcheck error: error returned from external package is unwrapped
	}

	return t, nil
}

...
```
