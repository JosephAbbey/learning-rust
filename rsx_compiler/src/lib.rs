use pcre2::bytes::Regex;
use xml::reader::{EventReader, XmlEvent};

fn indent(size: i8) -> String {
  const INDENT: &'static str = "  ";
  (0..size).map(|_| INDENT).fold(
    String::with_capacity(size as usize * INDENT.len()),
    |r, s| r + s,
  )
}

pub fn rsx(input: String) -> String {
  // regular expression to find some XML
  let re = Regex::new(r#"([[:blank:]]*)(?<tag>(<(?<tag_name>\w+)(\s+\w+(=\".*\")?)*\s*>[^<]*((?&tag)[^<]*)*<\/\k<tag_name>>)|(<(\w+)(\s+\w+(=\".*\")?)*\s*\/>))"#).unwrap();

  let mut output = input.clone();
  let mut offset = 0isize;
  for result in re.captures_iter(input.as_bytes()) {
    // split the match into it's components
    let m = result.unwrap();
    let i = String::from_utf8_lossy(m.get(1).unwrap().as_bytes());
    let m = m.get(0).unwrap();
    let a = m.start() as isize + offset;
    let b = m.end() as isize + offset;
    let m = String::from_utf8_lossy(m.as_bytes());
    // println!("{}, {}, {}", m, a, b);
    let mut out = String::new();
    // create an XML reader from the input
    let parser = EventReader::from_str(&m);
    let mut depth = 0i8;
    for e in parser {
      match e {
        // on opening tag, indent and add the iced code
        Ok(XmlEvent::StartElement {
          name, attributes, ..
        }) => {
          let mut attrs = String::new();
          for attr in attributes {
            if attr.value.chars().next().unwrap().to_string() == "{"
              && attr.value.chars().last().unwrap().to_string() == "}"
            {
              let mut value = attr.value.clone();
              value.remove(0);
              value.remove(value.len() - 1);
              attrs += (".".to_string() + &attr.name.local_name + "(" + &value + ")").as_str();
            } else {
              attrs +=
                (".".to_string() + &attr.name.local_name + "(\"" + &attr.value + "\")").as_str();
            }
          }
          if depth == 0 {
            out.push_str(&format!("{}{}{}::new(){}\n", i, indent(depth), name, attrs));
            depth += 1;
          } else {
            out.push_str(&format!(
              "{}{}.push(\n{}{}{}::new(){}\n",
              i,
              indent(depth),
              i,
              indent(depth + 1),
              name,
              attrs
            ));
            depth += 2;
          }
        }
        // on closing tag, dedent
        Ok(XmlEvent::EndElement { .. }) => {
          depth -= 2;
          if depth < 0 {
            depth = 0;
          }
          if depth == 0 {
            out.push_str(&format!("{}{}\n", i, indent(depth)));
          } else {
            out.push_str(&format!("{}{})\n", i, indent(depth)));
          }
        }
        // on text, add iced code
        Ok(XmlEvent::Characters(chars)) => {
          if chars.chars().next().unwrap().to_string() == "{"
            && chars.chars().last().unwrap().to_string() == "}"
          {
            let mut value = chars.clone();
            value.remove(0);
            value.remove(value.len() - 1);
            out.push_str(&format!(
              "{}{}.push(Text::new({}))\n",
              i,
              indent(depth),
              value
            ));
          } else {
            out.push_str(&format!(
              "{}{}.push(Text::new(\"{}\"))\n",
              i,
              indent(depth),
              chars
            ));
          }
        }
        Err(e) => {
          println!("Error: {}", e);
          break;
        }
        _ => {}
      }
    }
    out += &(i.to_string() + ".into()");
    output.replace_range(a as usize..b as usize, &out);
    offset += out.len() as isize - m.len() as isize;
  }

  output
}
