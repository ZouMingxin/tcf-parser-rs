# iabtcf-parser-rs

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![master](https://github.com/ZouMingxin/tcf-parser-rs/workflows/build/badge.svg)

Port of parser in Rust from [iabtcf-java](https://github.com/InteractiveAdvertisingBureau/iabtcf-java).

*Working in progress, not ready for production !*

# Usage

```
let tc_string = "COwxsONOwxsONKpAAAENAdCAAMAAAAAAAAAAAAAAAAAA";
let parsed_opt: Option<IabTcf> = iabtcf_parser::parse(&tc_string);

match parsed_opt {
  Some(iabtcf_parser::V1(tc_string_v1_object) => {
    // make usage of v1 data object.
  }
  Some(iabtcf_parser::V2(tc_string_v2_object)) => {
    // all fields from core string will be available here from IabTcf struct.
    // usage of the object
  },
  _ => // when there is an error happened.
}
```

# Roadmap

- [x] Support v1
- [x] Parse Core String of v2 tc_string
- [ ] Parse other part of v2 tc_string
- [ ] Return Custom Error instead of Option
- [ ] More Documentation

# Resources

The IAB specification for the consent string format is available on the [IAB Github](https://github.com/InteractiveAdvertisingBureau/GDPR-Transparency-and-Consent-Framework/tree/master/TCFv2).

### IAB Europe Transparency and Consent Framework v2
Version 2 of the TCF Specifications were released 21 August 2019 with industry adoption commencing first half of 2020.

Framework Technical specifications available at: https://github.com/InteractiveAdvertisingBureau/GDPR-Transparency-and-Consent-Framework/tree/master/TCFv2

