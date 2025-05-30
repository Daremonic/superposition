// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::bulk_operation::_bulk_operation_output::BulkOperationOutputBuilder;

pub use crate::operation::bulk_operation::_bulk_operation_input::BulkOperationInputBuilder;

impl crate::operation::bulk_operation::builders::BulkOperationInputBuilder {
                    /// Sends a request with this input using the given client.
                    pub async fn send_with(self, client: &crate::Client) -> ::std::result::Result<
                        crate::operation::bulk_operation::BulkOperationOutput,
                        ::aws_smithy_runtime_api::client::result::SdkError<
                            crate::operation::bulk_operation::BulkOperationError,
                            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse
                        >
                    > {
                        let mut fluent_builder = client.bulk_operation();
                        fluent_builder.inner = self;
                        fluent_builder.send().await
                    }
                }
/// Fluent builder constructing a request to `BulkOperation`.
/// 
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BulkOperationFluentBuilder {
                handle: ::std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::bulk_operation::builders::BulkOperationInputBuilder,
config_override: ::std::option::Option<crate::config::Builder>,
            }
impl
                crate::client::customize::internal::CustomizableSend<
                    crate::operation::bulk_operation::BulkOperationOutput,
                    crate::operation::bulk_operation::BulkOperationError,
                > for BulkOperationFluentBuilder
            {
                fn send(
                    self,
                    config_override: crate::config::Builder,
                ) -> crate::client::customize::internal::BoxFuture<
                    crate::client::customize::internal::SendResult<
                        crate::operation::bulk_operation::BulkOperationOutput,
                        crate::operation::bulk_operation::BulkOperationError,
                    >,
                > {
                    ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
                }
            }
impl BulkOperationFluentBuilder {
    /// Creates a new `BulkOperationFluentBuilder`.
                    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
                        Self {
                            handle,
                            inner: ::std::default::Default::default(),
    config_override: ::std::option::Option::None,
                        }
                    }
    /// Access the BulkOperation as a reference.
                    pub fn as_input(&self) -> &crate::operation::bulk_operation::builders::BulkOperationInputBuilder {
                        &self.inner
                    }
    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> ::std::result::Result<crate::operation::bulk_operation::BulkOperationOutput, ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::bulk_operation::BulkOperationError, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>> {
                        let input = self.inner.build().map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
                        let runtime_plugins = crate::operation::bulk_operation::BulkOperation::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
                        crate::operation::bulk_operation::BulkOperation::orchestrate(&runtime_plugins, input).await
                    }
    
                    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
                    pub fn customize(
                        self,
                    ) -> crate::client::customize::CustomizableOperation<crate::operation::bulk_operation::BulkOperationOutput, crate::operation::bulk_operation::BulkOperationError, Self> {
                        crate::client::customize::CustomizableOperation::new(self)
                    }
    pub(crate) fn config_override(
                            mut self,
                            config_override: impl ::std::convert::Into<crate::config::Builder>,
                        ) -> Self {
                            self.set_config_override(::std::option::Option::Some(config_override.into()));
                            self
                        }
    
                        pub(crate) fn set_config_override(
                            &mut self,
                            config_override: ::std::option::Option<crate::config::Builder>,
                        ) -> &mut Self {
                            self.config_override = config_override;
                            self
                        }
    #[allow(missing_docs)] // documentation missing in model
    pub fn workspace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
                    self.inner = self.inner.workspace_id(input.into());
                    self
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_workspace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
                    self.inner = self.inner.set_workspace_id(input);
                    self
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_workspace_id(&self) -> &::std::option::Option<::std::string::String> {
                    self.inner.get_workspace_id()
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn org_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
                    self.inner = self.inner.org_id(input.into());
                    self
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_org_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
                    self.inner = self.inner.set_org_id(input);
                    self
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_org_id(&self) -> &::std::option::Option<::std::string::String> {
                    self.inner.get_org_id()
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn config_tags(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
                    self.inner = self.inner.config_tags(input.into());
                    self
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_config_tags(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
                    self.inner = self.inner.set_config_tags(input);
                    self
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_config_tags(&self) -> &::std::option::Option<::std::string::String> {
                    self.inner.get_config_tags()
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn bulk_operation(mut self, input: crate::types::BulkOperationReq) -> Self {
                    self.inner = self.inner.bulk_operation(input);
                    self
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_bulk_operation(mut self, input: ::std::option::Option<crate::types::BulkOperationReq>) -> Self {
                    self.inner = self.inner.set_bulk_operation(input);
                    self
                }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_bulk_operation(&self) -> &::std::option::Option<crate::types::BulkOperationReq> {
                    self.inner.get_bulk_operation()
                }
}

