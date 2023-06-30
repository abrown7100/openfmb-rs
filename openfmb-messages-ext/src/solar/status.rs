// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus, StatusProfileExt};
use openfmb_messages::commonmodule::StateKind;
use openfmb_messages::{
    commonmodule::{MessageInfo, StatusMessageInfo},
    solarmodule::SolarStatusProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

impl OpenFMBExt for SolarStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .solar_status
            .as_ref()
            .context(NoSolarStatusSnafu)?
            .solar_status_zgen
            .as_ref()
            .context(NoSolarStatusZGenSnafu)?
            .solar_event_and_status_zgen
            .as_ref()
            .context(NoSolarEventAndStatusZGenSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .state
            .as_ref()
            .context(NoStateSnafu)
        {
            Ok(state) => match state.value {
                0 => Ok("Undefined".to_string()),
                1 => Ok("Off".to_string()),
                2 => Ok("On".to_string()),
                3 => Ok("StandBy".to_string()),
                _ => Err(OpenFMBError::InvalidValue),
            },
            Err(_) => Err(OpenFMBError::InvalidOpenFMBMessage),
        }
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("SolarStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .solar_inverter
                .as_ref()
                .context(NoSolarInverterSnafu)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .solar_inverter
            .as_ref()
            .context(NoSolarInverterSnafu)?
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

impl OpenFMBExtStatus for SolarStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

pub trait SolarStatusExt: StatusProfileExt {
    fn solar_state(&self) -> OpenFMBResult<StateKind>;
    fn pct_v_droop(&self) -> OpenFMBResult<Option<f32>>;
}

impl SolarStatusExt for SolarStatusProfile {
    fn solar_state(&self) -> OpenFMBResult<StateKind> {
        Ok(self
            .solar_status
            .as_ref()
            .context(NoSolarStatusSnafu)?
            .solar_status_zgen
            .as_ref()
            .context(NoSolarStatusZGenSnafu)?
            .solar_event_and_status_zgen
            .as_ref()
            .context(NoSolarEventAndStatusZGenSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .state
            .as_ref()
            .context(NoStateSnafu)?
            .value())
    }

    fn pct_v_droop(&self) -> OpenFMBResult<Option<f32>> {
        Ok(self
            .solar_status
            .as_ref()
            .context(NoSolarStatusSnafu)?
            .solar_status_zgen
            .as_ref()
            .context(NoSolarStatusZGenSnafu)?
            .solar_event_and_status_zgen
            .as_ref()
            .context(NoSolarEventAndStatusZGenSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .pct_v_droop)
    }
}

impl StatusProfileExt for SolarStatusProfile {}
