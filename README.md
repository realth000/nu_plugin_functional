# nu_plugin_functional

Functional-like commands for nushell.

WIP: **This plugin is still work in process**

Tested with `nushell==0.114.0`

## Build and install

This plugin requires:

* `nushell`
* `rust` preferred lastest stable version.

To build:

```nu
# Release build
cargo build -r

# Debug build
cargo build
```

To install, follow the [official instruction for third-party plugins](https://www.nushell.sh/book/plugins.html#third-party-plugins).

## Commands

### other

Use another value if input is `null`.

```nu
$ null | fp other 100
100

$ 1 | fp other 100
1

$ let foo = 100; null | fp other {|| $foo + 2}
102
```

### first-where

Get the first element in `list/table/range` that meets a given condition.

Return null if no element meets the condition.

```nu
$ [1, 2, 4] | fp first-where $it > 5
<output nothing>

$ [1, 2, 4, 8] | fp first-where {|x| $x > 5}
8
```

### is

Check if input type is a specified type.

```nu
$ 1 | fp is int
true

$ [1, 2] | fp is list
true

$ [1, 2] | fp is list<string>
false
```

### then

Do something if input is not `null`.

```nu
$ 100 | fp then {$in + 2}
102

$ let foo = 2; 1 | fp then {|x| $x + $foo}
3

$ null | fp then 100
<output nothing>

$ [1, 2, 4, 8] | fp first-where $it > 5 | fp then {$in * 2}
16
```

### pure

Convert a value to null if it is the default value of its type.

For the following types:

* int: 0
* float: 0.0, Nan, Inf, -Inf
* string: (empty string) '', ""
* boolean: false
* duration 0day, 0hr, 0min, 0sec
* fize-size: 0b, 0kb, 0mb, 0gb, 0tb, 0pb, 0eb
* list: []
* record: {}
* table: [{}]
* nothing: null

values will be converted to null.

These yypes are not supported:

* date
* range
* binary
* closure
* block
* any
* glob
* error

```nu
$ 0 | fp pure | describe
nothing

$ false | fp pure | describe
nothing

$ [] | fp pure | describe
nothing
```

### handle

A combination of `then` and `other`: do something when input is good and do some other thing if input is bad.

"bad" means the input is null.

```nu
$ 100 | fp handle {$in + 200} {200}
300

$ null | fp handle {$in + 200} {200}
200

$ [1,2,3] | fp pure | fp handle {|x| $x | each {$in + 10} } {[200,300,400]}
[11,12,13]

$ [] | fp pure | fp handle {|x| $x | each {$in + 10} } {[200,300,400]}
[200,300,400]
```
