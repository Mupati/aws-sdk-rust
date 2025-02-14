// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`DisableControlInput`](crate::input::DisableControlInput).
pub mod disable_control_input {

    /// A builder for [`DisableControlInput`](crate::input::DisableControlInput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) control_identifier: std::option::Option<std::string::String>,
        pub(crate) target_identifier: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
        pub fn control_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.control_identifier = Some(input.into());
            self
        }
        /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
        pub fn set_control_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.control_identifier = input;
            self
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn target_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.target_identifier = Some(input.into());
            self
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn set_target_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.target_identifier = input;
            self
        }
        /// Consumes the builder and constructs a [`DisableControlInput`](crate::input::DisableControlInput).
        pub fn build(
            self,
        ) -> Result<crate::input::DisableControlInput, aws_smithy_http::operation::error::BuildError>
        {
            Ok(crate::input::DisableControlInput {
                control_identifier: self.control_identifier,
                target_identifier: self.target_identifier,
            })
        }
    }
}
impl DisableControlInput {
    /// Consumes the builder and constructs an Operation<[`DisableControl`](crate::operation::DisableControl)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::DisableControl,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::DisableControlInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/disable-control").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::DisableControlInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_disable_control(&self)?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
        request
            .properties_mut()
            .insert::<aws_smithy_http::endpoint::Result>(
                _config.endpoint_resolver.resolve_endpoint(&endpoint_params),
            );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::DisableControl::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "DisableControl",
            "controltower",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`DisableControlInput`](crate::input::DisableControlInput).
    pub fn builder() -> crate::input::disable_control_input::Builder {
        crate::input::disable_control_input::Builder::default()
    }
}

/// See [`EnableControlInput`](crate::input::EnableControlInput).
pub mod enable_control_input {

    /// A builder for [`EnableControlInput`](crate::input::EnableControlInput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) control_identifier: std::option::Option<std::string::String>,
        pub(crate) target_identifier: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
        pub fn control_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.control_identifier = Some(input.into());
            self
        }
        /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
        pub fn set_control_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.control_identifier = input;
            self
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn target_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.target_identifier = Some(input.into());
            self
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn set_target_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.target_identifier = input;
            self
        }
        /// Consumes the builder and constructs a [`EnableControlInput`](crate::input::EnableControlInput).
        pub fn build(
            self,
        ) -> Result<crate::input::EnableControlInput, aws_smithy_http::operation::error::BuildError>
        {
            Ok(crate::input::EnableControlInput {
                control_identifier: self.control_identifier,
                target_identifier: self.target_identifier,
            })
        }
    }
}
impl EnableControlInput {
    /// Consumes the builder and constructs an Operation<[`EnableControl`](crate::operation::EnableControl)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::EnableControl,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::EnableControlInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/enable-control").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::EnableControlInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_enable_control(&self)?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
        request
            .properties_mut()
            .insert::<aws_smithy_http::endpoint::Result>(
                _config.endpoint_resolver.resolve_endpoint(&endpoint_params),
            );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::EnableControl::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "EnableControl",
            "controltower",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`EnableControlInput`](crate::input::EnableControlInput).
    pub fn builder() -> crate::input::enable_control_input::Builder {
        crate::input::enable_control_input::Builder::default()
    }
}

/// See [`GetControlOperationInput`](crate::input::GetControlOperationInput).
pub mod get_control_operation_input {

    /// A builder for [`GetControlOperationInput`](crate::input::GetControlOperationInput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) operation_identifier: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
        pub fn operation_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.operation_identifier = Some(input.into());
            self
        }
        /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
        pub fn set_operation_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.operation_identifier = input;
            self
        }
        /// Consumes the builder and constructs a [`GetControlOperationInput`](crate::input::GetControlOperationInput).
        pub fn build(
            self,
        ) -> Result<
            crate::input::GetControlOperationInput,
            aws_smithy_http::operation::error::BuildError,
        > {
            Ok(crate::input::GetControlOperationInput {
                operation_identifier: self.operation_identifier,
            })
        }
    }
}
impl GetControlOperationInput {
    /// Consumes the builder and constructs an Operation<[`GetControlOperation`](crate::operation::GetControlOperation)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetControlOperation,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::GetControlOperationInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/get-control-operation").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::GetControlOperationInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_get_control_operation(&self)?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
        request
            .properties_mut()
            .insert::<aws_smithy_http::endpoint::Result>(
                _config.endpoint_resolver.resolve_endpoint(&endpoint_params),
            );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::GetControlOperation::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetControlOperation",
            "controltower",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`GetControlOperationInput`](crate::input::GetControlOperationInput).
    pub fn builder() -> crate::input::get_control_operation_input::Builder {
        crate::input::get_control_operation_input::Builder::default()
    }
}

