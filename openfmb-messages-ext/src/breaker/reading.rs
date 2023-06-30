// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use breakermodule::BreakerReadingProfile;
use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    *,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExt for BreakerReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        if !self.breaker_reading.is_empty() {
            return Ok(self
                .breaker_reading
                .first()
                .as_ref()
                .context(NoBreakerReadingSnafu)?
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
            .context(NoReadingMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("BreakerReadingProfile".to_string())
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

impl OpenFMBExtReading for BreakerReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

pub trait BreakerReadingExt: ReadingProfileExt {
    fn w_net(&self) -> OpenFMBResult<f64>;
    fn w_net_load_side(&self) -> OpenFMBResult<f64>;

    fn q_net(&self) -> OpenFMBResult<f64>;
    fn q_net_load_side(&self) -> OpenFMBResult<f64>;

    fn v_net(&self) -> OpenFMBResult<f64>;
    fn v_net_load_side(&self) -> OpenFMBResult<f64>;

    fn a_net(&self) -> OpenFMBResult<f64>;
    fn a_net_load_side(&self) -> OpenFMBResult<f64>;
}

impl BreakerReadingExt for BreakerReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() {
            return Ok(self
                .breaker_reading
                .first()
                .as_ref()
                .context(NoBreakerReadingSnafu)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn w_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() || self.breaker_reading.len() < 2 {
            return Ok(self
                .breaker_reading
                .get(1)
                .as_ref()
                .context(NoBreakerReadingSnafu)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn q_net(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() {
            return Ok(self
                .breaker_reading
                .first()
                .as_ref()
                .context(NoBreakerReadingSnafu)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn q_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() || self.breaker_reading.len() < 2 {
            return Ok(self
                .breaker_reading
                .get(1)
                .as_ref()
                .context(NoBreakerReadingSnafu)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn v_net(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() {
            return Ok(self
                .breaker_reading
                .first()
                .as_ref()
                .context(NoBreakerReadingSnafu)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn v_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() || self.breaker_reading.len() < 2 {
            return Ok(self
                .breaker_reading
                .get(1)
                .as_ref()
                .context(NoBreakerReadingSnafu)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() {
            return Ok(self
                .breaker_reading
                .first()
                .as_ref()
                .context(NoBreakerReadingSnafu)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn a_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() || self.breaker_reading.len() < 2 {
            return Ok(self
                .breaker_reading
                .get(1)
                .as_ref()
                .context(NoBreakerReadingSnafu)?
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
        Err(OpenFMBError::NoBreakerReading)
    }
}

impl ReadingProfileExt for BreakerReadingProfile {}
