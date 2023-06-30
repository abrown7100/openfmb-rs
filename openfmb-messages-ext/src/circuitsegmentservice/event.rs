// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use circuitsegmentservicemodule::CircuitSegmentEventProfile;
use openfmb_messages::{commonmodule::*, *};

use crate::{error::*, OpenFMBExt, OpenFMBExtEvent};

impl OpenFMBExtEvent for CircuitSegmentEventProfile {
    fn event_message_info(&self) -> OpenFMBResult<&EventMessageInfo> {
        Ok(self
            .event_message_info
            .as_ref()
            .context(NoEventMessageInfoSnafu)?)
    }
}

impl OpenFMBExt for CircuitSegmentEventProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
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
        Ok("CircuitSegmentEventProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .application_system
                .as_ref()
                .context(NoApplicationSystemSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .application_system
            .as_ref()
            .context(NoApplicationSystemSnafu)?
            .named_object
            .as_ref()
            .context(NoNamedObjectSnafu)?
            .name
            .clone()
            .context(NoNameSnafu)?)
    }
}
