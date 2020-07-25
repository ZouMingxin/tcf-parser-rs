use chrono::{DateTime, Utc};
use nom::bits::complete::take;
use nom::multi::many_m_n;
use nom::sequence::pair;
use nom::IResult;

use crate::models::{IabTcf, PublisherRestriction, TcfString};
use crate::utils::{from_i64_to_datetime, from_u8_to_char};

fn parse_vendor_list(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<u16>> {
    let (left_over, max_vendor_id): ((&[u8], usize), u16) = take(16u8)(input)?;
    let (left_over, is_range_encoding) = parse_1_bit_to_bool(left_over)?;
    if is_range_encoding {
        parse_range_entry_section(left_over)
    } else {
        parse_bit_fields_section(left_over, max_vendor_id)
    }
}

fn parse_bit_fields_section(
    input: (&[u8], usize),
    max_vendor_id: u16,
) -> IResult<(&[u8], usize), Vec<u16>> {
    let mut vendor_list = Vec::<u16>::default();
    let (left_over, vendor_list_bits) = parse_vendor_items(input, max_vendor_id as usize)?;

    for i in 0..max_vendor_id {
        if let Some(v) = vendor_list_bits.get(i as usize) {
            if *v == 1 {
                vendor_list.push((i + 1) as u16);
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
    let (left_over, vendor_ids) = parse_range_entry_section(left_over)?;

    Ok((
        left_over,
        PublisherRestriction {
            purpose_id,
            restriction_type,
            vendor_ids,
        },
    ))
}

fn parse_range_entry_section(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<u16>> {
    let (left_over, number_of_vendors): ((&[u8], usize), usize) = take(12u8)(input)?;
    many_m_n(0, number_of_vendors, parse_range_entry)(left_over)
        .map(|(input, v)| (input, v.into_iter().flatten().collect()))
}

fn parse_range_entry(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<u16>> {
    let (left_over, is_a_range) = parse_1_bit_to_bool(input)?;

    if is_a_range {
        parse_range_vendor_ids(left_over)
    } else {
        let (left_over, vendor_id) = take(16u8)(left_over)?;

        Ok((left_over, vec![vendor_id]))
    }
}

fn parse_range_vendor_ids(left_over: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<u16>> {
    let (left_over, starting_id): ((&[u8], usize), u16) = take(16u8)(left_over)?;
    let (left_over, ending_id): ((&[u8], usize), u16) = take(16u8)(left_over)?;

    let mut result = vec![];
    for x in starting_id..ending_id + 1 {
        result.push(x);
    }

    Ok((left_over, result))
}

fn parse_purposes(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<u8>> {
    let (left_over, raw_vec) = many_m_n(0, 24, take(1u8))(input)?;

    Ok((
        left_over,
        raw_vec
            .into_iter()
            .enumerate()
            .filter_map(|(index, r): (usize, u8)| {
                if r == 0 {
                    None
                } else {
                    Some((index + 1) as u8)
                }
            })
            .collect(),
    ))
}

fn parse_v2(input: (&[u8], usize)) -> IResult<(&[u8], usize), TcfString> {
    let (left_over, created) = parse_timestamp(input)?;
    let (left_over, last_updated) = parse_timestamp(left_over)?;
    let (left_over, cmp_id) = take(12u8)(left_over)?;
    let (left_over, cmp_version) = take(12u8)(left_over)?;
    let (left_over, consent_screen) = take(6u8)(left_over)?;
    let (left_over, consent_language) = parse_language(left_over)?;
    let (left_over, vendor_list_version) = take(12u8)(left_over)?;
    let (left_over, tcf_policy_version) = take(6u8)(left_over)?;
    let (left_over, is_service_specific) = parse_1_bit_to_bool(left_over)?;
    let (left_over, use_non_standard_stacks) = parse_1_bit_to_bool(left_over)?;
    let (left_over, special_feature_opt_ins) = take(12u8)(left_over)?;
    let (left_over, purposes_consent) = parse_purposes(left_over)?;
    let (left_over, purposes_li_transparency) = parse_purposes(left_over)?;
    let (left_over, purpose_one_treatment) = take(1u8)(left_over)?;
    let (left_over, publisher_cc) = parse_language(left_over)?;
    let (left_over, vendor_consents) = parse_vendor_list(left_over)?;
    let (left_over, vendor_legitimate_interests) = parse_vendor_list(left_over)?;
    let (left_over, publisher_restrictions) = parse_publisher_restrictions(left_over)?;

    Ok((
        left_over,
        TcfString {
            created,
            last_updated,
            cmp_id,
            cmp_version,
            consent_screen,
            consent_language,
            vendor_list_version,
            tcf_policy_version,
            is_service_specific,
            use_non_standard_stacks,
            purposes_consent,
            special_feature_opt_ins,
            purpose_one_treatment,
            purposes_li_transparency,
            publisher_cc,
            vendor_consents,
            vendor_legitimate_interests,
            publisher_restrictions,
        },
    ))
}

fn parse_version(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(6u8)(input)
}

fn parse_bits(input: (&[u8], usize)) -> IResult<(&[u8], usize), IabTcf> {
    let (left_over, version) = parse_version(input)?;

    match version {
        1 => Ok((left_over, IabTcf::V1)),
        2 => Ok((left_over, IabTcf::V2(parse_v2(left_over)?.1))),
        _ => Ok((left_over, IabTcf::Unknown)),
    }
}

pub(crate) fn try_parse(input: &[u8]) -> IResult<&[u8], IabTcf> {
    nom::bits::bits(parse_bits)(input)
}

fn parse_language(input: (&[u8], usize)) -> IResult<(&[u8], usize), [char; 2]> {
    let (left_over, (letter1, letter2)) = pair(take(6u8), take(6u8))(input)?;

    Ok((
        left_over,
        [from_u8_to_char(letter1), from_u8_to_char(letter2)],
    ))
}

fn parse_1_bit_to_bool(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    let (left_over, bit_val): ((&[u8], usize), u8) = take(1u8)(input)?;
    Ok((left_over, bit_val == 1))
}

fn parse_timestamp(input: (&[u8], usize)) -> IResult<(&[u8], usize), DateTime<Utc>> {
    let (left_over, timestamp_val) = take(36u8)(input)?;

    Ok((left_over, from_i64_to_datetime(timestamp_val)))
}
