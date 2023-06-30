// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    solarmodule::SolarReadingProfile,
};

use crate::ReadingProfileExt;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading};

impl OpenFMBExt for SolarReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(self
            .solar_reading
            .as_ref()
            .context(NoSolarReadingSnafu)?
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
            .to_string())
    }

    fn message_info(&self) -> Result<&MessageInfo, OpenFMBError> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("SolarReadingProfile".to_string())
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

impl OpenFMBExtReading for SolarReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

pub trait SolarReadingExt: ReadingProfileExt {
    fn w_net(&self) -> OpenFMBResult<f64>;
    fn q_net(&self) -> OpenFMBResult<f64>;
}

impl SolarReadingExt for SolarReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .solar_reading
            .as_ref()
            .context(NoSolarReadingSnafu)?
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
            .mag)
    }

    fn q_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .solar_reading
            .as_ref()
            .context(NoSolarReadingSnafu)?
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
            .mag)
    }
}

impl ReadingProfileExt for SolarReadingProfile {}
