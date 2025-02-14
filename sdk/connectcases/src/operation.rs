// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchGetField`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_get_field`](crate::client::Client::batch_get_field).
///
/// See [`crate::client::fluent_builders::BatchGetField`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchGetField {
    _private: (),
}
impl BatchGetField {
    /// Creates a new builder-style object to manufacture [`BatchGetFieldInput`](crate::input::BatchGetFieldInput).
    pub fn builder() -> crate::input::batch_get_field_input::Builder {
        crate::input::batch_get_field_input::Builder::default()
    }
    /// Creates a new `BatchGetField` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchGetField {
    type Output =
        std::result::Result<crate::output::BatchGetFieldOutput, crate::error::BatchGetFieldError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_get_field_error(response)
        } else {
            crate::operation_deser::parse_batch_get_field_response(response)
        }
    }
}

/// Operation shape for `BatchPutFieldOptions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_put_field_options`](crate::client::Client::batch_put_field_options).
///
/// See [`crate::client::fluent_builders::BatchPutFieldOptions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchPutFieldOptions {
    _private: (),
}
impl BatchPutFieldOptions {
    /// Creates a new builder-style object to manufacture [`BatchPutFieldOptionsInput`](crate::input::BatchPutFieldOptionsInput).
    pub fn builder() -> crate::input::batch_put_field_options_input::Builder {
        crate::input::batch_put_field_options_input::Builder::default()
    }
    /// Creates a new `BatchPutFieldOptions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchPutFieldOptions {
    type Output = std::result::Result<
        crate::output::BatchPutFieldOptionsOutput,
        crate::error::BatchPutFieldOptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_put_field_options_error(response)
        } else {
            crate::operation_deser::parse_batch_put_field_options_response(response)
        }
    }
}

/// Operation shape for `CreateCase`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_case`](crate::client::Client::create_case).
///
/// See [`crate::client::fluent_builders::CreateCase`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateCase {
    _private: (),
}
impl CreateCase {
    /// Creates a new builder-style object to manufacture [`CreateCaseInput`](crate::input::CreateCaseInput).
    pub fn builder() -> crate::input::create_case_input::Builder {
        crate::input::create_case_input::Builder::default()
    }
    /// Creates a new `CreateCase` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCase {
    type Output =
        std::result::Result<crate::output::CreateCaseOutput, crate::error::CreateCaseError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_case_error(response)
        } else {
            crate::operation_deser::parse_create_case_response(response)
        }
    }
}

/// Operation shape for `CreateDomain`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_domain`](crate::client::Client::create_domain).
///
/// See [`crate::client::fluent_builders::CreateDomain`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateDomain {
    _private: (),
}
impl CreateDomain {
    /// Creates a new builder-style object to manufacture [`CreateDomainInput`](crate::input::CreateDomainInput).
    pub fn builder() -> crate::input::create_domain_input::Builder {
        crate::input::create_domain_input::Builder::default()
    }
    /// Creates a new `CreateDomain` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDomain {
    type Output =
        std::result::Result<crate::output::CreateDomainOutput, crate::error::CreateDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_domain_error(response)
        } else {
            crate::operation_deser::parse_create_domain_response(response)
        }
    }
}

/// Operation shape for `CreateField`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_field`](crate::client::Client::create_field).
///
/// See [`crate::client::fluent_builders::CreateField`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateField {
    _private: (),
}
impl CreateField {
    /// Creates a new builder-style object to manufacture [`CreateFieldInput`](crate::input::CreateFieldInput).
    pub fn builder() -> crate::input::create_field_input::Builder {
        crate::input::create_field_input::Builder::default()
    }
    /// Creates a new `CreateField` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateField {
    type Output =
        std::result::Result<crate::output::CreateFieldOutput, crate::error::CreateFieldError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_field_error(response)
        } else {
            crate::operation_deser::parse_create_field_response(response)
        }
    }
}

