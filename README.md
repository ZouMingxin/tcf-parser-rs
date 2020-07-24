# iabtcf-parser-rs

Port of parser in Rust from [iabtcf-java](https://github.com/InteractiveAdvertisingBureau/iabtcf-java).

*Working in progress, not ready for production !*

This crate parses the *core string* of v2 consent only now.

# Usage

```
let tc_string = "COwxsONOwxsONKpAAAENAdCAAMAAAAAAAAAAAAAAAAAA";
let parsed_opt: Option<IabTcf> = iabtcf_parser::parse(&tc_string);

match parsed_opt {
  Some(iabtcf_parser::V1) => println!("You shouldn't be here since you passed in a v2 consent"),
  Some(iabtcf_parser::V2(tc_string_object)) => {
    // all fields from core string will be available here from IabTcf struct.
    // usage of the object
  },
  _ => // when there is an error happened.
}
```

# Roadmap

- [ ] Support v1
- [ ] Parse other part of v2 tc_string
- [ ] Return Custom Error instead of Option
- [ ] More Documentation

# Resources

The IAB specification for the consent string format is available on the [IAB Github](https://github.com/InteractiveAdvertisingBureau/GDPR-Transparency-and-Consent-Framework/tree/master/TCFv2).

### IAB Europe Transparency and Consent Framework v2
Version 2 of the TCF Specifications were released 21 August 2019 with industry adoption commencing first half of 2020.

Framework Technical specifications available at: https://github.com/InteractiveAdvertisingBureau/GDPR-Transparency-and-Consent-Framework/tree/master/TCFv2

