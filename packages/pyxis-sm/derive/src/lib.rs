use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DataEnum, DeriveInput};

/// Merges the variants of two enums.
///
/// Adapted from DAO DAO:
/// https://github.com/DA0-DA0/dao-contracts/blob/74bd3881fdd86829e5e8b132b9952dd64f2d0737/packages/dao-macros/src/lib.rs#L9
fn merge_variants(metadata: TokenStream, left: TokenStream, right: TokenStream) -> TokenStream {
    use syn::Data::Enum;

    // parse metadata
    let args = parse_macro_input!(metadata as AttributeArgs);
    if let Some(first_arg) = args.first() {
        return syn::Error::new_spanned(first_arg, "macro takes no arguments")
            .to_compile_error()
            .into();
    }

    // parse the left enum
    let mut left: DeriveInput = parse_macro_input!(left);
    let Enum(DataEnum { variants, .. }) = &mut left.data else {
        return syn::Error::new(left.ident.span(), "only enums can accept variants")
            .to_compile_error()
            .into();
    };

    // parse the right enum
    let right: DeriveInput = parse_macro_input!(right);
    let Enum(DataEnum {
        variants: to_add, ..
    }) = right.data
    else {
        return syn::Error::new(left.ident.span(), "only enums can provide variants")
            .to_compile_error()
            .into();
    };

    // insert variants from the right to the left
    variants.extend(to_add.into_iter());

    quote! { #left }.into()
}

/// Append pyxis-sm plugin execute message variant(s) to an enum.
///
/// For example, apply the `base_plugin_execute` macro to the following enum:
///
/// ```rust
/// use cosmwasm_schema::cw_serde;
/// use pyxis_sm::base_plugin_execute;
///
/// #[base_plugin_execute]
/// #[cw_serde]
/// enum ExecuteMsg {
///     Foo {},
///     Bar {},
/// }
/// ```
///
/// Is equivalent to:
///
/// ```rust
/// use cosmwasm_schema::cw_serde;
///
/// #[cw_serde]
/// enum ExecuteMsg {
///     Register {config:String},
///     ...
///     AfterExecute {
///         msgs: Vec<::pyxis_sm::msg::SdkMsg>,
///         call_info: ::pyxis_sm::msg::CallInfo,
///         is_authz: bool,
///         },
///     Foo {},
///     Bar {},
/// }
/// ```
///
/// Note: `#[base_plugin_execute]` must be applied _before_ `#[cw_serde]`.
#[proc_macro_attribute]
pub fn base_plugin_execute(metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        metadata,
        input,
        quote! {
            enum Right {
                /// Register a plugin to this smart account, the caller must be the smart account itself
                Register { config: String },
                /// Unregister a plugin from this smart account, the caller must be the smart account itself
                Unregister {},
                /// PreExecute is called before a transaction is executed
                PreExecute {
                    msgs: Vec<::pyxis_sm::msg::SdkMsg>,
                    call_info: ::pyxis_sm::msg::CallInfo,
                    authz_info: ::pyxis_sm::msg::AuthzInfo,
                },
                /// AfterExecute is called at the end of a transaction
                AfterExecute {
                    msgs: Vec<::pyxis_sm::msg::SdkMsg>,
                    call_info: ::pyxis_sm::msg::CallInfo,
                    authz_info: ::pyxis_sm::msg::AuthzInfo,
                },
            }
        }
        .into(),
    )
}

/// Append pyxis-sm recovery plugin execute message variant(s) to an enum.
#[proc_macro_attribute]
pub fn recovery_plugin_execute(metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        metadata,
        input,
        quote! {
            enum Right {
                /// Register a plugin to this smart account, the caller must be the smart account itself
                Register {
                    config: String,
                },
                /// Unregister a plugin from this smart account, the caller must be the smart account itself
                Unregister {},
                /// Validate smart account recovery action
                Recover {
                    caller: String,
                    pub_key: cosmwasm_std::Binary,
                    credentials: cosmwasm_std::Binary,
                },
            }
        }
        .into(),
    )
}