/// Operation shape for `CreateLayout`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_layout`](crate::client::Client::create_layout).
///
/// See [`crate::client::fluent_builders::CreateLayout`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateLayout {
    _private: (),
}
impl CreateLayout {
    /// Creates a new builder-style object to manufacture [`CreateLayoutInput`](crate::input::CreateLayoutInput).
    pub fn builder() -> crate::input::create_layout_input::Builder {
        crate::input::create_layout_input::Builder::default()
    }
    /// Creates a new `CreateLayout` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateLayout {
    type Output =
        std::result::Result<crate::output::CreateLayoutOutput, crate::error::CreateLayoutError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_layout_error(response)
        } else {
            crate::operation_deser::parse_create_layout_response(response)
        }
    }
}

/// Operation shape for `CreateRelatedItem`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_related_item`](crate::client::Client::create_related_item).
///
/// See [`crate::client::fluent_builders::CreateRelatedItem`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateRelatedItem {
    _private: (),
}
impl CreateRelatedItem {
    /// Creates a new builder-style object to manufacture [`CreateRelatedItemInput`](crate::input::CreateRelatedItemInput).
    pub fn builder() -> crate::input::create_related_item_input::Builder {
        crate::input::create_related_item_input::Builder::default()
    }
    /// Creates a new `CreateRelatedItem` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRelatedItem {
    type Output = std::result::Result<
        crate::output::CreateRelatedItemOutput,
        crate::error::CreateRelatedItemError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_related_item_error(response)
        } else {
            crate::operation_deser::parse_create_related_item_response(response)
        }
    }
}

/// Operation shape for `CreateTemplate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_template`](crate::client::Client::create_template).
///
/// See [`crate::client::fluent_builders::CreateTemplate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateTemplate {
    _private: (),
}
impl CreateTemplate {
    /// Creates a new builder-style object to manufacture [`CreateTemplateInput`](crate::input::CreateTemplateInput).
    pub fn builder() -> crate::input::create_template_input::Builder {
        crate::input::create_template_input::Builder::default()
    }
    /// Creates a new `CreateTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateTemplate {
    type Output =
        std::result::Result<crate::output::CreateTemplateOutput, crate::error::CreateTemplateError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_template_error(response)
        } else {
            crate::operation_deser::parse_create_template_response(response)
        }
    }
}

/// Operation shape for `GetCase`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_case`](crate::client::Client::get_case).
///
/// See [`crate::client::fluent_builders::GetCase`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetCase {
    _private: (),
}
impl GetCase {
    /// Creates a new builder-style object to manufacture [`GetCaseInput`](crate::input::GetCaseInput).
    pub fn builder() -> crate::input::get_case_input::Builder {
        crate::input::get_case_input::Builder::default()
    }
    /// Creates a new `GetCase` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCase {
    type Output = std::result::Result<crate::output::GetCaseOutput, crate::error::GetCaseError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_case_error(response)
        } else {
            crate::operation_deser::parse_get_case_response(response)
        }
    }
}

/// Operation shape for `GetCaseEventConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_case_event_configuration`](crate::client::Client::get_case_event_configuration).
///
/// See [`crate::client::fluent_builders::GetCaseEventConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetCaseEventConfiguration {
    _private: (),
}
impl GetCaseEventConfiguration {
    /// Creates a new builder-style object to manufacture [`GetCaseEventConfigurationInput`](crate::input::GetCaseEventConfigurationInput).
    pub fn builder() -> crate::input::get_case_event_configuration_input::Builder {
        crate::input::get_case_event_configuration_input::Builder::default()
    }
    /// Creates a new `GetCaseEventConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCaseEventConfiguration {
    type Output = std::result::Result<
        crate::output::GetCaseEventConfigurationOutput,
        crate::error::GetCaseEventConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_case_event_configuration_error(response)
        } else {
            crate::operation_deser::parse_get_case_event_configuration_response(response)
        }
    }
}

