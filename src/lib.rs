use chrono::{DateTime, NaiveDateTime, Utc};
use nom::*;

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
    pub max_vendor_id: i16,
    // pub vendor_consents: Vec<bool>,
}

named!(parse_bits<(&[u8], usize), (u8, i64, i64, u16, u16, u8, (u8, u8), u16, u8, u8, u8, u16, u32, u32, u8, (u8, u8), i16)>, tuple!(
    take_bits!(6u8),
    take_bits!(36u8),
    take_bits!(36u8),
    take_bits!(12u8),
    take_bits!(12u8),
    take_bits!(6u8),
    pair!(take_bits!(6u8), take_bits!(6u8)),
    take_bits!(12u8),
    take_bits!(6u8),
    take_bits!(1u8),
    take_bits!(1u8),
    take_bits!(12u8),
    take_bits!(24u8),
    take_bits!(24u8),
    take_bits!(1u8),
    pair!(take_bits!(6u8), take_bits!(6u8)),
    take_bits!(16u8)
));

fn from_i64_to_datetime(i: i64) -> DateTime<Utc> {
    let naive_date_time =
        NaiveDateTime::from_timestamp((i / 10) as i64, ((i % 10) * 100_000_000) as u32);

    DateTime::<Utc>::from_utc(naive_date_time, Utc)
}

pub fn parse(input: &[u8]) -> IResult<&[u8], TcfString> {
    let (
        input,
        (
            version,
            created,
            last_updated,
            cmp_id,
            cmp_version,
            consent_screen,
            (language_letter_1, language_letter_2),
            vendor_list_version,
            tcf_policy_version,
            is_service_sepecific_val,
            use_non_standard_stacks_val,
            special_feature_opt_ins,
            purposes_consent,
            purposes_li_transparency,
            purpose_one_treatment,
            (publisher_cc_letter_1, publisher_cc_letter_2),
            max_vendor_id,
        ),
    ) = bits(parse_bits)(input)?;

    Ok((
        input,
        TcfString {
            version,
            created: from_i64_to_datetime(created),
            last_updated: from_i64_to_datetime(last_updated),
            cmp_id,
            cmp_version,
            consent_screen,
            consent_language: [
                (65 + language_letter_1).into(),
                (65 + language_letter_2).into(),
            ],
            vendor_list_version,
            tcf_policy_version,
            is_service_specific: is_service_sepecific_val == 1,
            use_non_standard_stacks: use_non_standard_stacks_val == 1,
            special_feature_opt_ins,
            purposes_consent,
            purposes_li_transparency,
            purpose_one_treatment,
            publisher_cc: [
                (65 + publisher_cc_letter_1).into(),
                (65 + publisher_cc_letter_2).into(),
            ],
            max_vendor_id,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use chrono::DateTime;

    #[test]
    fn should_parse_v2_consent() {
        let raw_string = "CO27L5XO27L5XDbACBENAtCAAIoAABQAAAIYAOBAhABAB5IAAQCAAA";
        let decoded = base64::decode_config(raw_string, base64::URL_SAFE).unwrap();

        let r = parse(&decoded).ok();

        assert_eq!(r.as_ref().clone().unwrap().1.version, 2);
        assert_eq!(
            r.as_ref().clone().unwrap().1.created,
            DateTime::parse_from_rfc3339("2020-07-22T03:04:02.300Z").unwrap()
        );
        assert_eq!(
            r.as_ref().clone().unwrap().1.last_updated,
            DateTime::parse_from_rfc3339("2020-07-22T03:04:02.300Z").unwrap()
        );
        assert_eq!(r.as_ref().clone().unwrap().1.cmp_id, 219);
        assert_eq!(r.as_ref().clone().unwrap().1.cmp_version, 2);
        assert_eq!(r.as_ref().clone().unwrap().1.consent_screen, 1);
        assert_eq!(r.as_ref().clone().unwrap().1.consent_language, ['E', 'N']);
        assert_eq!(r.as_ref().clone().unwrap().1.vendor_list_version, 45);
        assert_eq!(r.as_ref().clone().unwrap().1.tcf_policy_version, 2);
        assert!(!r.as_ref().clone().unwrap().1.is_service_specific);
        assert!(!r.as_ref().clone().unwrap().1.use_non_standard_stacks);
        // assert_eq!(r.as_ref().clone().unwrap().1.special_feature_opt_ins, 0);
        // assert_eq!(r.as_ref().clone().unwrap().1.purposes_consent, 3);
        // assert_eq!(r.as_ref().clone().unwrap().1.purposes_li_transparency, 0);
        // assert_eq!(r.as_ref().clone().unwrap().1.purpose_one_treatment, 0);
        assert_eq!(r.as_ref().clone().unwrap().1.publisher_cc, ['B', 'D']);
        assert_eq!(r.as_ref().clone().unwrap().1.max_vendor_id, 28);
    }
}
