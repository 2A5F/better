use crate::*;

pub fn build_template_string(s: &LitStr) -> TokenStream2 {
    let s = s.value();
    let args = parse_temstr(&s.chars().collect::<Box<_>>()).into_iter().map(|v| format_ident!("{}", v));
    quote! { format!(#s #(, #args = #args)* ) }
}

fn parse_temstr(mut s: &[char]) -> Vec<String> {
    let mut items = vec![];
    let mut argi: Option<usize> = None;
    'root: while !s.is_empty() {
        if let Some(i) = &mut argi {
            loop {
                match s.get(*i) {
                    Some('}') | Some(':') => {
                        if *i > 0 {
                            let arg: String = (&s[..*i]).iter().collect();
                            items.push(arg);
                        }
                        s = &s[*i + 1..];
                        argi = None;
                        continue 'root;
                    }
                    None => break 'root,
                    _ => {
                        *i += 1;
                    }
                }
            }
        } else {
            if let Some('{') = s.first() {
                if let Some('{') = s.get(1) {
                    s = &s[2..];
                    continue;
                }
                argi = Some(0);
            }
            s = &s[1..];
            continue;
        }
    }
    items
}

#[test]
fn test_parse_temstr() {
    let code = "ASD{a}123{B:xxx} foo";
    let r = parse_temstr(&code.chars().collect::<Box<_>>());
    println!("{:?}", r);
    assert_eq!(r, vec!["a", "B"]);
}