/// See [`ListEnabledControlsInput`](crate::input::ListEnabledControlsInput).
pub mod list_enabled_controls_input {

    /// A builder for [`ListEnabledControlsInput`](crate::input::ListEnabledControlsInput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) target_identifier: std::option::Option<std::string::String>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The ARN of the organizational unit.</p>
        pub fn target_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.target_identifier = Some(input.into());
            self
        }
        /// <p>The ARN of the organizational unit.</p>
        pub fn set_target_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.target_identifier = input;
            self
        }
        /// <p>The token to continue the list from a previous API call with the same parameters.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to continue the list from a previous API call with the same parameters.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>How many results to return per API call.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        /// <p>How many results to return per API call.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// Consumes the builder and constructs a [`ListEnabledControlsInput`](crate::input::ListEnabledControlsInput).
        pub fn build(
            self,
        ) -> Result<
            crate::input::ListEnabledControlsInput,
            aws_smithy_http::operation::error::BuildError,
        > {
            Ok(crate::input::ListEnabledControlsInput {
                target_identifier: self.target_identifier,
                next_token: self.next_token,
                max_results: self.max_results,
            })
        }
    }
}
impl ListEnabledControlsInput {
    /// Consumes the builder and constructs an Operation<[`ListEnabledControls`](crate::operation::ListEnabledControls)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::ListEnabledControls,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::ListEnabledControlsInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/list-enabled-controls").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::ListEnabledControlsInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_list_enabled_controls(&self)?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
        request
            .properties_mut()
            .insert::<aws_smithy_http::endpoint::Result>(
                _config.endpoint_resolver.resolve_endpoint(&endpoint_params),
            );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::ListEnabledControls::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "ListEnabledControls",
            "controltower",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`ListEnabledControlsInput`](crate::input::ListEnabledControlsInput).
    pub fn builder() -> crate::input::list_enabled_controls_input::Builder {
        crate::input::list_enabled_controls_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListEnabledControlsInput {
    /// <p>The ARN of the organizational unit.</p>
    #[doc(hidden)]
    pub target_identifier: std::option::Option<std::string::String>,
    /// <p>The token to continue the list from a previous API call with the same parameters.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>How many results to return per API call.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
}
impl ListEnabledControlsInput {
    /// <p>The ARN of the organizational unit.</p>
    pub fn target_identifier(&self) -> std::option::Option<&str> {
        self.target_identifier.as_deref()
    }
    /// <p>The token to continue the list from a previous API call with the same parameters.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>How many results to return per API call.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetControlOperationInput {
    /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
    #[doc(hidden)]
    pub operation_identifier: std::option::Option<std::string::String>,
}
impl GetControlOperationInput {
    /// <p>The ID of the asynchronous operation, which is used to track status. The operation is available for 90 days.</p>
    pub fn operation_identifier(&self) -> std::option::Option<&str> {
        self.operation_identifier.as_deref()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct EnableControlInput {
    /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    #[doc(hidden)]
    pub control_identifier: std::option::Option<std::string::String>,
    /// <p>The ARN of the organizational unit.</p>
    #[doc(hidden)]
    pub target_identifier: std::option::Option<std::string::String>,
}
impl EnableControlInput {
    /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    pub fn control_identifier(&self) -> std::option::Option<&str> {
        self.control_identifier.as_deref()
    }
    /// <p>The ARN of the organizational unit.</p>
    pub fn target_identifier(&self) -> std::option::Option<&str> {
        self.target_identifier.as_deref()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DisableControlInput {
    /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    #[doc(hidden)]
    pub control_identifier: std::option::Option<std::string::String>,
    /// <p>The ARN of the organizational unit.</p>
    #[doc(hidden)]
    pub target_identifier: std::option::Option<std::string::String>,
}
impl DisableControlInput {
    /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    pub fn control_identifier(&self) -> std::option::Option<&str> {
        self.control_identifier.as_deref()
    }
    /// <p>The ARN of the organizational unit.</p>
    pub fn target_identifier(&self) -> std::option::Option<&str> {
        self.target_identifier.as_deref()
    }
}
