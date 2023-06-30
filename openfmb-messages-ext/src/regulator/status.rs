// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use regulatormodule::RegulatorStatusProfile;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus, StatusProfileExt};

impl OpenFMBExtStatus for RegulatorStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

impl OpenFMBExt for RegulatorStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
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
        Ok("RegulatorStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .regulator_system
                .as_ref()
                .context(NoRegulatorSystemSnafu)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .regulator_system
            .as_ref()
            .context(NoRegulatorSystemSnafu)?
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

pub trait RegulatorStatusExt: StatusProfileExt {
    fn tap_pos_phs3(&self) -> OpenFMBResult<i32>;
    fn tap_pos_phs_a(&self) -> OpenFMBResult<i32>;
    fn tap_pos_phs_b(&self) -> OpenFMBResult<i32>;
    fn tap_pos_phs_c(&self) -> OpenFMBResult<i32>;

    fn vol_lmt_hi_phs3(&self) -> OpenFMBResult<bool>;
    fn vol_lmt_lo_phs3(&self) -> OpenFMBResult<bool>;

    fn voltage_setpoint_enabled(&self) -> OpenFMBResult<bool>;

    fn bnd_wid_hi_phs3(&self) -> OpenFMBResult<bool>;
    fn bnd_wid_lo_phs3(&self) -> OpenFMBResult<bool>;

    fn hot_line_tag(&self) -> OpenFMBResult<bool>;
    fn tap_op_err(&self) -> OpenFMBResult<bool>;

    fn state(&self) -> OpenFMBResult<StateKind>;
}

impl StatusProfileExt for RegulatorStatusProfile {}

impl RegulatorStatusExt for RegulatorStatusProfile {
    fn tap_pos_phs3(&self) -> OpenFMBResult<i32> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .tap_pos
            .as_ref()
            .context(NoTapPosSnafu)?
            .phs3
            .as_ref()
            .context(NoPhs3Snafu)?
            .st_val)
    }

    fn tap_pos_phs_a(&self) -> OpenFMBResult<i32> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .tap_pos
            .as_ref()
            .context(NoTapPosSnafu)?
            .phs_a
            .as_ref()
            .context(NoPhsASnafu)?
            .st_val)
    }

    fn tap_pos_phs_b(&self) -> OpenFMBResult<i32> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .tap_pos
            .as_ref()
            .context(NoTapPosSnafu)?
            .phs_b
            .as_ref()
            .context(NoPhsBSnafu)?
            .st_val)
    }

    fn tap_pos_phs_c(&self) -> OpenFMBResult<i32> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .tap_pos
            .as_ref()
            .context(NoTapPosSnafu)?
            .phs_c
            .as_ref()
            .context(NoPhsCSnafu)?
            .st_val)
    }

    fn vol_lmt_hi_phs3(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .vol_lmt_hi
            .as_ref()
            .context(NoVolLmtHiSnafu)?
            .phs3
            .as_ref()
            .context(NoPhs3Snafu)?
            .st_val)
    }

    fn vol_lmt_lo_phs3(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .vol_lmt_lo
            .as_ref()
            .context(NoVolLmtLoSnafu)?
            .phs3
            .as_ref()
            .context(NoPhs3Snafu)?
            .st_val)
    }

    fn bnd_wid_hi_phs3(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .bnd_wid_hi
            .as_ref()
            .context(NoBndWidHiSnafu)?
            .phs3
            .as_ref()
            .context(NoPhs3Snafu)?
            .st_val)
    }

    fn bnd_wid_lo_phs3(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .bnd_wid_lo
            .as_ref()
            .context(NoBndWidLoSnafu)?
            .phs3
            .as_ref()
            .context(NoPhs3Snafu)?
            .st_val)
    }

    fn tap_op_err(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .tap_op_err
            .as_ref()
            .context(NoTapOpErrSnafu)?
            .st_val)
    }

    fn hot_line_tag(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .logical_node_for_event_and_status
            .as_ref()
            .context(NoLogicalNodeForEventAndStatusSnafu)?
            .hot_line_tag
            .as_ref()
            .context(NoHotLineTagSnafu)?
            .st_val)
    }

    fn voltage_setpoint_enabled(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatusSnafu)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncrSnafu)?
            .point_status
            .as_ref()
            .context(NoPointStatusSnafu)?
            .voltage_set_point_enabled
            .as_ref()
            .context(NoVoltageSetPointEnabledSnafu)?
            .st_val)
    }

    fn state(&self) -> OpenFMBResult<StateKind> {
        Ok(
            match self
                .regulator_status
                .as_ref()
                .context(NoRegulatorStatusSnafu)?
                .regulator_event_and_status_ancr
                .as_ref()
                .context(NoRegulatorEventAndStatusAncrSnafu)?
                .point_status
                .as_ref()
                .context(NoPointStatusSnafu)?
                .state
                .as_ref()
                .context(NoStateSnafu)?
                .value
            {
                1 => StateKind::Off,
                2 => StateKind::On,
                3 => StateKind::Standby,
                _ => StateKind::Undefined,
            },
        )
    }
}
