// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_get_latest_configuration_get_latest_configuration_output_configuration(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<aws_smithy_types::Blob>,
    crate::error::GetLatestConfigurationError,
> {
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}

pub(crate) fn deser_header_get_latest_configuration_get_latest_configuration_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_get_latest_configuration_get_latest_configuration_output_next_poll_configuration_token(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Next-Poll-Configuration-Token").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_get_latest_configuration_get_latest_configuration_output_next_poll_interval_in_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Next-Poll-Interval-In-Seconds").iter();
    let var_1 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_1.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_1.len()
        )))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}
