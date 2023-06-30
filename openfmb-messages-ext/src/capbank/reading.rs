// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use capbankmodule::CapBankReadingProfile;
use openfmb_messages::{commonmodule::*, *};

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExtReading for CapBankReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoReadingMessageInfoSnafu)?)
    }
}

impl OpenFMBExt for CapBankReadingProfile {
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
        Ok("CapBankReadingProfile".to_string())
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

pub trait CapBankReadingExt: ReadingProfileExt {
    fn w_net(&self) -> OpenFMBResult<f64>;

    fn q_net(&self) -> OpenFMBResult<f64>;

    fn v_net(&self) -> OpenFMBResult<f64>;

    fn a_net(&self) -> OpenFMBResult<f64>;
}

impl CapBankReadingExt for CapBankReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .cap_bank_reading
            .as_ref()
            .context(NoCapBankReadingSnafu)?
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

    fn q_net(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .cap_bank_reading
            .as_ref()
            .context(NoCapBankReadingSnafu)?
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

    fn v_net(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .cap_bank_reading
            .as_ref()
            .context(NoCapBankReadingSnafu)?
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

    fn a_net(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .cap_bank_reading
            .as_ref()
            .context(NoCapBankReadingSnafu)?
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
}

impl ReadingProfileExt for CapBankReadingProfile {}
