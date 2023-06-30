// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};
use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    switchmodule::SwitchReadingProfile,
};
use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

impl OpenFMBExt for SwitchReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        if !self.switch_reading.is_empty() {
            return Ok(self
                .switch_reading
                .first()
                .as_ref()
                .context(NoSwitchReadingSnafu)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxuSnafu)?
                .w
                .as_ref()
                .context(NoWSnafu)?
                .net
                .as_ref()
                .context(NoNetSnafu)?
                .c_val
                .as_ref()
                .context(NoCValSnafu)?
                .mag
                .to_string());
        }
        Err(OpenFMBError::InvalidOpenFMBMessage)
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("SwitchReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .protected_switch
                .as_ref()
                .context(NoProtectedSwitchSnafu)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .protected_switch
            .as_ref()
            .context(NoProtectedSwitchSnafu)?
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

impl OpenFMBExtReading for SwitchReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

pub trait SwitchReadingExt: ReadingProfileExt {
    fn w_net(&self) -> OpenFMBResult<f64>;
    fn w_net_load_side(&self) -> OpenFMBResult<f64>;

    fn q_net(&self) -> OpenFMBResult<f64>;
    fn q_net_load_side(&self) -> OpenFMBResult<f64>;

    fn v_net(&self) -> OpenFMBResult<f64>;
    fn v_net_load_side(&self) -> OpenFMBResult<f64>;

    fn a_net(&self) -> OpenFMBResult<f64>;
    fn a_net_load_side(&self) -> OpenFMBResult<f64>;
}

impl SwitchReadingExt for SwitchReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        if !self.switch_reading.is_empty() {
            return Ok(self
                .switch_reading
                .first()
                .as_ref()
                .context(NoSwitchReadingSnafu)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxuSnafu)?
                .w
                .as_ref()
                .context(NoWSnafu)?
                .net
                .as_ref()
                .context(NoNetSnafu)?
                .c_val
                .as_ref()
                .context(NoCValSnafu)?
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn q_net(&self) -> OpenFMBResult<f64> {
        if !self.switch_reading.is_empty() {
            return Ok(self
                .switch_reading
                .first()
                .as_ref()
                .context(NoSwitchReadingSnafu)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxuSnafu)?
                .v_ar
                .as_ref()
                .context(NoWSnafu)?
                .net
                .as_ref()
                .context(NoNetSnafu)?
                .c_val
                .as_ref()
                .context(NoCValSnafu)?
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn w_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.switch_reading.is_empty() || self.switch_reading.len() < 2 {
            return Ok(self
                .switch_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReadingSnafu)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxuSnafu)?
                .w
                .as_ref()
                .context(NoWSnafu)?
                .net
                .as_ref()
                .context(NoNetSnafu)?
                .c_val
                .as_ref()
                .context(NoCValSnafu)?
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn q_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.switch_reading.is_empty() || self.switch_reading.len() < 2 {
            return Ok(self
                .switch_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReadingSnafu)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxuSnafu)?
                .v_ar
                .as_ref()
                .context(NoWSnafu)?
                .net
                .as_ref()
                .context(NoNetSnafu)?
                .c_val
                .as_ref()
                .context(NoCValSnafu)?
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn v_net(&self) -> OpenFMBResult<f64> {
        if !self.switch_reading.is_empty() {
            return Ok(self
                .switch_reading
                .first()
                .as_ref()
                .context(NoSwitchReadingSnafu)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxuSnafu)?
                .ph_v
                .as_ref()
                .context(NoValueSnafu)?
                .net
                .as_ref()
                .context(NoNetSnafu)?
                .c_val
                .as_ref()
                .context(NoCValSnafu)?
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn v_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.switch_reading.is_empty() || self.switch_reading.len() < 2 {
            return Ok(self
                .switch_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReadingSnafu)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxuSnafu)?
                .ph_v
                .as_ref()
                .context(NoValueSnafu)?
                .net
                .as_ref()
                .context(NoNetSnafu)?
                .c_val
                .as_ref()
                .context(NoCValSnafu)?
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        if !self.switch_reading.is_empty() {
            return Ok(self
                .switch_reading
                .first()
                .as_ref()
                .context(NoSwitchReadingSnafu)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxuSnafu)?
                .a
                .as_ref()
                .context(NoValueSnafu)?
                .net
                .as_ref()
                .context(NoNetSnafu)?
                .c_val
                .as_ref()
                .context(NoCValSnafu)?
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn a_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.switch_reading.is_empty() || self.switch_reading.len() < 2 {
            return Ok(self
                .switch_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReadingSnafu)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxuSnafu)?
                .a
                .as_ref()
                .context(NoValueSnafu)?
                .net
                .as_ref()
                .context(NoNetSnafu)?
                .c_val
                .as_ref()
                .context(NoCValSnafu)?
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }
}

impl ReadingProfileExt for SwitchReadingProfile {}
