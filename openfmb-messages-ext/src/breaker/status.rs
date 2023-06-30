// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use breakermodule::BreakerStatusProfile;
use openfmb_messages::{
    commonmodule::{DbPosKind, MessageInfo, StatusMessageInfo},
    *,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus, Phase, Position};

impl OpenFMBExtStatus for BreakerStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

impl OpenFMBExt for BreakerStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .breaker_status
            .as_ref()
            .context(NoBreakerStatusSnafu)?
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
            Ok(v) => match v.st_val {
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
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("BreakerStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .breaker
                .as_ref()
                .context(NoBreakerSnafu)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .breaker
            .as_ref()
            .context(NoBreakerSnafu)?
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

impl Position for BreakerStatusProfile {
    fn pos(&self) -> OpenFMBResult<DbPosKind> {
        self.pos_per_phase(Phase::Phs3)
    }

    fn pos_per_phase(&self, phase: Phase) -> OpenFMBResult<DbPosKind> {
        let st_val = match phase {
            Phase::Phs3 => {
                self.breaker_status
                    .as_ref()
                    .context(NoBreakerStatusSnafu)?
                    .status_and_event_xcbr
                    .as_ref()
                    .context(NoStatusAndEventXcbrSnafu)?
                    .pos
                    .as_ref()
                    .context(NoPosSnafu)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3Snafu)?
                    .st_val
            }
            Phase::PhsA => {
                self.breaker_status
                    .as_ref()
                    .context(NoBreakerStatusSnafu)?
                    .status_and_event_xcbr
                    .as_ref()
                    .context(NoStatusAndEventXcbrSnafu)?
                    .pos
                    .as_ref()
                    .context(NoPosSnafu)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsASnafu)?
                    .st_val
            }
            Phase::PhsB => {
                self.breaker_status
                    .as_ref()
                    .context(NoBreakerStatusSnafu)?
                    .status_and_event_xcbr
                    .as_ref()
                    .context(NoStatusAndEventXcbrSnafu)?
                    .pos
                    .as_ref()
                    .context(NoPosSnafu)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsBSnafu)?
                    .st_val
            }
            Phase::PhsC => {
                self.breaker_status
                    .as_ref()
                    .context(NoBreakerStatusSnafu)?
                    .status_and_event_xcbr
                    .as_ref()
                    .context(NoStatusAndEventXcbrSnafu)?
                    .pos
                    .as_ref()
                    .context(NoPosSnafu)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsCSnafu)?
                    .st_val
            }
        };

        match st_val {
            0 => Ok(DbPosKind::Undefined),
            1 => Ok(DbPosKind::Transient),
            2 => Ok(DbPosKind::Closed),
            3 => Ok(DbPosKind::Open),
            4 => Ok(DbPosKind::Invalid),
            _ => Ok(DbPosKind::Undefined),
        }
    }
}
