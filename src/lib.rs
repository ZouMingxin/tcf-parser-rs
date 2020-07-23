use crate::models::TcfString;
use crate::parsers::try_parse;

mod models;
mod parsers;
mod utils;

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

    #[test]
    fn should_parse_range_vendor_consents() {
        let consent = "CO2_lBRO2_lBRDbAAAENAAAAAAAAAAAAACiQABMAAAAQIAEAIgARQIADAIsAjQ";

        let r = parse(consent).unwrap();

        assert_eq!(r.vendor_consents, vec![1, 2]);
    }
}
