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
    // pub purposes_allowed: u16,
    // pub max_vendor_id: u16,
    // pub vendor_consents: Vec<bool>,
}

named!(parse_bits<(&[u8], usize), (u8, i64, i64, u16, u16, u8, (u8, u8), u16)>, tuple!(
    take_bits!(6u8),
    take_bits!(36u8),
    take_bits!(36u8),
    take_bits!(12u8),
    take_bits!(12u8),
    take_bits!(6u8),
    pair!(take_bits!(6u8), take_bits!(6u8)),
    take_bits!(12u8),
    take_bits!()
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
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use chrono::DateTime;

    #[test]
    fn should_parse_v1_consent() {
        let raw_string = "BOyL46sOyL46sAfPwBDEAB-AAAAo9aPK3aqSxoB49NRFNAgAKCmMoBiEQEQUEQAnQAAAo0DACAAgwBEACAgAAABAAAVAJAIAAAASggEAAAAAABAAAhAAAAQAAIAABAgQAAAAgAAAABA";
        let decoded = base64::decode_config(raw_string, base64::URL_SAFE).unwrap();

        let r = parse(&decoded).ok();

        assert_eq!(r.as_ref().clone().unwrap().1.version, 1);
        assert_eq!(
            r.as_ref().clone().unwrap().1.created,
            DateTime::parse_from_rfc3339("2020-04-21T02:31:45.200Z").unwrap()
        );
        assert_eq!(
            r.as_ref().clone().unwrap().1.last_updated,
            DateTime::parse_from_rfc3339("2020-04-21T02:31:45.200Z").unwrap()
        );
        assert_eq!(r.as_ref().clone().unwrap().1.cmp_id, 31);
        assert_eq!(r.as_ref().clone().unwrap().1.cmp_version, 1008);
        assert_eq!(r.as_ref().clone().unwrap().1.consent_screen, 1);
        assert_eq!(r.as_ref().clone().unwrap().1.consent_language, ['D', 'E']);
        assert_eq!(r.as_ref().clone().unwrap().1.vendor_list_version, 1);
    }
}