/// Operation shape for `GetDomain`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_domain`](crate::client::Client::get_domain).
///
/// See [`crate::client::fluent_builders::GetDomain`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetDomain {
    _private: (),
}
impl GetDomain {
    /// Creates a new builder-style object to manufacture [`GetDomainInput`](crate::input::GetDomainInput).
    pub fn builder() -> crate::input::get_domain_input::Builder {
        crate::input::get_domain_input::Builder::default()
    }
    /// Creates a new `GetDomain` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDomain {
    type Output = std::result::Result<crate::output::GetDomainOutput, crate::error::GetDomainError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_domain_error(response)
        } else {
            crate::operation_deser::parse_get_domain_response(response)
        }
    }
}

/// Operation shape for `GetLayout`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_layout`](crate::client::Client::get_layout).
///
/// See [`crate::client::fluent_builders::GetLayout`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetLayout {
    _private: (),
}
impl GetLayout {
    /// Creates a new builder-style object to manufacture [`GetLayoutInput`](crate::input::GetLayoutInput).
    pub fn builder() -> crate::input::get_layout_input::Builder {
        crate::input::get_layout_input::Builder::default()
    }
    /// Creates a new `GetLayout` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetLayout {
    type Output = std::result::Result<crate::output::GetLayoutOutput, crate::error::GetLayoutError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_layout_error(response)
        } else {
            crate::operation_deser::parse_get_layout_response(response)
        }
    }
}

/// Operation shape for `GetTemplate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_template`](crate::client::Client::get_template).
///
/// See [`crate::client::fluent_builders::GetTemplate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetTemplate {
    _private: (),
}
impl GetTemplate {
    /// Creates a new builder-style object to manufacture [`GetTemplateInput`](crate::input::GetTemplateInput).
    pub fn builder() -> crate::input::get_template_input::Builder {
        crate::input::get_template_input::Builder::default()
    }
    /// Creates a new `GetTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTemplate {
    type Output =
        std::result::Result<crate::output::GetTemplateOutput, crate::error::GetTemplateError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_template_error(response)
        } else {
            crate::operation_deser::parse_get_template_response(response)
        }
    }
}

/// Operation shape for `ListCasesForContact`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_cases_for_contact`](crate::client::Client::list_cases_for_contact).
///
/// See [`crate::client::fluent_builders::ListCasesForContact`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListCasesForContact {
    _private: (),
}
impl ListCasesForContact {
    /// Creates a new builder-style object to manufacture [`ListCasesForContactInput`](crate::input::ListCasesForContactInput).
    pub fn builder() -> crate::input::list_cases_for_contact_input::Builder {
        crate::input::list_cases_for_contact_input::Builder::default()
    }
    /// Creates a new `ListCasesForContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListCasesForContact {
    type Output = std::result::Result<
        crate::output::ListCasesForContactOutput,
        crate::error::ListCasesForContactError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_cases_for_contact_error(response)
        } else {
            crate::operation_deser::parse_list_cases_for_contact_response(response)
        }
    }
}

/// Operation shape for `ListDomains`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_domains`](crate::client::Client::list_domains).
///
/// See [`crate::client::fluent_builders::ListDomains`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListDomains {
    _private: (),
}
impl ListDomains {
    /// Creates a new builder-style object to manufacture [`ListDomainsInput`](crate::input::ListDomainsInput).
    pub fn builder() -> crate::input::list_domains_input::Builder {
        crate::input::list_domains_input::Builder::default()
    }
    /// Creates a new `ListDomains` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDomains {
    type Output =
        std::result::Result<crate::output::ListDomainsOutput, crate::error::ListDomainsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_domains_error(response)
        } else {
            crate::operation_deser::parse_list_domains_response(response)
        }
    }
}

/// Operation shape for `ListFieldOptions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_field_options`](crate::client::Client::list_field_options).
///
/// See [`crate::client::fluent_builders::ListFieldOptions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListFieldOptions {
    _private: (),
}
impl ListFieldOptions {
    /// Creates a new builder-style object to manufacture [`ListFieldOptionsInput`](crate::input::ListFieldOptionsInput).
    pub fn builder() -> crate::input::list_field_options_input::Builder {
        crate::input::list_field_options_input::Builder::default()
    }
    /// Creates a new `ListFieldOptions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListFieldOptions {
    type Output = std::result::Result<
        crate::output::ListFieldOptionsOutput,
        crate::error::ListFieldOptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_field_options_error(response)
        } else {
            crate::operation_deser::parse_list_field_options_response(response)
        }
    }
}

