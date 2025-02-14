// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_suite_definition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSuiteDefinitionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.suite_definition_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("suiteDefinitionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_suite_definition_configuration(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    if let Some(var_3) = &input.tags {
        #[allow(unused_mut)]
        let mut object_4 = object.key("tags").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_suite_run_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartSuiteRunInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.suite_definition_version {
        object.key("suiteDefinitionVersion").string(var_7.as_str());
    }
    if let Some(var_8) = &input.suite_run_configuration {
        #[allow(unused_mut)]
        let mut object_9 = object.key("suiteRunConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_suite_run_configuration(
            &mut object_9,
            var_8,
        )?;
        object_9.finish();
    }
    if let Some(var_10) = &input.tags {
        #[allow(unused_mut)]
        let mut object_11 = object.key("tags").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_14) = &input.tags {
        #[allow(unused_mut)]
        let mut object_15 = object.key("tags").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16.as_str()).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_suite_definition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSuiteDefinitionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.suite_definition_configuration {
        #[allow(unused_mut)]
        let mut object_19 = object.key("suiteDefinitionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_suite_definition_configuration(
            &mut object_19,
            var_18,
        )?;
        object_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_suite_definition_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SuiteDefinitionConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.suite_definition_name {
        object.key("suiteDefinitionName").string(var_20.as_str());
    }
    if let Some(var_21) = &input.devices {
        let mut array_22 = object.key("devices").start_array();
        for item_23 in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_device_under_test(
                    &mut object_24,
                    item_23,
                )?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if input.intended_for_qualification {
        object
            .key("intendedForQualification")
            .boolean(input.intended_for_qualification);
    }
    if input.is_long_duration_test {
        object
            .key("isLongDurationTest")
            .boolean(input.is_long_duration_test);
    }
    if let Some(var_25) = &input.root_group {
        object.key("rootGroup").string(var_25.as_str());
    }
    if let Some(var_26) = &input.device_permission_role_arn {
        object
            .key("devicePermissionRoleArn")
            .string(var_26.as_str());
    }
    if let Some(var_27) = &input.protocol {
        object.key("protocol").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_suite_run_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SuiteRunConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.primary_device {
        #[allow(unused_mut)]
        let mut object_29 = object.key("primaryDevice").start_object();
        crate::json_ser::serialize_structure_crate_model_device_under_test(&mut object_29, var_28)?;
        object_29.finish();
    }
    if let Some(var_30) = &input.selected_test_list {
        let mut array_31 = object.key("selectedTestList").start_array();
        for item_32 in var_30 {
            {
                array_31.value().string(item_32.as_str());
            }
        }
        array_31.finish();
    }
    if input.parallel_run {
        object.key("parallelRun").boolean(input.parallel_run);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_device_under_test(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeviceUnderTest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_33) = &input.thing_arn {
        object.key("thingArn").string(var_33.as_str());
    }
    if let Some(var_34) = &input.certificate_arn {
        object.key("certificateArn").string(var_34.as_str());
    }
    Ok(())
}
