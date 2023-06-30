// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    loadmodule::LoadReadingProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExt for LoadReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".to_string())
        //panic!("{:?}", self);
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
        Ok("LoadReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .energy_consumer
                .as_ref()
                .context(NoEnergyConsumerSnafu)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .energy_consumer
            .as_ref()
            .context(NoEnergyConsumerSnafu)?
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

impl OpenFMBExtReading for LoadReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfoSnafu)?)
    }
}

pub trait LoadReadingExt: ReadingProfileExt {
    fn w_net(&self) -> OpenFMBResult<f64>;

    fn q_net(&self) -> OpenFMBResult<f64>;

    fn v_net(&self) -> OpenFMBResult<f64>;
    fn v_phs_a(&self) -> OpenFMBResult<f64>;
    fn v_phs_b(&self) -> OpenFMBResult<f64>;
    fn v_phs_c(&self) -> OpenFMBResult<f64>;

    fn a_net(&self) -> OpenFMBResult<f64>;
}

impl LoadReadingExt for LoadReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReadingSnafu)?
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
            .load_reading
            .as_ref()
            .context(NoLoadReadingSnafu)?
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

    fn v_net(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReadingSnafu)?
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

    fn v_phs_a(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReadingSnafu)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxuSnafu)?
            .ph_v
            .as_ref()
            .context(NoValueSnafu)?
            .phs_a
            .as_ref()
            .context(NoNetSnafu)?
            .c_val
            .as_ref()
            .context(NoCValSnafu)?
            .mag);
    }

    fn v_phs_b(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReadingSnafu)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxuSnafu)?
            .ph_v
            .as_ref()
            .context(NoValueSnafu)?
            .phs_b
            .as_ref()
            .context(NoPhsBSnafu)?
            .c_val
            .as_ref()
            .context(NoCValSnafu)?
            .mag);
    }

    fn v_phs_c(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReadingSnafu)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxuSnafu)?
            .ph_v
            .as_ref()
            .context(NoValueSnafu)?
            .phs_c
            .as_ref()
            .context(NoPhsCSnafu)?
            .c_val
            .as_ref()
            .context(NoCValSnafu)?
            .mag);
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReadingSnafu)?
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

impl ReadingProfileExt for LoadReadingProfile {}
