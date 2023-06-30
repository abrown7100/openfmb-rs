// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use regulatormodule::RegulatorReadingProfile;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt, Side};

impl OpenFMBExtReading for RegulatorReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoReadingMessageInfoSnafu)?)
    }
}

impl OpenFMBExt for RegulatorReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoReadingMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("RegulatorReadingProfile".to_string())
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

pub trait RegulatorReadingExt: ReadingProfileExt {
    fn w_net(&self, side: Side) -> OpenFMBResult<f64>;
    fn q_net(&self, side: Side) -> OpenFMBResult<f64>;
    fn v_net(&self, side: Side) -> OpenFMBResult<f64>;
    fn a_net(&self, side: Side) -> OpenFMBResult<f64>;
}

impl RegulatorReadingExt for RegulatorReadingProfile {
    fn w_net(&self, side: Side) -> OpenFMBResult<f64> {
        let index = side as usize;
        if !self.regulator_reading.is_empty() && self.regulator_reading.len() > index {
            return Ok(self
                .regulator_reading
                .get(index)
                .as_ref()
                .context(NoRegulatorReadingSnafu)?
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
        Err(OpenFMBError::NoRegulatorReading)
    }

    fn q_net(&self, side: Side) -> OpenFMBResult<f64> {
        let index = side as usize;
        if !self.regulator_reading.is_empty() && self.regulator_reading.len() > index {
            return Ok(self
                .regulator_reading
                .get(index)
                .as_ref()
                .context(NoRegulatorReadingSnafu)?
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
        Err(OpenFMBError::NoRegulatorReading)
    }

    fn v_net(&self, side: Side) -> OpenFMBResult<f64> {
        let index = side as usize;
        if !self.regulator_reading.is_empty() && self.regulator_reading.len() > index {
            return Ok(self
                .regulator_reading
                .first()
                .as_ref()
                .context(NoRegulatorReadingSnafu)?
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
        Err(OpenFMBError::NoRegulatorReading)
    }

    fn a_net(&self, side: Side) -> OpenFMBResult<f64> {
        let index = side as usize;
        if !self.regulator_reading.is_empty() && self.regulator_reading.len() > index {
            return Ok(self
                .regulator_reading
                .first()
                .as_ref()
                .context(NoRegulatorReadingSnafu)?
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
        Err(OpenFMBError::NoRegulatorReading)
    }
}

impl ReadingProfileExt for RegulatorReadingProfile {}
