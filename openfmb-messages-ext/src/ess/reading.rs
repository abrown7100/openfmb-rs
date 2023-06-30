// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use openfmb_messages::{commonmodule::MessageInfo, essmodule::EssReadingProfile};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, ReadingProfileExt};

impl OpenFMBExt for EssReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".to_string())
        //panic!("{:?}", self);
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
        Ok("ESSReadingProfile".to_string())
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

pub trait EssReadingExt: ReadingProfileExt {
    fn w_net(&self) -> OpenFMBResult<f64>;

    fn q_net(&self) -> OpenFMBResult<f64>;
}

impl EssReadingExt for EssReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_reading
            .as_ref()
            .context(NoEssReadingSnafu)?
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
            .ess_reading
            .as_ref()
            .context(NoEssReadingSnafu)?
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

impl ReadingProfileExt for EssReadingProfile {}
