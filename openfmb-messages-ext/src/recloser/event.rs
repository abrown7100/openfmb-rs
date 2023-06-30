// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use reclosermodule::RecloserEventProfile;

use crate::{error::*, OpenFMBExt, OpenFMBExtEvent};

impl OpenFMBExtEvent for RecloserEventProfile {
    fn event_message_info(&self) -> OpenFMBResult<&EventMessageInfo> {
        Ok(self
            .event_message_info
            .as_ref()
            .context(NoEventMessageInfoSnafu)?)
    }
}

impl OpenFMBExt for RecloserEventProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .recloser_event
            .as_ref()
            .context(NoRecloserEventSnafu)?
            .status_and_event_xcbr
            .as_ref()
            .context(NoStatusAndEventXcbrSnafu)?
            .pos
            .as_ref()
            .context(NoPosSnafu)?
            .phs3
            .as_ref()
            .context(NoPhs3Snafu)
        {
            Ok(phs3) => match phs3.st_val {
                0 => Ok("Undefined".into()),
                1 => Ok("Transient".into()),
                2 => Ok("Closed".into()),
                3 => Ok("Open".into()),
                4 => Ok("Invalid".into()),
                _ => Err(OpenFMBError::InvalidValue),
            },
            Err(_) => Err(OpenFMBError::InvalidOpenFMBMessage),
        }
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .event_message_info
            .as_ref()
            .context(NoEventMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("RecloserEventProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .recloser
                .as_ref()
                .context(NoRecloserSnafu)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .recloser
            .as_ref()
            .context(NoRecloserSnafu)?
            .conducting_equipment
            .as_ref()
            .context(NoConductingEquipmentSnafu)?
            .named_object
            .as_ref()
            .context(NoNamedObjectSnafu)?
            .name
            .clone()
            .context(NoNameSnafu)?)
    }
}
