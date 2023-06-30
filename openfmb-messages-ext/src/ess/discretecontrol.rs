// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, essmodule::*};

use crate::{error::*, ControlProfileExt, OpenFMBExt};

impl OpenFMBExt for EssDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Err(OpenFMBError::NoState)
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .control_message_info
            .as_ref()
            .context(NoControlMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("EssDiscreteControlProfile".to_string())
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
        Ok(self
            .ess
            .as_ref()
            .context(NoEssSnafu)?
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

impl ControlProfileExt for EssDiscreteControlProfile {}
