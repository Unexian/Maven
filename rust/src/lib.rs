#[allow(dead_code)]

mod maven {
    use regex::Regex;
    use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering};

    #[derive(Clone)]
    struct Maven<'l> {
        major: i16,
        minor: i16,
        patch: Option<i16>,
        extra: Option<&'l str>,
        tagln: Option<&'l str>
    }
    impl<'l> Maven<'l> {
        pub fn new(from: &'l str) -> Result<Self, &'l str> {
            let regex = Regex::new(
                r#"^(\d+?)\.(\d+?)(?:\.(\d+?))?(?:-(.+?)(?: |$))?(?: ?"(.*?)")?$"#
            ).unwrap();
            match regex.captures(from) {
                None => return Err("Malformed maven version"),
                Some(caps) => {
                    let major: i16 = match caps.name("1") {
                        Some(s) => {
                            let parsed: Result<i16, _> = str::parse(s.into());
                            match parsed { Ok(x) => x as i16, Err(_) => return Err("Error parsing version")}
                        },
                        None => panic!("Internal error!")
                    };
                    let minor: i16 = match caps.name("2") {
                        Some(s) => {
                            let parsed: Result<i16, _> = str::parse(s.into());
                            match parsed { Ok(x) => x as i16, Err(_) => return Err("Error parsing version")}
                        },
                        None => panic!("Internal error!")
                    };
                    let patch: Option<i16> = match caps.name("3") {
                        Some(s) => Some({
                            let parsed: Result<i16, _> = str::parse(s.into());
                            match parsed { Ok(x) => x as i16, Err(_) => return Err("Error parsing version")}
                        }),
                        None => None
                    };
                    let extra: Option<&str> = match caps.name("4") {
                        Some(s) => Some(s.into()),
                        None => None
                    };
                    let tagln: Option<&str> = match caps.name("5") {
                        Some(s) => Some(s.into()),
                        None => None
                    };
                    Ok(Self {
                        major: major,
                        minor: minor,
                        patch: patch,
                        extra: extra,
                        tagln: tagln
                    })
        }   }   }
        pub fn get_id(&self) -> String {
            format!("{}.{}{}{}", self.major, self.minor, match self.patch {
                Some(pat) => format!(".{}", pat), None => "".to_owned(),
            }, match self.extra {
                Some(ext) => format!("-{}", ext), None => "".to_owned(),
            })
        }
        pub fn get_name(&self) -> String {
            match self.tagln {
                Some(tag) => format!("{} \"{}\"", self.get_id(), tag),
                None => self.get_id(),
            }
        }
    }
    impl<'l> PartialEq for Maven<'l> {
        fn eq(&self, other: &Self) -> bool {
            self.major == other.major &&
            self.minor == other.minor &&
            self.patch == other.patch &&
            self.extra == other.extra
        }
    }
    impl<'l> PartialOrd for Maven<'l> {
        fn partial_cmp(&self, other: &Maven) -> Option<Ordering> {
            if self.major != other.major { return self.major.partial_cmp(&other.major); };
            if self.minor != other.minor { return self.major.partial_cmp(&other.minor); };
            if self.patch != None || other.patch != None {
                if self.patch == None { return Some(Ordering::Less) };
                if other.patch == None { return Some(Ordering::Greater) };
                let spatch = match self.patch { Some(x) => x, None => panic!() };
                let opatch = match other.patch { Some(x) => x, None => panic!() };
                if spatch != opatch { return spatch.partial_cmp(&opatch); };
            };
            if self.extra != None || other.extra != None {
                if self.extra == None { return Some(Ordering::Less) };
                if other.extra == None { return Some(Ordering::Greater) };
                let sextra = match self.extra { Some(x) => x, None => panic!() };
                let oextra = match other.extra { Some(x) => x, None => panic!() };
                if sextra != oextra { return sextra.partial_cmp(&oextra); };
            };
            Some(Ordering::Equal)
        }
    }
    impl<'l> Eq for Maven<'l> {}
    impl<'l> Ord for Maven<'l> {
        fn cmp(&self, other: &Maven) -> Ordering {
            if self.major != other.major { return self.major.cmp(&other.major); };
            if self.minor != other.minor { return self.major.cmp(&other.minor); };
            if self.patch != None || other.patch != None {
                if self.patch == None { return Ordering::Less };
                if other.patch == None { return Ordering::Greater };
                let spatch = match self.patch { Some(x) => x, None => panic!() };
                let opatch = match other.patch { Some(x) => x, None => panic!() };
                if spatch != opatch { return spatch.cmp(&opatch); };
            };
            if self.extra != None || other.extra != None {
                if self.extra == None { return Ordering::Less };
                if other.extra == None { return Ordering::Greater };
                let sextra = match self.extra { Some(x) => x, None => panic!() };
                let oextra = match other.extra { Some(x) => x, None => panic!() };
                if sextra != oextra { return sextra.cmp(&oextra); };
            };
            Ordering::Equal
        }
    }
}