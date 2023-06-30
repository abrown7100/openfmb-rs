// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    generationmodule::GenerationReadingProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExt for GenerationReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(self
            .generation_reading
            .as_ref()
            .context(NoGenerationReadingSnafu)?
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
        Ok("GenerationReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .generating_unit
                .as_ref()
                .context(NoGeneratingUnitSnafu)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .generating_unit
            .as_ref()
            .context(NoGeneratingUnitSnafu)?
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

impl OpenFMBExtReading for GenerationReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

pub trait GenerationReadingExt: ReadingProfileExt {
    fn w_net(&self) -> OpenFMBResult<f64>;

    fn q_net(&self) -> OpenFMBResult<f64>;
}

impl GenerationReadingExt for GenerationReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .generation_reading
            .as_ref()
            .context(NoGenerationReadingSnafu)?
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
            .generation_reading
            .as_ref()
            .context(NoGenerationReadingSnafu)?
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

impl ReadingProfileExt for GenerationReadingProfile {}
