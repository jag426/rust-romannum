#![feature(core, plugin)]
#![plugin(peg_syntax_ext)]

pub fn parse_string(n: String) -> Option<u64> {
    parse(n.as_slice())
}

pub fn parse(n: &str) -> Option<u64> {
    match grammar::numeral(n) {
        Ok(r) => Some(r),
        _ => None
    }
}

peg! grammar(r#"
ones -> u64
    = [iI]* { (pos - start_pos) as u64 }
onesplace -> u64
    = [iI] [xX] { 9u64 }
    / [iI] [vV] { 4u64 }
    / [vV] is:ones { 5u64 + is }
    / ones
tens -> u64
    = [xX]* { (pos - start_pos) as u64 * 10u64 }
tensplace -> u64
    = [xX] [cC] { 90u64 }
    / [xX] [lL] { 40u64 }
    / [lL] xs:tens { 50u64 + xs }
    / tens
hundreds -> u64
    = [cC]* { (pos - start_pos) as u64 * 100u64 }
hundredsplace -> u64
    = [cC] [mM] { 900u64 }
    / [cC] [dD] { 400u64 }
    / [dD] cs:hundreds { 500u64 + cs }
    / hundreds
thousandsplace -> u64
    = [mM]* { (pos - start_pos) as u64 * 1000u64 }
#[pub]
numeral -> u64
    = m:thousandsplace c:hundredsplace x:tensplace i:onesplace { m + c + x + i }
"#);

#[test]
fn test() {
    assert!(parse("") == Some(0u64));
    assert!(parse("mmmdccclxxxviii") ==
            parse("MMMDCCCLXXXVIII"));
    assert!(parse("mmmdccclxxxviii") ==
            parse("MmMdcCcLXxXViIi"));

    assert!(parse("iii") == Some(3u64));
    assert!(parse("xxx") == Some(30u64));
    assert!(parse("ccc") == Some(300u64));
    assert!(parse("mmm") == Some(3000u64));
    assert!(parse("vii") == Some(7u64));
    assert!(parse("lxvi") == Some(66u64));
    assert!(parse("cl") == Some(150u64));
    assert!(parse("mcc") == Some(1200u64));
    assert!(parse("iv") == Some(4u64));
    assert!(parse("iiii") == Some(4u64));
    assert!(parse("ix") == Some(9u64));
    assert!(parse("viiii") == Some(9u64));
    assert!(parse("xc") == Some(90u64));
    assert!(parse("xix") == Some(19u64));
    assert!(parse("mmxiv") == Some(2014u64));
    assert!(parse("mcmxcix") == Some(1999u64));
    assert!(parse("xxv") == Some(25u64));
    assert!(parse("mdclxvi") == Some(1666u64));
    assert!(parse("mmmdccclxxxviii") == Some(3888u64));

    assert!(parse("vv") == None);
    assert!(parse("xiix") == None);
    assert!(parse("iix") == None);
    assert!(parse("mcmxcc") == None);
    assert!(parse("mcmxcixi") == None);
}
