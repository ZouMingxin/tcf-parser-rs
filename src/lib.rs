use crate::models::PublisherRestriction;
use chrono::{DateTime, NaiveDateTime, Utc};
use nom::bits::complete::take;
use nom::multi::many_m_n;
use nom::sequence::pair;
use nom::IResult;

mod models;

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

fn parse_publisher_restrictions(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), Vec<PublisherRestriction>> {
    let (left_over, number_of_publisher_restrictions): ((&[u8], usize), usize) = take(12u8)(input)?;

    many_m_n(
        0,
        number_of_publisher_restrictions,
        parse_publisher_restriction_item,
    )(left_over)
}

fn parse_publisher_restriction_item(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), PublisherRestriction> {
    let (left_over, purpose_id) = take(6u8)(input)?;
    let (left_over, restriction_type) = take(2u8)(left_over)?;
    let (left_over, vendor_ids) = parse_publisher_restriction_item_vendors(left_over)?;

    Ok((
        left_over,
        PublisherRestriction {
            purpose_id,
            restriction_type,
            vendor_ids,
        },
    ))
}

fn parse_publisher_restriction_item_vendors(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), Vec<u16>> {
    let (left_over, number_of_vendors): ((&[u8], usize), usize) = take(12u8)(input)?;
    many_m_n(0, number_of_vendors, parse_publisher_vendor_items)(left_over)
        .map(|(input, v)| (input, v.into_iter().flatten().collect()))
}

fn parse_publisher_vendor_items(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<u16>> {
    let (left_over, is_a_range): ((&[u8], usize), u8) = take(1u8)(input)?;

    if is_a_range == 1 {
        let (left_over, starting_id): ((&[u8], usize), u16) = take(16u8)(left_over)?;
        let (left_over, ending_id): ((&[u8], usize), u16) = take(16u8)(left_over)?;

        let mut result = vec![];
        for x in starting_id..ending_id + 1 {
            result.push(x);
        }

        Ok((left_over, result))
    } else {
        let (left_over, vendor_id) = take(16u8)(left_over)?;

        Ok((left_over, vec![vendor_id]))
    }
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
    let (left_over, publisher_restrictions) = parse_publisher_restrictions(left_over)?;

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
            publisher_restrictions,
        },
    ))
}

fn from_i64_to_datetime(i: i64) -> DateTime<Utc> {
    let naive_date_time =
        NaiveDateTime::from_timestamp((i / 10) as i64, ((i % 10) * 100_000_000) as u32);

    DateTime::<Utc>::from_utc(naive_date_time, Utc)
}

pub fn try_parse(input: &[u8]) -> IResult<&[u8], TcfString> {
    nom::bits::bits(parse_bits)(input)
}

pub fn parse(input: &str) -> Option<TcfString> {
    input
        .split(".")
        .collect::<Vec<_>>()
        .first()
        .and_then(|core_string| base64::decode_config(core_string, base64::URL_SAFE).ok())
        .and_then(|base64_decoded| {
            try_parse(base64_decoded.as_slice())
                .map(|(_, tcf_string)| tcf_string)
                .ok()
        })
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use crate::parse;

    #[test]
    fn should_be_able_to_parse_consent_with_dot() {
        let raw_string = "CO27L5XO27L5XDbACBENAtCAAIoAABQAAAIYAOBAhABAB5IAAQCAAA.something";

        let r = parse(&raw_string);

        assert_eq!(r.as_ref().clone().unwrap().version, 2);
    }

    #[test]
    fn should_parse_v2_consent() {
        let raw_string = "CO27L5XO27L5XDbACBENAtCAAIoAABQAAAIYAOBAhABAB5IAAQCAAA";

        let r = parse(&raw_string);

        assert_eq!(r.as_ref().clone().unwrap().version, 2);
        assert_eq!(
            r.as_ref().clone().unwrap().created,
            DateTime::parse_from_rfc3339("2020-07-22T03:04:02.300Z").unwrap()
        );
        assert_eq!(
            r.as_ref().clone().unwrap().last_updated,
            DateTime::parse_from_rfc3339("2020-07-22T03:04:02.300Z").unwrap()
        );
        assert_eq!(r.as_ref().clone().unwrap().cmp_id, 219);
        assert_eq!(r.as_ref().clone().unwrap().cmp_version, 2);
        assert_eq!(r.as_ref().clone().unwrap().consent_screen, 1);
        assert_eq!(r.as_ref().clone().unwrap().consent_language, ['E', 'N']);
        assert_eq!(r.as_ref().clone().unwrap().vendor_list_version, 45);
        assert_eq!(r.as_ref().clone().unwrap().tcf_policy_version, 2);
        assert!(!r.as_ref().clone().unwrap().is_service_specific);
        assert!(!r.as_ref().clone().unwrap().use_non_standard_stacks);
        // assert_eq!(r.as_ref().clone().unwrap().special_feature_opt_ins, 0);
        // assert_eq!(r.as_ref().clone().unwrap().purposes_consent, 3);
        // assert_eq!(r.as_ref().clone().unwrap().purposes_li_transparency, 0);
        // assert_eq!(r.as_ref().clone().unwrap().purpose_one_treatment, 0);
        assert_eq!(r.as_ref().clone().unwrap().publisher_cc, ['B', 'D']);
        assert_eq!(
            r.as_ref().clone().unwrap().vendor_consents,
            vec![4, 11, 16, 28]
        );
        assert_eq!(
            r.as_ref().clone().unwrap().vendor_legitimate_interests,
            vec![1, 4, 21, 30,]
        );
    }

    #[test]
    fn should_parse_publisher_restrictions_items() {
        let consent = "CO2_OuxO2_OuxDbAAAENAAAAAAAAAAAAACiQAAAAAABAgAQAiABFAgAMAiwCNA";

        let r = parse(consent).unwrap();

        assert_eq!(r.publisher_restrictions.len(), 2);
        assert_eq!(r.publisher_restrictions.first().unwrap().purpose_id, 1);
        assert_eq!(
            r.publisher_restrictions.first().unwrap().restriction_type,
            0
        );

        // separate vendor ids
        assert_eq!(
            r.publisher_restrictions.first().unwrap().vendor_ids,
            vec![136, 138]
        );

        // consecutive vendor ids
        assert_eq!(
            r.publisher_restrictions.last().unwrap().vendor_ids,
            vec![139, 140, 141]
        )
    }
}
