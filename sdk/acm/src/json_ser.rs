// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_tags_to_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddTagsToCertificateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.certificate_arn {
        object.key("CertificateArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tags {
        let mut array_3 = object.key("Tags").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteCertificateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_6) = &input.certificate_arn {
        object.key("CertificateArn").string(var_6.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeCertificateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.certificate_arn {
        object.key("CertificateArn").string(var_7.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_export_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExportCertificateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_8) = &input.certificate_arn {
        object.key("CertificateArn").string(var_8.as_str());
    }
    if let Some(var_9) = &input.passphrase {
        object
            .key("Passphrase")
            .string_unchecked(&aws_smithy_types::base64::encode(var_9));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCertificateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.certificate_arn {
        object.key("CertificateArn").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_import_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportCertificateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_11) = &input.certificate_arn {
        object.key("CertificateArn").string(var_11.as_str());
    }
    if let Some(var_12) = &input.certificate {
        object
            .key("Certificate")
            .string_unchecked(&aws_smithy_types::base64::encode(var_12));
    }
    if let Some(var_13) = &input.private_key {
        object
            .key("PrivateKey")
            .string_unchecked(&aws_smithy_types::base64::encode(var_13));
    }
    if let Some(var_14) = &input.certificate_chain {
        object
            .key("CertificateChain")
            .string_unchecked(&aws_smithy_types::base64::encode(var_14));
    }
    if let Some(var_15) = &input.tags {
        let mut array_16 = object.key("Tags").start_array();
        for item_17 in var_15 {
            {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_certificates_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCertificatesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.certificate_statuses {
        let mut array_20 = object.key("CertificateStatuses").start_array();
        for item_21 in var_19 {
            {
                array_20.value().string(item_21.as_str());
            }
        }
        array_20.finish();
    }
    if let Some(var_22) = &input.includes {
        #[allow(unused_mut)]
        let mut object_23 = object.key("Includes").start_object();
        crate::json_ser::serialize_structure_crate_model_filters(&mut object_23, var_22)?;
        object_23.finish();
    }
    if let Some(var_24) = &input.next_token {
        object.key("NextToken").string(var_24.as_str());
    }
    if let Some(var_25) = &input.max_items {
        object.key("MaxItems").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_25).into()),
        );
    }
    if let Some(var_26) = &input.sort_by {
        object.key("SortBy").string(var_26.as_str());
    }
    if let Some(var_27) = &input.sort_order {
        object.key("SortOrder").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForCertificateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.certificate_arn {
        object.key("CertificateArn").string(var_28.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_account_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutAccountConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_29) = &input.expiry_events {
        #[allow(unused_mut)]
        let mut object_30 = object.key("ExpiryEvents").start_object();
        crate::json_ser::serialize_structure_crate_model_expiry_events_configuration(
            &mut object_30,
            var_29,
        )?;
        object_30.finish();
    }
    if let Some(var_31) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_tags_from_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveTagsFromCertificateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.certificate_arn {
        object.key("CertificateArn").string(var_32.as_str());
    }
    if let Some(var_33) = &input.tags {
        let mut array_34 = object.key("Tags").start_array();
        for item_35 in var_33 {
            {
                #[allow(unused_mut)]
                let mut object_36 = array_34.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_36, item_35)?;
                object_36.finish();
            }
        }
        array_34.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_renew_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RenewCertificateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_37) = &input.certificate_arn {
        object.key("CertificateArn").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_request_certificate_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RequestCertificateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.domain_name {
        object.key("DomainName").string(var_38.as_str());
    }
    if let Some(var_39) = &input.validation_method {
        object.key("ValidationMethod").string(var_39.as_str());
    }
    if let Some(var_40) = &input.subject_alternative_names {
        let mut array_41 = object.key("SubjectAlternativeNames").start_array();
        for item_42 in var_40 {
            {
                array_41.value().string(item_42.as_str());
            }
        }
        array_41.finish();
    }
    if let Some(var_43) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_43.as_str());
    }
    if let Some(var_44) = &input.domain_validation_options {
        let mut array_45 = object.key("DomainValidationOptions").start_array();
        for item_46 in var_44 {
            {
                #[allow(unused_mut)]
                let mut object_47 = array_45.value().start_object();
                crate::json_ser::serialize_structure_crate_model_domain_validation_option(
                    &mut object_47,
                    item_46,
                )?;
                object_47.finish();
            }
        }
        array_45.finish();
    }
    if let Some(var_48) = &input.options {
        #[allow(unused_mut)]
        let mut object_49 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_certificate_options(
            &mut object_49,
            var_48,
        )?;
        object_49.finish();
    }
    if let Some(var_50) = &input.certificate_authority_arn {
        object
            .key("CertificateAuthorityArn")
            .string(var_50.as_str());
    }
    if let Some(var_51) = &input.tags {
        let mut array_52 = object.key("Tags").start_array();
        for item_53 in var_51 {
            {
                #[allow(unused_mut)]
                let mut object_54 = array_52.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_54, item_53)?;
                object_54.finish();
            }
        }
        array_52.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_resend_validation_email_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResendValidationEmailInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_55) = &input.certificate_arn {
        object.key("CertificateArn").string(var_55.as_str());
    }
    if let Some(var_56) = &input.domain {
        object.key("Domain").string(var_56.as_str());
    }
    if let Some(var_57) = &input.validation_domain {
        object.key("ValidationDomain").string(var_57.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_certificate_options_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCertificateOptionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_58) = &input.certificate_arn {
        object.key("CertificateArn").string(var_58.as_str());
    }
    if let Some(var_59) = &input.options {
        #[allow(unused_mut)]
        let mut object_60 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_certificate_options(
            &mut object_60,
            var_59,
        )?;
        object_60.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_61) = &input.key {
        object.key("Key").string(var_61.as_str());
    }
    if let Some(var_62) = &input.value {
        object.key("Value").string(var_62.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filters,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.extended_key_usage {
        let mut array_64 = object.key("extendedKeyUsage").start_array();
        for item_65 in var_63 {
            {
                array_64.value().string(item_65.as_str());
            }
        }
        array_64.finish();
    }
    if let Some(var_66) = &input.key_usage {
        let mut array_67 = object.key("keyUsage").start_array();
        for item_68 in var_66 {
            {
                array_67.value().string(item_68.as_str());
            }
        }
        array_67.finish();
    }
    if let Some(var_69) = &input.key_types {
        let mut array_70 = object.key("keyTypes").start_array();
        for item_71 in var_69 {
            {
                array_70.value().string(item_71.as_str());
            }
        }
        array_70.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_expiry_events_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExpiryEventsConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_72) = &input.days_before_expiry {
        object.key("DaysBeforeExpiry").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_72).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_domain_validation_option(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DomainValidationOption,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_73) = &input.domain_name {
        object.key("DomainName").string(var_73.as_str());
    }
    if let Some(var_74) = &input.validation_domain {
        object.key("ValidationDomain").string(var_74.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_certificate_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CertificateOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_75) = &input.certificate_transparency_logging_preference {
        object
            .key("CertificateTransparencyLoggingPreference")
            .string(var_75.as_str());
    }
    Ok(())
}