/// Operation shape for `ListFields`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_fields`](crate::client::Client::list_fields).
///
/// See [`crate::client::fluent_builders::ListFields`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListFields {
    _private: (),
}
impl ListFields {
    /// Creates a new builder-style object to manufacture [`ListFieldsInput`](crate::input::ListFieldsInput).
    pub fn builder() -> crate::input::list_fields_input::Builder {
        crate::input::list_fields_input::Builder::default()
    }
    /// Creates a new `ListFields` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListFields {
    type Output =
        std::result::Result<crate::output::ListFieldsOutput, crate::error::ListFieldsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_fields_error(response)
        } else {
            crate::operation_deser::parse_list_fields_response(response)
        }
    }
}

/// Operation shape for `ListLayouts`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_layouts`](crate::client::Client::list_layouts).
///
/// See [`crate::client::fluent_builders::ListLayouts`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListLayouts {
    _private: (),
}
impl ListLayouts {
    /// Creates a new builder-style object to manufacture [`ListLayoutsInput`](crate::input::ListLayoutsInput).
    pub fn builder() -> crate::input::list_layouts_input::Builder {
        crate::input::list_layouts_input::Builder::default()
    }
    /// Creates a new `ListLayouts` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListLayouts {
    type Output =
        std::result::Result<crate::output::ListLayoutsOutput, crate::error::ListLayoutsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_layouts_error(response)
        } else {
            crate::operation_deser::parse_list_layouts_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `ListTemplates`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_templates`](crate::client::Client::list_templates).
///
/// See [`crate::client::fluent_builders::ListTemplates`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTemplates {
    _private: (),
}
impl ListTemplates {
    /// Creates a new builder-style object to manufacture [`ListTemplatesInput`](crate::input::ListTemplatesInput).
    pub fn builder() -> crate::input::list_templates_input::Builder {
        crate::input::list_templates_input::Builder::default()
    }
    /// Creates a new `ListTemplates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTemplates {
    type Output =
        std::result::Result<crate::output::ListTemplatesOutput, crate::error::ListTemplatesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_templates_error(response)
        } else {
            crate::operation_deser::parse_list_templates_response(response)
        }
    }
}

/// Operation shape for `PutCaseEventConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_case_event_configuration`](crate::client::Client::put_case_event_configuration).
///
/// See [`crate::client::fluent_builders::PutCaseEventConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutCaseEventConfiguration {
    _private: (),
}
impl PutCaseEventConfiguration {
    /// Creates a new builder-style object to manufacture [`PutCaseEventConfigurationInput`](crate::input::PutCaseEventConfigurationInput).
    pub fn builder() -> crate::input::put_case_event_configuration_input::Builder {
        crate::input::put_case_event_configuration_input::Builder::default()
    }
    /// Creates a new `PutCaseEventConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutCaseEventConfiguration {
    type Output = std::result::Result<
        crate::output::PutCaseEventConfigurationOutput,
        crate::error::PutCaseEventConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_case_event_configuration_error(response)
        } else {
            crate::operation_deser::parse_put_case_event_configuration_response(response)
        }
    }
}

/// Operation shape for `SearchCases`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`search_cases`](crate::client::Client::search_cases).
///
/// See [`crate::client::fluent_builders::SearchCases`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct SearchCases {
    _private: (),
}
impl SearchCases {
    /// Creates a new builder-style object to manufacture [`SearchCasesInput`](crate::input::SearchCasesInput).
    pub fn builder() -> crate::input::search_cases_input::Builder {
        crate::input::search_cases_input::Builder::default()
    }
    /// Creates a new `SearchCases` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SearchCases {
    type Output =
        std::result::Result<crate::output::SearchCasesOutput, crate::error::SearchCasesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_search_cases_error(response)
        } else {
            crate::operation_deser::parse_search_cases_response(response)
        }
    }
}

