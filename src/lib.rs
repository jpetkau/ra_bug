peg::parser!(
    pub grammar gram() for str {
        pub rule arule() -> i32 = precedence!{
            t:"x" { 0 }
        }
    }
);
