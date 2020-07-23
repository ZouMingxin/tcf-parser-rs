use chrono::{DateTime, Utc};

pub struct PublisherRestriction {
    pub purpose_id: u16,
    pub restriction_type: u8,
    pub vendor_ids: Vec<u16>,
}

pub struct TcfString {
    pub version: u8,
    pub created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub cmp_id: u16,
    pub cmp_version: u16,
    pub consent_screen: u8,
    pub consent_language: [char; 2],
    pub vendor_list_version: u16,
    pub tcf_policy_version: u8,
    pub is_service_specific: bool,
    pub use_non_standard_stacks: bool,
    pub special_feature_opt_ins: u16,
    pub purposes_consent: u32,
    pub purposes_li_transparency: u32,
    pub purpose_one_treatment: u8,
    pub publisher_cc: [char; 2],
    pub vendor_consents: Vec<u16>,
    pub vendor_legitimate_interests: Vec<u16>,
    pub publisher_restrictions: Vec<PublisherRestriction>,
}
