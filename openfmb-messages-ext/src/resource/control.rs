// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{error::*, ControlProfileExt, OpenFMBExt};
use openfmb_messages::{
    commonmodule::{
        ConductingEquipment, ControlApc, ControlInc, ControlSpc, IdentifiedObject,
        IsControlMessageInfo, IsMessageInfo, LogicalNode, MessageInfo, Vsc,
    },
    resourcemodule::{
        AnalogControlGgio, BooleanControlGgio, IntegerControlGgio,
        IsResourceDiscreteControlProfile, ResourceDiscreteControl, ResourceDiscreteControlProfile,
        StringControlGgio,
    },
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

impl OpenFMBExt for ResourceDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        unimplemented!()
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("ResourceDiscreteControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipmentSnafu)?
                .m_rid,
        )
        .context(UuidSnafu)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
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

pub trait ResourceControlExt: ControlProfileExt {
    fn set_analog_msg(m_rid: &str, val: f64) -> ResourceDiscreteControlProfile {
        Self::set_double_msg(m_rid, val, 0)
    }

    fn set_double_msg(m_rid: &str, val: f64, index: usize) -> ResourceDiscreteControlProfile;
    fn set_bool_msg(m_rid: &str, val: bool, index: usize) -> ResourceDiscreteControlProfile;
    fn set_int_msg(m_rid: &str, val: i32, index: usize) -> ResourceDiscreteControlProfile;
    fn set_string_msg(m_rid: &str, val: String, index: usize) -> ResourceDiscreteControlProfile;

    // ev
    fn start_transaction(m_rid: &str, connector_id: i32) -> ResourceDiscreteControlProfile;
    fn stop_transaction(
        m_rid: &str,
        connector_id: Option<i32>,
        transaction_id: Option<i32>,
    ) -> ResourceDiscreteControlProfile;

    fn build_control_profile(
        m_rid: &str,
        val: f64,
        start_time: SystemTime,
    ) -> ResourceDiscreteControlProfile;

    fn message_identified_object_name(&self) -> OpenFMBResult<String>;
    fn message_identified_description(&self) -> OpenFMBResult<String>;
    fn string_ggio(&self) -> OpenFMBResult<Vec<StringControlGgio>>;
    fn analog_ggio(&self) -> OpenFMBResult<Vec<AnalogControlGgio>>;
    fn integer_ggio(&self) -> OpenFMBResult<Vec<IntegerControlGgio>>;
    fn boolean_ggio(&self) -> OpenFMBResult<Vec<BooleanControlGgio>>;

    fn string_value_by_key(&self, key: &str) -> OpenFMBResult<String>;
    fn analog_value_by_key(&self, key: &str) -> OpenFMBResult<f64>;
    fn integer_value_by_key(&self, key: &str) -> OpenFMBResult<i32>;
    fn boolean_value_by_key(&self, key: &str) -> OpenFMBResult<bool>;
}

impl ControlProfileExt for ResourceDiscreteControlProfile {}

impl ResourceControlExt for ResourceDiscreteControlProfile {
    fn build_control_profile(
        m_rid: &str,
        val: f64,
        _start_time: SystemTime,
    ) -> ResourceDiscreteControlProfile {
        ResourceDiscreteControlProfile {
            control_message_info: Some(ResourceDiscreteControlProfile::build_control_message_info()),
            conducting_equipment: Some(ConductingEquipment {
                named_object: None,
                m_rid: m_rid.to_string(),
            }),
            resource_discrete_control: Some(ResourceDiscreteControl {
                analog_control_ggio: vec![AnalogControlGgio {
                    logical_node: None,
                    phase: None,
                    an_out: Some(ControlApc { ctl_val: val }),
                }],
                check: None,
                identified_object: None,
                boolean_control_ggio: vec![],
                integer_control_ggio: vec![],
                string_control_ggio: vec![],
            }),
        }
    }

    fn set_double_msg(m_rid: &str, val: f64, index: usize) -> ResourceDiscreteControlProfile {
        let mut controls: Vec<AnalogControlGgio> = Vec::new();
        for _n in 0..index {
            controls.push(AnalogControlGgio {
                logical_node: None,
                phase: None,
                an_out: None,
            });
        }
        controls.push(AnalogControlGgio {
            logical_node: None,
            phase: None,
            an_out: Some(ControlApc { ctl_val: val }),
        });

        ResourceDiscreteControlProfile {
            control_message_info: Some(ResourceDiscreteControlProfile::build_control_message_info()),
            conducting_equipment: Some(ConductingEquipment {
                named_object: None,
                m_rid: m_rid.to_string(),
            }),
            resource_discrete_control: Some(ResourceDiscreteControl {
                check: None,
                identified_object: None,
                boolean_control_ggio: vec![],
                analog_control_ggio: controls,
                integer_control_ggio: vec![],
                string_control_ggio: vec![],
            }),
        }
    }

    fn set_bool_msg(m_rid: &str, val: bool, index: usize) -> ResourceDiscreteControlProfile {
        let mut controls: Vec<BooleanControlGgio> = Vec::new();
        for _n in 0..index {
            controls.push(BooleanControlGgio {
                logical_node: None,
                phase: None,
                spcso: None,
            });
        }
        controls.push(BooleanControlGgio {
            logical_node: None,
            phase: None,
            spcso: Some(ControlSpc { ctl_val: val }),
        });

        ResourceDiscreteControlProfile {
            control_message_info: Some(ResourceDiscreteControlProfile::build_control_message_info()),
            conducting_equipment: Some(ConductingEquipment {
                named_object: None,
                m_rid: m_rid.to_string(),
            }),
            resource_discrete_control: Some(ResourceDiscreteControl {
                check: None,
                identified_object: None,
                boolean_control_ggio: controls,
                analog_control_ggio: vec![],
                integer_control_ggio: vec![],
                string_control_ggio: vec![],
            }),
        }
    }

    fn set_int_msg(m_rid: &str, val: i32, index: usize) -> ResourceDiscreteControlProfile {
        let mut controls: Vec<IntegerControlGgio> = Vec::new();
        for _n in 0..index {
            controls.push(IntegerControlGgio {
                logical_node: None,
                phase: None,
                iscso: None,
            });
        }
        controls.push(IntegerControlGgio {
            logical_node: None,
            phase: None,
            iscso: Some(ControlInc { ctl_val: val }),
        });

        ResourceDiscreteControlProfile {
            control_message_info: Some(ResourceDiscreteControlProfile::build_control_message_info()),
            conducting_equipment: Some(ConductingEquipment {
                named_object: None,
                m_rid: m_rid.to_string(),
            }),
            resource_discrete_control: Some(ResourceDiscreteControl {
                check: None,
                identified_object: None,
                boolean_control_ggio: vec![],
                analog_control_ggio: vec![],
                integer_control_ggio: controls,
                string_control_ggio: vec![],
            }),
        }
    }

    fn set_string_msg(m_rid: &str, val: String, index: usize) -> ResourceDiscreteControlProfile {
        let mut controls: Vec<StringControlGgio> = Vec::new();
        for _n in 0..index {
            controls.push(StringControlGgio {
                logical_node: None,
                phase: None,
                str_out: None,
            });
        }
        controls.push(StringControlGgio {
            logical_node: None,
            phase: None,
            str_out: Some(Vsc { ctl_val: val }),
        });

        ResourceDiscreteControlProfile {
            control_message_info: Some(ResourceDiscreteControlProfile::build_control_message_info()),
            conducting_equipment: Some(ConductingEquipment {
                named_object: None,
                m_rid: m_rid.to_string(),
            }),
            resource_discrete_control: Some(ResourceDiscreteControl {
                check: None,
                identified_object: None,
                boolean_control_ggio: vec![],
                analog_control_ggio: vec![],
                integer_control_ggio: vec![],
                string_control_ggio: controls,
            }),
        }
    }

    fn start_transaction(m_rid: &str, connector_id: i32) -> ResourceDiscreteControlProfile {
        let mut control_message_info = ResourceDiscreteControlProfile::build_control_message_info();
        control_message_info
            .message_info_mut()
            .identified_object_mut()
            .name = Some("RemoteStartTransactionRequest".into());

        ResourceDiscreteControlProfile {
            control_message_info: Some(control_message_info),
            conducting_equipment: Some(ConductingEquipment {
                named_object: None,
                m_rid: m_rid.to_string(),
            }),
            resource_discrete_control: Some(ResourceDiscreteControl {
                integer_control_ggio: vec![IntegerControlGgio {
                    logical_node: Some(LogicalNode {
                        identified_object: Some(IdentifiedObject {
                            name: Some("ocpp16j.RemoteStartTransactionRequest.connector_id".into()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    iscso: Some(ControlInc {
                        ctl_val: connector_id,
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
        }
    }

    fn stop_transaction(
        m_rid: &str,
        connector_id: Option<i32>,
        transaction_id: Option<i32>,
    ) -> ResourceDiscreteControlProfile {
        let mut control_message_info = ResourceDiscreteControlProfile::build_control_message_info();
        control_message_info
            .message_info_mut()
            .identified_object_mut()
            .name = Some("RemoteStopTransactionRequest".into());

        let mut profile = ResourceDiscreteControlProfile {
            control_message_info: Some(control_message_info),
            conducting_equipment: Some(ConductingEquipment {
                named_object: None,
                m_rid: m_rid.to_string(),
            }),
            resource_discrete_control: Some(ResourceDiscreteControl {
                integer_control_ggio: vec![],
                ..Default::default()
            }),
        };

        profile
            .resource_discrete_control_mut()
            .integer_control_ggio
            .push(IntegerControlGgio {
                logical_node: Some(LogicalNode {
                    identified_object: Some(IdentifiedObject {
                        name: Some("ocpp16j.RemoteStopTransactionRequest.connector_id".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                iscso: Some(ControlInc {
                    ctl_val: connector_id.unwrap_or(0) as i32,
                }),
                ..Default::default()
            });

        if let Some(transaction_id) = transaction_id {
            profile
                .resource_discrete_control_mut()
                .integer_control_ggio
                .push(IntegerControlGgio {
                    logical_node: Some(LogicalNode {
                        identified_object: Some(IdentifiedObject {
                            name: Some(
                                "ocpp16j.RemoteStopTransactionRequest.transaction_id".into(),
                            ),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    iscso: Some(ControlInc {
                        ctl_val: transaction_id as i32,
                    }),
                    ..Default::default()
                });
        }

        profile
    }

    fn message_identified_object_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .control_message_info
            .as_ref()
            .context(NoControlMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?
            .identified_object
            .as_ref()
            .context(NoIdentifiedObjectSnafu)?
            .name
            .clone()
            .unwrap_or("".to_string()))
    }

    fn message_identified_description(&self) -> OpenFMBResult<String> {
        Ok(self
            .control_message_info
            .as_ref()
            .context(NoControlMessageInfoSnafu)?
            .message_info
            .as_ref()
            .context(NoMessageInfoSnafu)?
            .identified_object
            .as_ref()
            .context(NoIdentifiedObjectSnafu)?
            .description
            .clone()
            .unwrap_or("".to_string()))
    }

    fn string_ggio(&self) -> OpenFMBResult<Vec<StringControlGgio>> {
        Ok(self
            .resource_discrete_control
            .as_ref()
            .context(NoResourceDiscreteControlSnafu)?
            .string_control_ggio
            .clone())
    }

    fn analog_ggio(&self) -> OpenFMBResult<Vec<AnalogControlGgio>> {
        Ok(self
            .resource_discrete_control
            .as_ref()
            .context(NoResourceDiscreteControlSnafu)?
            .analog_control_ggio
            .clone())
    }

    fn integer_ggio(&self) -> OpenFMBResult<Vec<IntegerControlGgio>> {
        Ok(self
            .resource_discrete_control
            .as_ref()
            .context(NoResourceDiscreteControlSnafu)?
            .integer_control_ggio
            .clone())
    }

    fn boolean_ggio(&self) -> OpenFMBResult<Vec<BooleanControlGgio>> {
        Ok(self
            .resource_discrete_control
            .as_ref()
            .context(NoResourceDiscreteControlSnafu)?
            .boolean_control_ggio
            .clone())
    }

    fn string_value_by_key(&self, key: &str) -> OpenFMBResult<String> {
        let into_iter = self.string_ggio()?.into_iter();

        for item in into_iter {
            if let Ok(name) = item
                .logical_node
                .as_ref()
                .context(NoLogicalNodeSnafu)?
                .identified_object
                .as_ref()
                .context(NoIdentifiedObjectSnafu)?
                .name
                .as_ref()
                .context(NoNameSnafu)
            {
                if key == name.to_string() {
                    return Ok(item.str_out.as_ref().context(NoVscSnafu)?.ctl_val.clone());
                }
            }
        }

        Err(OpenFMBError::NoValue)
    }

    fn analog_value_by_key(&self, key: &str) -> OpenFMBResult<f64> {
        let into_iter = self.analog_ggio()?.into_iter();

        for item in into_iter {
            if let Ok(name) = item
                .logical_node
                .as_ref()
                .context(NoLogicalNodeSnafu)?
                .identified_object
                .as_ref()
                .context(NoIdentifiedObjectSnafu)?
                .name
                .as_ref()
                .context(NoNameSnafu)
            {
                if key == name.to_string() {
                    return Ok(item.an_out.as_ref().context(NoControlApcSnafu)?.ctl_val);
                }
            }
        }

        Err(OpenFMBError::NoValue)
    }

    fn integer_value_by_key(&self, key: &str) -> OpenFMBResult<i32> {
        let into_iter = self.integer_ggio()?.into_iter();

        for item in into_iter {
            if let Ok(name) = item
                .logical_node
                .as_ref()
                .context(NoLogicalNodeSnafu)?
                .identified_object
                .as_ref()
                .context(NoIdentifiedObjectSnafu)?
                .name
                .as_ref()
                .context(NoNameSnafu)
            {
                if key == name.to_string() {
                    return Ok(item.iscso.as_ref().context(NoControlIncSnafu)?.ctl_val);
                }
            }
        }

        Err(OpenFMBError::NoValue)
    }

    fn boolean_value_by_key(&self, key: &str) -> OpenFMBResult<bool> {
        let into_iter = self.boolean_ggio()?.into_iter();

        for item in into_iter {
            if let Ok(name) = item
                .logical_node
                .as_ref()
                .context(NoLogicalNodeSnafu)?
                .identified_object
                .as_ref()
                .context(NoIdentifiedObjectSnafu)?
                .name
                .as_ref()
                .context(NoNameSnafu)
            {
                if key == name.to_string() {
                    return Ok(item.spcso.as_ref().context(NoControlSpcSnafu)?.ctl_val);
                }
            }
        }

        Err(OpenFMBError::NoValue)
    }
}
