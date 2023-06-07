use proc_macro::{TokenStream};
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(ToSqlJson)]
pub fn postgres_to_sql(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse_macro_input!(input);
    let name = &ast.ident;
    let tokens = quote! {

        impl ToSql for #name {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                postgres_types::Json(self).to_sql(ty, out)
            }

            postgres_types::accepts!(JSON, JSONB);

            postgres_types::to_sql_checked!();
        }
    };
    tokens.into()
}

#[proc_macro_derive(FromSqlJson)]
pub fn postgres_from_sql(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse_macro_input!(input);
    let name = &ast.ident;
    let tokens = quote! {
        impl<'a> FromSql<'a> for #name {
            fn from_sql(
                ty: &postgres_types::Type,
                raw: &[u8],
            ) -> Result<#name, Box<dyn std::error::Error + Sync + Send>> {
                postgres_types::Json::<#name>::from_sql(ty, raw).map(|json| json.0)
            }

            postgres_types::accepts!(JSON, JSONB);
        }
    };
    tokens.into()
}
