use async_graphql::OutputType;

use crate::{ArcProject, ArcProjectOption};

#[async_graphql::async_trait::async_trait]
impl<T: ?Sized + Unpin, U: ?Sized + OutputType> OutputType for ArcProjectOption<'_, '_, T, U> {
    fn type_name() -> std::borrow::Cow<'static, str> {
        Option::<&U>::type_name()
    }

    fn create_type_info(registry: &mut async_graphql::registry::Registry) -> String {
        Option::<&U>::create_type_info(registry)
    }

    async fn resolve(
        &self,
        ctx: &async_graphql::ContextSelectionSet<'_>,
        field: &async_graphql::Positioned<async_graphql::parser::types::Field>,
    ) -> async_graphql::ServerResult<async_graphql::Value> {
        Option::<&U>::resolve(&self.as_option(), ctx, field).await
    }
}

#[async_graphql::async_trait::async_trait]
impl<T: ?Sized + Unpin, U: OutputType> OutputType for ArcProject<'_, '_, T, U> {
    fn type_name() -> std::borrow::Cow<'static, str> {
        <&U>::type_name()
    }

    fn create_type_info(registry: &mut async_graphql::registry::Registry) -> String {
        <&U>::create_type_info(registry)
    }

    async fn resolve(
        &self,
        ctx: &async_graphql::ContextSelectionSet<'_>,
        field: &async_graphql::Positioned<async_graphql::parser::types::Field>,
    ) -> async_graphql::ServerResult<async_graphql::Value> {
        <&U>::resolve(&&**self, ctx, field).await
    }
}
