// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use openfmb_messages::commonmodule::EngGridConnectModeKind;
use std::str::FromStr;

use essmodule::EssStatusProfile;
use openfmb_messages::{
    commonmodule::{MessageInfo, StateKind, StatusMessageInfo},
    *,
};

use crate::StatusProfileExt;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus};

impl OpenFMBExt for EssStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .ess_status
            .as_ref()
            .context(NoEssStatusSnafu)?
            .ess_status_zgen
            .as_ref()
            .context(NoEssStatusSnafu)?
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
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("ESSStatusProfile".to_string())
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

impl OpenFMBExtStatus for EssStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

pub trait EssStatusExt: StatusProfileExt {
    fn ess_soc(&self) -> OpenFMBResult<f64>;
    fn ess_mode(&self) -> OpenFMBResult<EngGridConnectModeKind>;
    fn ess_state(&self) -> OpenFMBResult<StateKind>;
    fn ess_gn_sync_st(&self) -> OpenFMBResult<bool>;
}

impl EssStatusExt for EssStatusProfile {
    fn ess_soc(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_status
            .as_ref()
            .context(NoEssStatusSnafu)?
            .ess_status_zbat
            .as_ref()
            .context(NoEssStatusZBatSnafu)?
            .soc
            .as_ref()
            .context(NoSocSnafu)?
            .mag
            / 100.0)
    }

    fn ess_mode(&self) -> OpenFMBResult<EngGridConnectModeKind> {
        Ok(self
            .ess_status
            .as_ref()
            .context(NoEssStatusSnafu)?
            .ess_status_zgen
            .as_ref()
            .context(NoEssStatusZGenSnafu)?
            .e_ss_event_and_status_zgen
            .as_ref()
            .context(NoEssEventAndStatusZGenSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .mode
            .clone()
            .context(NoModeSnafu)?)
    }

    fn ess_state(&self) -> OpenFMBResult<StateKind> {
        Ok(self
            .ess_status
            .as_ref()
            .context(NoEssStatusSnafu)?
            .ess_status_zgen
            .as_ref()
            .context(NoEssStatusZGenSnafu)?
            .e_ss_event_and_status_zgen
            .as_ref()
            .context(NoEssEventAndStatusZGenSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .state
            .as_ref()
            .context(NoStateSnafu)?
            .value())
    }

    fn ess_gn_sync_st(&self) -> OpenFMBResult<bool> {
        Ok(self
            .ess_status
            .as_ref()
            .context(NoEssStatusSnafu)?
            .ess_status_zgen
            .as_ref()
            .context(NoEssStatusZGenSnafu)?
            .e_ss_event_and_status_zgen
            .as_ref()
            .context(NoEssEventAndStatusZGenSnafu)?
            .gn_syn_st
            .as_ref()
            .context(NoEssGnSyncStSnafu)?
            .st_val)
    }
}

impl StatusProfileExt for EssStatusProfile {}
