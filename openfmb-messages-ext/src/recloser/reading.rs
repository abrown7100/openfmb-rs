// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use reclosermodule::RecloserReadingProfile;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExt for RecloserReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
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
        Ok("RecloserReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .recloser
                .as_ref()
                .context(NoRecloserSnafu)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .recloser
            .as_ref()
            .context(NoRecloserSnafu)?
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

impl OpenFMBExtReading for RecloserReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

pub trait RecloserReadingExt: ReadingProfileExt {
    fn w_net(&self) -> OpenFMBResult<f64>;
    fn w_net_load_side(&self) -> OpenFMBResult<f64>;

    fn q_net(&self) -> OpenFMBResult<f64>;
    fn q_net_load_side(&self) -> OpenFMBResult<f64>;

    fn v_net(&self) -> OpenFMBResult<f64>;
    fn v_net_load_side(&self) -> OpenFMBResult<f64>;

    fn a_net(&self) -> OpenFMBResult<f64>;
    fn a_net_load_side(&self) -> OpenFMBResult<f64>;
}

impl RecloserReadingExt for RecloserReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() {
            return Ok(self
                .recloser_reading
                .first()
                .as_ref()
                .context(NoRecloserReadingSnafu)?
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
        Err(OpenFMBError::NoRecloserReading)
    }

    fn q_net(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() {
            return Ok(self
                .recloser_reading
                .first()
                .as_ref()
                .context(NoRecloserReadingSnafu)?
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
        Err(OpenFMBError::NoRecloserReading)
    }

    fn w_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() || self.recloser_reading.len() < 2 {
            return Ok(self
                .recloser_reading
                .get(1)
                .as_ref()
                .context(NoRecloserReadingSnafu)?
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
        Err(OpenFMBError::NoRecloserReading)
    }

    fn q_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() || self.recloser_reading.len() < 2 {
            return Ok(self
                .recloser_reading
                .get(1)
                .as_ref()
                .context(NoRecloserReadingSnafu)?
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
        Err(OpenFMBError::NoRecloserReading)
    }

    fn v_net(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() {
            return Ok(self
                .recloser_reading
                .first()
                .as_ref()
                .context(NoRecloserReadingSnafu)?
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
        Err(OpenFMBError::NoRecloserReading)
    }

    fn v_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() || self.recloser_reading.len() < 2 {
            return Ok(self
                .recloser_reading
                .get(1)
                .as_ref()
                .context(NoRecloserReadingSnafu)?
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
        Err(OpenFMBError::NoRecloserReading)
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() {
            return Ok(self
                .recloser_reading
                .first()
                .as_ref()
                .context(NoRecloserReadingSnafu)?
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
        Err(OpenFMBError::NoRecloserReading)
    }

    fn a_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() || self.recloser_reading.len() < 2 {
            return Ok(self
                .recloser_reading
                .get(1)
                .as_ref()
                .context(NoRecloserReadingSnafu)?
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
        Err(OpenFMBError::NoRecloserReading)
    }
}

impl ReadingProfileExt for RecloserReadingProfile {}
