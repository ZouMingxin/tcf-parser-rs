use crate::parsers::try_parse;
use models::IabTcf;

pub mod models;
mod parsers;
mod utils;

pub fn parse(input: &str) -> Option<IabTcf> {
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

    use crate::{models::IabTcf, parse};

    #[test]
    fn should_be_able_to_parse_consent_with_dot() {
        let raw_string = "CO27L5XO27L5XDbACBENAtCAAIoAABQAAAIYAOBAhABAB5IAAQCAAA.something";

        let r = parse(&raw_string).unwrap();

        match r {
            IabTcf::V2(tcf_string) => {
                assert_eq!(tcf_string.cmp_id, 219);
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn should_parse_v2_consent() {
        let raw_string = "CO27L5XO27L5XDbACBENAtCAAIoAABQAAAIYAOBAhABAB5IAAQCAAA";

        let r = parse(&raw_string).unwrap();

        match r {
            IabTcf::V2(tcf_string) => {
                assert_eq!(
                    tcf_string.created,
                    DateTime::parse_from_rfc3339("2020-07-22T03:04:02.300Z").unwrap()
                );
                assert_eq!(
                    tcf_string.last_updated,
                    DateTime::parse_from_rfc3339("2020-07-22T03:04:02.300Z").unwrap()
                );
                assert_eq!(tcf_string.cmp_id, 219);
                assert_eq!(tcf_string.cmp_version, 2);
                assert_eq!(tcf_string.consent_screen, 1);
                assert_eq!(tcf_string.consent_language, ['E', 'N']);
                assert_eq!(tcf_string.vendor_list_version, 45);
                assert_eq!(tcf_string.tcf_policy_version, 2);
                assert!(!tcf_string.is_service_specific);
                assert!(!tcf_string.use_non_standard_stacks);
                // assert_eq!(tcf_string.special_feature_opt_ins, 0);
                assert_eq!(tcf_string.purposes_consent, vec![1, 5, 7]);
                // assert_eq!(tcf_string.purposes_li_transparency, 0);
                // assert_eq!(tcf_string.purpose_one_treatment, 0);
                assert_eq!(tcf_string.publisher_cc, ['B', 'D']);
                assert_eq!(tcf_string.vendor_consents, vec![4, 11, 16, 28]);
                assert_eq!(tcf_string.vendor_legitimate_interests, vec![1, 4, 21, 30,]);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn should_parse_publisher_restrictions_items() {
        let consent = "CO2_OuxO2_OuxDbAAAENAAAAAAAAAAAAACiQAAAAAABAgAQAiABFAgAMAiwCNA";

        let r = parse(consent).unwrap();

        match r {
            IabTcf::V2(tcf_string) => {
                assert_eq!(tcf_string.publisher_restrictions.len(), 2);
                assert_eq!(
                    tcf_string
                        .publisher_restrictions
                        .first()
                        .unwrap()
                        .purpose_id,
                    1
                );
                assert_eq!(
                    tcf_string
                        .publisher_restrictions
                        .first()
                        .unwrap()
                        .restriction_type,
                    0
                );

                // separate vendor ids
                assert_eq!(
                    tcf_string
                        .publisher_restrictions
                        .first()
                        .unwrap()
                        .vendor_ids,
                    vec![136, 138]
                );

                // consecutive vendor ids
                assert_eq!(
                    tcf_string.publisher_restrictions.last().unwrap().vendor_ids,
                    vec![139, 140, 141]
                );
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn should_parse_range_vendor_consents() {
        let consent = "CO2_lBRO2_lBRDbAAAENAAAAAAAAAAAAACiQABMAAAAQIAEAIgARQIADAIsAjQ";

        let r = parse(consent).unwrap();

        match r {
            IabTcf::V2(tcf_string) => {
                assert_eq!(tcf_string.vendor_consents, vec![1, 2]);
            }
            _ => assert!(false),
        }
    }
}
