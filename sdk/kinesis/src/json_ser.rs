// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_tags_to_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddTagsToStreamInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.stream_name {
        object.key("StreamName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tags {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Tags").start_object();
        for (key_4, value_5) in var_2 {
            {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStreamInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_6) = &input.stream_name {
        object.key("StreamName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.shard_count {
        object.key("ShardCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.stream_mode_details {
        #[allow(unused_mut)]
        let mut object_9 = object.key("StreamModeDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_mode_details(&mut object_9, var_8)?;
        object_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_decrease_stream_retention_period_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DecreaseStreamRetentionPeriodInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.stream_name {
        object.key("StreamName").string(var_10.as_str());
    }
    if let Some(var_11) = &input.retention_period_hours {
        object.key("RetentionPeriodHours").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteStreamInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.stream_name {
        object.key("StreamName").string(var_12.as_str());
    }
    if let Some(var_13) = &input.enforce_consumer_deletion {
        object.key("EnforceConsumerDeletion").boolean(*var_13);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_deregister_stream_consumer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeregisterStreamConsumerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_14) = &input.stream_arn {
        object.key("StreamARN").string(var_14.as_str());
    }
    if let Some(var_15) = &input.consumer_name {
        object.key("ConsumerName").string(var_15.as_str());
    }
    if let Some(var_16) = &input.consumer_arn {
        object.key("ConsumerARN").string(var_16.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeStreamInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_17) = &input.stream_name {
        object.key("StreamName").string(var_17.as_str());
    }
    if let Some(var_18) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_18).into()),
        );
    }
    if let Some(var_19) = &input.exclusive_start_shard_id {
        object.key("ExclusiveStartShardId").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_stream_consumer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeStreamConsumerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.stream_arn {
        object.key("StreamARN").string(var_20.as_str());
    }
    if let Some(var_21) = &input.consumer_name {
        object.key("ConsumerName").string(var_21.as_str());
    }
    if let Some(var_22) = &input.consumer_arn {
        object.key("ConsumerARN").string(var_22.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_stream_summary_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeStreamSummaryInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_23) = &input.stream_name {
        object.key("StreamName").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disable_enhanced_monitoring_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisableEnhancedMonitoringInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.stream_name {
        object.key("StreamName").string(var_24.as_str());
    }
    if let Some(var_25) = &input.shard_level_metrics {
        let mut array_26 = object.key("ShardLevelMetrics").start_array();
        for item_27 in var_25 {
            {
                array_26.value().string(item_27.as_str());
            }
        }
        array_26.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_enable_enhanced_monitoring_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::EnableEnhancedMonitoringInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.stream_name {
        object.key("StreamName").string(var_28.as_str());
    }
    if let Some(var_29) = &input.shard_level_metrics {
        let mut array_30 = object.key("ShardLevelMetrics").start_array();
        for item_31 in var_29 {
            {
                array_30.value().string(item_31.as_str());
            }
        }
        array_30.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_records_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRecordsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.shard_iterator {
        object.key("ShardIterator").string(var_32.as_str());
    }
    if let Some(var_33) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_33).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_shard_iterator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetShardIteratorInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_34) = &input.stream_name {
        object.key("StreamName").string(var_34.as_str());
    }
    if let Some(var_35) = &input.shard_id {
        object.key("ShardId").string(var_35.as_str());
    }
    if let Some(var_36) = &input.shard_iterator_type {
        object.key("ShardIteratorType").string(var_36.as_str());
    }
    if let Some(var_37) = &input.starting_sequence_number {
        object.key("StartingSequenceNumber").string(var_37.as_str());
    }
    if let Some(var_38) = &input.timestamp {
        object
            .key("Timestamp")
            .date_time(var_38, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_input_increase_stream_retention_period_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::IncreaseStreamRetentionPeriodInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_39) = &input.stream_name {
        object.key("StreamName").string(var_39.as_str());
    }
    if let Some(var_40) = &input.retention_period_hours {
        object.key("RetentionPeriodHours").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_40).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_shards_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListShardsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.stream_name {
        object.key("StreamName").string(var_41.as_str());
    }
    if let Some(var_42) = &input.next_token {
        object.key("NextToken").string(var_42.as_str());
    }
    if let Some(var_43) = &input.exclusive_start_shard_id {
        object.key("ExclusiveStartShardId").string(var_43.as_str());
    }
    if let Some(var_44) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_44).into()),
        );
    }
    if let Some(var_45) = &input.stream_creation_timestamp {
        object
            .key("StreamCreationTimestamp")
            .date_time(var_45, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_46) = &input.shard_filter {
        #[allow(unused_mut)]
        let mut object_47 = object.key("ShardFilter").start_object();
        crate::json_ser::serialize_structure_crate_model_shard_filter(&mut object_47, var_46)?;
        object_47.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_stream_consumers_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListStreamConsumersInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_48) = &input.stream_arn {
        object.key("StreamARN").string(var_48.as_str());
    }
    if let Some(var_49) = &input.next_token {
        object.key("NextToken").string(var_49.as_str());
    }
    if let Some(var_50) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_50).into()),
        );
    }
    if let Some(var_51) = &input.stream_creation_timestamp {
        object
            .key("StreamCreationTimestamp")
            .date_time(var_51, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_streams_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListStreamsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_52) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_52).into()),
        );
    }
    if let Some(var_53) = &input.exclusive_start_stream_name {
        object
            .key("ExclusiveStartStreamName")
            .string(var_53.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForStreamInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_54) = &input.stream_name {
        object.key("StreamName").string(var_54.as_str());
    }
    if let Some(var_55) = &input.exclusive_start_tag_key {
        object.key("ExclusiveStartTagKey").string(var_55.as_str());
    }
    if let Some(var_56) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_56).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_merge_shards_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::MergeShardsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_57) = &input.stream_name {
        object.key("StreamName").string(var_57.as_str());
    }
    if let Some(var_58) = &input.shard_to_merge {
        object.key("ShardToMerge").string(var_58.as_str());
    }
    if let Some(var_59) = &input.adjacent_shard_to_merge {
        object.key("AdjacentShardToMerge").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_record_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutRecordInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_60) = &input.stream_name {
        object.key("StreamName").string(var_60.as_str());
    }
    if let Some(var_61) = &input.data {
        object
            .key("Data")
            .string_unchecked(&aws_smithy_types::base64::encode(var_61));
    }
    if let Some(var_62) = &input.partition_key {
        object.key("PartitionKey").string(var_62.as_str());
    }
    if let Some(var_63) = &input.explicit_hash_key {
        object.key("ExplicitHashKey").string(var_63.as_str());
    }
    if let Some(var_64) = &input.sequence_number_for_ordering {
        object
            .key("SequenceNumberForOrdering")
            .string(var_64.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_records_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutRecordsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_65) = &input.records {
        let mut array_66 = object.key("Records").start_array();
        for item_67 in var_65 {
            {
                #[allow(unused_mut)]
                let mut object_68 = array_66.value().start_object();
                crate::json_ser::serialize_structure_crate_model_put_records_request_entry(
                    &mut object_68,
                    item_67,
                )?;
                object_68.finish();
            }
        }
        array_66.finish();
    }
    if let Some(var_69) = &input.stream_name {
        object.key("StreamName").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_stream_consumer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterStreamConsumerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.stream_arn {
        object.key("StreamARN").string(var_70.as_str());
    }
    if let Some(var_71) = &input.consumer_name {
        object.key("ConsumerName").string(var_71.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_tags_from_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveTagsFromStreamInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_72) = &input.stream_name {
        object.key("StreamName").string(var_72.as_str());
    }
    if let Some(var_73) = &input.tag_keys {
        let mut array_74 = object.key("TagKeys").start_array();
        for item_75 in var_73 {
            {
                array_74.value().string(item_75.as_str());
            }
        }
        array_74.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_split_shard_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SplitShardInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.stream_name {
        object.key("StreamName").string(var_76.as_str());
    }
    if let Some(var_77) = &input.shard_to_split {
        object.key("ShardToSplit").string(var_77.as_str());
    }
    if let Some(var_78) = &input.new_starting_hash_key {
        object.key("NewStartingHashKey").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_stream_encryption_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartStreamEncryptionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_79) = &input.stream_name {
        object.key("StreamName").string(var_79.as_str());
    }
    if let Some(var_80) = &input.encryption_type {
        object.key("EncryptionType").string(var_80.as_str());
    }
    if let Some(var_81) = &input.key_id {
        object.key("KeyId").string(var_81.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_stream_encryption_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopStreamEncryptionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_82) = &input.stream_name {
        object.key("StreamName").string(var_82.as_str());
    }
    if let Some(var_83) = &input.encryption_type {
        object.key("EncryptionType").string(var_83.as_str());
    }
    if let Some(var_84) = &input.key_id {
        object.key("KeyId").string(var_84.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_shard_count_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateShardCountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_85) = &input.stream_name {
        object.key("StreamName").string(var_85.as_str());
    }
    if let Some(var_86) = &input.target_shard_count {
        object.key("TargetShardCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_86).into()),
        );
    }
    if let Some(var_87) = &input.scaling_type {
        object.key("ScalingType").string(var_87.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_stream_mode_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateStreamModeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_88) = &input.stream_arn {
        object.key("StreamARN").string(var_88.as_str());
    }
    if let Some(var_89) = &input.stream_mode_details {
        #[allow(unused_mut)]
        let mut object_90 = object.key("StreamModeDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_mode_details(
            &mut object_90,
            var_89,
        )?;
        object_90.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_mode_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StreamModeDetails,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_91) = &input.stream_mode {
        object.key("StreamMode").string(var_91.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_shard_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ShardFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_92) = &input.r#type {
        object.key("Type").string(var_92.as_str());
    }
    if let Some(var_93) = &input.shard_id {
        object.key("ShardId").string(var_93.as_str());
    }
    if let Some(var_94) = &input.timestamp {
        object
            .key("Timestamp")
            .date_time(var_94, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_put_records_request_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PutRecordsRequestEntry,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_95) = &input.data {
        object
            .key("Data")
            .string_unchecked(&aws_smithy_types::base64::encode(var_95));
    }
    if let Some(var_96) = &input.explicit_hash_key {
        object.key("ExplicitHashKey").string(var_96.as_str());
    }
    if let Some(var_97) = &input.partition_key {
        object.key("PartitionKey").string(var_97.as_str());
    }
    Ok(())
}
