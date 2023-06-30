// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use capbankmodule::CapBankStatusProfile;
use openfmb_messages::{commonmodule::*, *};

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus, Phase, Position, StatusProfileExt};

impl OpenFMBExtStatus for CapBankStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

impl OpenFMBExt for CapBankStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
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
        Ok("CapBankStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .cap_bank_system
                .as_ref()
                .context(NoCapBankSystemSnafu)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .cap_bank_system
            .as_ref()
            .context(NoCapBankSystemSnafu)?
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

impl Position for CapBankStatusProfile {
    fn pos(&self) -> OpenFMBResult<DbPosKind> {
        self.pos_per_phase(Phase::Phs3)
    }

    fn pos_per_phase(&self, phase: Phase) -> OpenFMBResult<DbPosKind> {
        let st_val = match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .pos
                    .as_ref()
                    .context(NoPosSnafu)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3Snafu)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .pos
                    .as_ref()
                    .context(NoPosSnafu)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsASnafu)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .pos
                    .as_ref()
                    .context(NoPosSnafu)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsBSnafu)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
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

pub trait CapBankStatusExt: StatusProfileExt {
    fn temp_lmt(&self, phase: Phase) -> OpenFMBResult<bool>;
    fn var_lmt(&self, phase: Phase) -> OpenFMBResult<bool>;
    fn vol_lmt(&self, phase: Phase) -> OpenFMBResult<bool>;
    fn amp_lmt(&self, phase: Phase) -> OpenFMBResult<bool>;
    fn dir_rev(&self, phase: Phase) -> OpenFMBResult<bool>;
    fn ctl_mode(&self) -> OpenFMBResult<ControlModeKind>;
}

impl CapBankStatusExt for CapBankStatusProfile {
    fn temp_lmt(&self, phase: Phase) -> OpenFMBResult<bool> {
        Ok(match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .temp_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3Snafu)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .temp_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsASnafu)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .temp_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsBSnafu)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .temp_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsCSnafu)?
                    .st_val
            }
        })
    }

    fn var_lmt(&self, phase: Phase) -> OpenFMBResult<bool> {
        Ok(match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .v_ar_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3Snafu)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .v_ar_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsASnafu)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .v_ar_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsBSnafu)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .v_ar_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsCSnafu)?
                    .st_val
            }
        })
    }

    fn vol_lmt(&self, phase: Phase) -> OpenFMBResult<bool> {
        Ok(match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .vol_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3Snafu)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .vol_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsASnafu)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .vol_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsBSnafu)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .vol_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsCSnafu)?
                    .st_val
            }
        })
    }

    fn amp_lmt(&self, phase: Phase) -> OpenFMBResult<bool> {
        Ok(match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .amp_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3Snafu)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .amp_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsASnafu)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .amp_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsBSnafu)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .amp_lmt
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsCSnafu)?
                    .st_val
            }
        })
    }

    fn dir_rev(&self, phase: Phase) -> OpenFMBResult<bool> {
        Ok(match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .dir_rev
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3Snafu)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .dir_rev
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsASnafu)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .dir_rev
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsBSnafu)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatusSnafu)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpshSnafu)?
                    .dir_rev
                    .as_ref()
                    .context(NoValueSnafu)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsCSnafu)?
                    .st_val
            }
        })
    }

    fn ctl_mode(&self) -> OpenFMBResult<ControlModeKind> {
        Ok(
            match self
                .cap_bank_status
                .as_ref()
                .context(NoCapBankStatusSnafu)?
                .cap_bank_event_and_status_ypsh
                .as_ref()
                .context(NoCapBankEventAndStatusYpshSnafu)?
                .ctl_mode
                .as_ref()
                .context(NoValueSnafu)?
                .value
            {
                1 => ControlModeKind::Auto,
                2 => ControlModeKind::Manual,
                3 => ControlModeKind::Override,
                4 => ControlModeKind::Remote,
                _ => ControlModeKind::Undefined,
            },
        )
    }
}

impl StatusProfileExt for CapBankStatusProfile {}
