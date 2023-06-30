// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use essmodule::EssEventProfile;
use openfmb_messages::{commonmodule::*, *};

use crate::{error::*, OpenFMBExt, OpenFMBExtEvent};

impl OpenFMBExtEvent for EssEventProfile {
    fn event_message_info(&self) -> OpenFMBResult<&EventMessageInfo> {
        Ok(self
            .event_message_info
            .as_ref()
            .context(NoEventMessageInfoSnafu)?)
    }
}

impl OpenFMBExt for EssEventProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .ess_event
            .as_ref()
            .context(NoEssEventSnafu)?
            .ess_event_zgen
            .as_ref()
            .context(NoEssEventZGenSnafu)?
            .e_ss_event_and_status_zgen
            .as_ref()
            .context(NoEssEventAndStatusZGenSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .state
            .as_ref()
            .context(NoStateSnafu)
        {
            Ok(state) => match state.value {
                0 => Ok("Undefined".into()),
                1 => Ok("Off".into()),
                2 => Ok("On".into()),
                3 => Ok("StandBy".into()),
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
        Ok("EssEventProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .ess
                .as_ref()
                .context(NoEssSnafu)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok("".to_string())
    }
}
