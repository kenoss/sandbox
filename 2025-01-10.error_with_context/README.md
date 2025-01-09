Context: https://x.com/tnishinaga/status/1875431872534401299

```
$ cargo run --example hoge
[examples/hoge.rs:113:9] hoge_app::main(i) = Err(
    HogeLib {
        source: Hoge {
            source: Os {
                code: 2,
                kind: NotFound,
                message: "No such file or directory",
            },
            context: ErrorContext {
                location: Location {
                    file: "examples/hoge.rs",
                    line: 20,
                    col: 22,
                },
            },
        },
        context: ErrorContext {
            location: Location {
                file: "examples/hoge.rs",
                line: 79,
                col: 26,
            },
        },
    },
)
[examples/hoge.rs:113:9] hoge_app::main(i) = Ok(
    (),
)
[examples/hoge.rs:113:9] hoge_app::main(i) = Err(
    HogeLib {
        source: Fuga {
            source: Os {
                code: 2,
                kind: NotFound,
                message: "No such file or directory",
            },
            context: ErrorContext {
                location: Location {
                    file: "examples/hoge.rs",
                    line: 36,
                    col: 22,
                },
            },
        },
        context: ErrorContext {
            location: Location {
                file: "examples/hoge.rs",
                line: 79,
                col: 26,
            },
        },
    },
)
[examples/hoge.rs:113:9] hoge_app::main(i) = Err(
    Mine {
        source: Os {
            code: 2,
            kind: NotFound,
            message: "No such file or directory",
        },
        context: ErrorContext {
            location: Location {
                file: "examples/hoge.rs",
                line: 100,
                col: 30,
            },
        },
    },
)
```