/// Operation shape for `SearchRelatedItems`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`search_related_items`](crate::client::Client::search_related_items).
///
/// See [`crate::client::fluent_builders::SearchRelatedItems`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct SearchRelatedItems {
    _private: (),
}
impl SearchRelatedItems {
    /// Creates a new builder-style object to manufacture [`SearchRelatedItemsInput`](crate::input::SearchRelatedItemsInput).
    pub fn builder() -> crate::input::search_related_items_input::Builder {
        crate::input::search_related_items_input::Builder::default()
    }
    /// Creates a new `SearchRelatedItems` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SearchRelatedItems {
    type Output = std::result::Result<
        crate::output::SearchRelatedItemsOutput,
        crate::error::SearchRelatedItemsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_search_related_items_error(response)
        } else {
            crate::operation_deser::parse_search_related_items_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateCase`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_case`](crate::client::Client::update_case).
///
/// See [`crate::client::fluent_builders::UpdateCase`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateCase {
    _private: (),
}
impl UpdateCase {
    /// Creates a new builder-style object to manufacture [`UpdateCaseInput`](crate::input::UpdateCaseInput).
    pub fn builder() -> crate::input::update_case_input::Builder {
        crate::input::update_case_input::Builder::default()
    }
    /// Creates a new `UpdateCase` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateCase {
    type Output =
        std::result::Result<crate::output::UpdateCaseOutput, crate::error::UpdateCaseError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_case_error(response)
        } else {
            crate::operation_deser::parse_update_case_response(response)
        }
    }
}

/// Operation shape for `UpdateField`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_field`](crate::client::Client::update_field).
///
/// See [`crate::client::fluent_builders::UpdateField`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateField {
    _private: (),
}
impl UpdateField {
    /// Creates a new builder-style object to manufacture [`UpdateFieldInput`](crate::input::UpdateFieldInput).
    pub fn builder() -> crate::input::update_field_input::Builder {
        crate::input::update_field_input::Builder::default()
    }
    /// Creates a new `UpdateField` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateField {
    type Output =
        std::result::Result<crate::output::UpdateFieldOutput, crate::error::UpdateFieldError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_field_error(response)
        } else {
            crate::operation_deser::parse_update_field_response(response)
        }
    }
}

/// Operation shape for `UpdateLayout`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_layout`](crate::client::Client::update_layout).
///
/// See [`crate::client::fluent_builders::UpdateLayout`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateLayout {
    _private: (),
}
impl UpdateLayout {
    /// Creates a new builder-style object to manufacture [`UpdateLayoutInput`](crate::input::UpdateLayoutInput).
    pub fn builder() -> crate::input::update_layout_input::Builder {
        crate::input::update_layout_input::Builder::default()
    }
    /// Creates a new `UpdateLayout` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateLayout {
    type Output =
        std::result::Result<crate::output::UpdateLayoutOutput, crate::error::UpdateLayoutError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_layout_error(response)
        } else {
            crate::operation_deser::parse_update_layout_response(response)
        }
    }
}

/// Operation shape for `UpdateTemplate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_template`](crate::client::Client::update_template).
///
/// See [`crate::client::fluent_builders::UpdateTemplate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateTemplate {
    _private: (),
}
impl UpdateTemplate {
    /// Creates a new builder-style object to manufacture [`UpdateTemplateInput`](crate::input::UpdateTemplateInput).
    pub fn builder() -> crate::input::update_template_input::Builder {
        crate::input::update_template_input::Builder::default()
    }
    /// Creates a new `UpdateTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateTemplate {
    type Output =
        std::result::Result<crate::output::UpdateTemplateOutput, crate::error::UpdateTemplateError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_template_error(response)
        } else {
            crate::operation_deser::parse_update_template_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
