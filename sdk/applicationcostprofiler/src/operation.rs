// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteReportDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_report_definition`](crate::client::Client::delete_report_definition).
///
/// See [`crate::client::fluent_builders::DeleteReportDefinition`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteReportDefinition {
    _private: (),
}
impl DeleteReportDefinition {
    /// Creates a new builder-style object to manufacture [`DeleteReportDefinitionInput`](crate::input::DeleteReportDefinitionInput).
    pub fn builder() -> crate::input::delete_report_definition_input::Builder {
        crate::input::delete_report_definition_input::Builder::default()
    }
    /// Creates a new `DeleteReportDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteReportDefinition {
    type Output = std::result::Result<
        crate::output::DeleteReportDefinitionOutput,
        crate::error::DeleteReportDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_report_definition_error(response)
        } else {
            crate::operation_deser::parse_delete_report_definition_response(response)
        }
    }
}

/// Operation shape for `GetReportDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_report_definition`](crate::client::Client::get_report_definition).
///
/// See [`crate::client::fluent_builders::GetReportDefinition`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetReportDefinition {
    _private: (),
}
impl GetReportDefinition {
    /// Creates a new builder-style object to manufacture [`GetReportDefinitionInput`](crate::input::GetReportDefinitionInput).
    pub fn builder() -> crate::input::get_report_definition_input::Builder {
        crate::input::get_report_definition_input::Builder::default()
    }
    /// Creates a new `GetReportDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetReportDefinition {
    type Output = std::result::Result<
        crate::output::GetReportDefinitionOutput,
        crate::error::GetReportDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_report_definition_error(response)
        } else {
            crate::operation_deser::parse_get_report_definition_response(response)
        }
    }
}

/// Operation shape for `ImportApplicationUsage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`import_application_usage`](crate::client::Client::import_application_usage).
///
/// See [`crate::client::fluent_builders::ImportApplicationUsage`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ImportApplicationUsage {
    _private: (),
}
impl ImportApplicationUsage {
    /// Creates a new builder-style object to manufacture [`ImportApplicationUsageInput`](crate::input::ImportApplicationUsageInput).
    pub fn builder() -> crate::input::import_application_usage_input::Builder {
        crate::input::import_application_usage_input::Builder::default()
    }
    /// Creates a new `ImportApplicationUsage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ImportApplicationUsage {
    type Output = std::result::Result<
        crate::output::ImportApplicationUsageOutput,
        crate::error::ImportApplicationUsageError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_import_application_usage_error(response)
        } else {
            crate::operation_deser::parse_import_application_usage_response(response)
        }
    }
}

/// Operation shape for `ListReportDefinitions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_report_definitions`](crate::client::Client::list_report_definitions).
///
/// See [`crate::client::fluent_builders::ListReportDefinitions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListReportDefinitions {
    _private: (),
}
impl ListReportDefinitions {
    /// Creates a new builder-style object to manufacture [`ListReportDefinitionsInput`](crate::input::ListReportDefinitionsInput).
    pub fn builder() -> crate::input::list_report_definitions_input::Builder {
        crate::input::list_report_definitions_input::Builder::default()
    }
    /// Creates a new `ListReportDefinitions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListReportDefinitions {
    type Output = std::result::Result<
        crate::output::ListReportDefinitionsOutput,
        crate::error::ListReportDefinitionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_report_definitions_error(response)
        } else {
            crate::operation_deser::parse_list_report_definitions_response(response)
        }
    }
}

/// Operation shape for `PutReportDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_report_definition`](crate::client::Client::put_report_definition).
///
/// See [`crate::client::fluent_builders::PutReportDefinition`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutReportDefinition {
    _private: (),
}
impl PutReportDefinition {
    /// Creates a new builder-style object to manufacture [`PutReportDefinitionInput`](crate::input::PutReportDefinitionInput).
    pub fn builder() -> crate::input::put_report_definition_input::Builder {
        crate::input::put_report_definition_input::Builder::default()
    }
    /// Creates a new `PutReportDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutReportDefinition {
    type Output = std::result::Result<
        crate::output::PutReportDefinitionOutput,
        crate::error::PutReportDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_report_definition_error(response)
        } else {
            crate::operation_deser::parse_put_report_definition_response(response)
        }
    }
}

/// Operation shape for `UpdateReportDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_report_definition`](crate::client::Client::update_report_definition).
///
/// See [`crate::client::fluent_builders::UpdateReportDefinition`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateReportDefinition {
    _private: (),
}
impl UpdateReportDefinition {
    /// Creates a new builder-style object to manufacture [`UpdateReportDefinitionInput`](crate::input::UpdateReportDefinitionInput).
    pub fn builder() -> crate::input::update_report_definition_input::Builder {
        crate::input::update_report_definition_input::Builder::default()
    }
    /// Creates a new `UpdateReportDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateReportDefinition {
    type Output = std::result::Result<
        crate::output::UpdateReportDefinitionOutput,
        crate::error::UpdateReportDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_report_definition_error(response)
        } else {
            crate::operation_deser::parse_update_report_definition_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
