use std::{pin::Pin, sync::Arc};

mod arc;
mod option;

#[cfg(feature = "async-graphql")]
pub mod graphql;

pub use arc::ArcProject;
pub use option::ArcProjectOption;

pub trait ArcExt<T: ?Sized>: Unpin
where
    T: Unpin,
{
    fn project<'a, 'b, U, F>(self, deref_fn: F) -> ArcProject<'a, 'b, T, U>
    where
        'a: 'b,
        T: 'a,
        U: 'b,
        F: Fn(&'a Pin<Arc<T>>) -> &'b U;

    fn project_option<'a, 'b, U, F>(self, deref_fn: F) -> ArcProjectOption<'a, 'b, T, U>
    where
        'a: 'b,
        T: 'a,
        U: 'b,
        F: Fn(&'a Pin<Arc<T>>) -> Option<&'b U>;
}

impl<T: ?Sized + Unpin> ArcExt<T> for Arc<T> {
    fn project<'a, 'b, U, F>(self, deref_fn: F) -> ArcProject<'a, 'b, T, U>
    where
        'a: 'b,
        T: 'a,
        U: 'b,
        F: Fn(&'a Pin<Arc<T>>) -> &'b U,
    {
        ArcProject::new(self, deref_fn)
    }

    fn project_option<'a, 'b, U, F>(self, deref_fn: F) -> ArcProjectOption<'a, 'b, T, U>
    where
        'a: 'b,
        T: 'a,
        U: 'b,
        F: Fn(&'a Pin<Arc<T>>) -> Option<&'b U>,
    {
        ArcProjectOption::new(self, deref_fn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
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
}
