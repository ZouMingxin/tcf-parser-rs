use chrono::{DateTime, NaiveDateTime, Utc};
use nom::bits::complete::take;
use nom::multi::many_m_n;
use nom::sequence::pair;
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
    pub vendor_consents: Vec<u16>,
    pub vendor_legitimate_interests: Vec<u16>,
}

fn parse_vendor_list(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<u16>> {
    let mut vendor_list = Vec::<u16>::default();
    let (left_over, max_vendor_id): ((&[u8], usize), u16) = take(16u8)(input)?;
    let (left_over, is_range_encoding): ((&[u8], usize), u8) = take(1u8)(left_over)?;
    let (left_over, vendor_list_bits) = parse_vendor_items(left_over, max_vendor_id as usize)?;

    for i in 0..max_vendor_id {
        if let Some(v) = vendor_list_bits.get(i as usize) {
            if *v == 1 {
                vendor_list.push(i + 1);
            }
        }
    }

    Ok((left_over, vendor_list))
}

fn parse_vendor_items(input: (&[u8], usize), count: usize) -> IResult<(&[u8], usize), Vec<u8>> {
    many_m_n(0, count, take(1u8))(input)
}

fn parse_bits(input: (&[u8], usize)) -> IResult<(&[u8], usize), TcfString> {
    let (left_over, version) = take(6u8)(input)?;
    let (left_over, created_val) = take(36u8)(left_over)?;
    let (left_over, last_updated_val) = take(36u8)(left_over)?;
    let (left_over, cmp_id) = take(12u8)(left_over)?;
    let (left_over, cmp_version) = take(12u8)(left_over)?;
    let (left_over, consent_screen) = take(6u8)(left_over)?;
    let (left_over, (language_letter_1, language_letter_2)): ((&[u8], usize), (u8, u8)) =
        pair(take(6u8), take(6u8))(left_over)?;

    let (left_over, vendor_list_version) = take(12u8)(left_over)?;
    let (left_over, tcf_policy_version) = take(6u8)(left_over)?;
    let (left_over, is_service_specific_val): ((&[u8], usize), u8) = take(1u8)(left_over)?;
    let (left_over, use_non_standard_stacks_val): ((&[u8], usize), u8) = take(1u8)(left_over)?;
    let (left_over, special_feature_opt_ins) = take(12u8)(left_over)?;
    let (left_over, purposes_consent) = take(24u8)(left_over)?;
    let (left_over, purposes_li_transparency) = take(24u8)(left_over)?;
    let (left_over, purpose_one_treatment) = take(1u8)(left_over)?;
    let (left_over, (publisher_cc_letter_1, publisher_cc_letter_2)): ((&[u8], usize), (u8, u8)) =
        pair(take(6u8), take(6u8))(left_over)?;

    let (left_over, vendor_consents) = parse_vendor_list(left_over)?;
    let (left_over, vendor_legitimate_interests) = parse_vendor_list(left_over)?;

    Ok((
        left_over,
        TcfString {
            version,
            created: from_i64_to_datetime(created_val),
            last_updated: from_i64_to_datetime(last_updated_val),
            cmp_id,
            cmp_version,
            consent_screen,
            consent_language: [
                (65 + language_letter_1).into(),
                (65 + language_letter_2).into(),
            ],
            vendor_list_version,
            tcf_policy_version,
            is_service_specific: is_service_specific_val == 1,
            use_non_standard_stacks: use_non_standard_stacks_val == 1,
            purposes_consent,
            special_feature_opt_ins,
            purpose_one_treatment,
            purposes_li_transparency,
            publisher_cc: [
                (65 + publisher_cc_letter_1).into(),
                (65 + publisher_cc_letter_2).into(),
            ],
            vendor_consents,
            vendor_legitimate_interests,
        },
    ))
}

fn from_i64_to_datetime(i: i64) -> DateTime<Utc> {
    let naive_date_time =
        NaiveDateTime::from_timestamp((i / 10) as i64, ((i % 10) * 100_000_000) as u32);

    DateTime::<Utc>::from_utc(naive_date_time, Utc)
}

pub fn parse(input: &[u8]) -> IResult<&[u8], TcfString> {
    bits(parse_bits)(input)
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use crate::parse;

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
        assert_eq!(
            r.as_ref().clone().unwrap().1.vendor_consents,
            vec![4, 11, 16, 28]
        );
        assert_eq!(
            r.as_ref().clone().unwrap().1.vendor_legitimate_interests,
            vec![1, 4, 21, 30,]
        );
    }
}
