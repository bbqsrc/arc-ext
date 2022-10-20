# arc-ext

Extensions for `Arc<T>` such as field projection.

## Usage

The `ArcExt` trait implementation extends `Arc<T>` with `.project` and `.project_option` methods.

The projection enforces lifetimes, so that no reference to it can outlive the projection (and therefore is not unsound).

See the following example:

```rust
use arc_ext::ArcExt;

#[derive(Debug, PartialEq, Eq)]
struct Top {
    nested: Nested,
    string: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Nested {
    a: u32,
    b: Box<[u8]>,
    c: Option<Arc<Top>>,
}

fn test() {
    let top = Arc::new(Top {
        nested: Nested {
            a: 32,
            b: vec![1, 2, 3, 4].into_boxed_slice(),
            c: Some(Arc::new(Top {
                nested: Nested {
                    a: 12,
                    b: vec![99].into_boxed_slice(),
                    c: None,
                },
                string: "nested".to_string(),
            })),
        },
        string: "owned str".to_string(),
    });

    let project = top.clone().project(|x| &x.nested.b);
    assert_eq!(&[1, 2, 3, 4], &**project);
    drop(project);

    let project = top.clone().project_option(|x| x.nested.c.as_ref());
    let opt = project.as_option().unwrap();
    assert_eq!(top.nested.c.as_ref().unwrap(), opt);
}

```

## License

This project is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.