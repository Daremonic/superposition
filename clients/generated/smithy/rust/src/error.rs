// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use ::aws_smithy_runtime_api::box_error::BoxError;

/// Error type returned by the client.
                pub type SdkError<E, R = ::aws_smithy_runtime_api::client::orchestrator::HttpResponse> = ::aws_smithy_runtime_api::client::result::SdkError<E, R>;
                pub use ::aws_smithy_types::error::operation::BuildError;
                pub use ::aws_smithy_runtime_api::client::result::ConnectorError;

                pub use ::aws_smithy_types::error::display::DisplayErrorContext;
                pub use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
                pub use ::aws_smithy_types::error::metadata::ErrorMetadata;

/// The given enum value failed to parse since it is not a known value.
            #[derive(Debug)]
            pub struct UnknownVariantError {
                value: ::std::string::String,
            }
            impl UnknownVariantError {
                pub(crate) fn new(value: impl ::std::convert::Into<::std::string::String>) -> Self {
                    Self { value: value.into() }
                }
            }
            impl ::std::fmt::Display for UnknownVariantError {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
                    write!(f, "unknown enum variant: '{}'", self.value)
                }
            }
            impl ::std::error::Error for UnknownVariantError {}

pub(crate) mod sealed_unhandled;

