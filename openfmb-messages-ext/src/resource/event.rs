// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use resourcemodule::ResourceEventProfile;

use crate::{error::*, OpenFMBExt, OpenFMBExtEvent};

impl OpenFMBExtEvent for ResourceEventProfile {
    fn event_message_info(&self) -> OpenFMBResult<&EventMessageInfo> {
        Ok(self
            .event_message_info
            .as_ref()
            .context(NoEventMessageInfo)?)
    }
}

impl OpenFMBExt for ResourceEventProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .event_message_info
            .as_ref()
            .context(NoEventMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("ResourceEventProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .conducting_equipment
            .as_ref()
            .context(NoConductingEquipment)?
            .named_object
            .as_ref()
            .context(NoNamedObject)?
            .name
            .clone()
            .context(NoName)?)
    }
}
