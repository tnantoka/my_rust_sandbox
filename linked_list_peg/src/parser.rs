use crate::list::List;

peg::parser!( pub grammar manager() for str {
  pub rule eval(list: &mut List) -> String
      = term(list)

  rule term(list: &mut List) -> String
      = "unshift" _ d:text() { list.unshift(d); String::new() }
      / "push" _ d:text() { list.push(d); String::new() }
      / "get" _ i:num() {
        match list.get(i) {
          None => String::new(),
          Some(v) => v,
        }
      }
      / "ls" {
        let mut result = String::new();
        if let Some(ref node) = list.head {
          let mut node = node;
          loop {
              result.push_str(&node.data);
              match node.link {
                  None => {
                      break;
                  }
                  Some(ref link) => node = link,
              }
              result.push('\n');
          }
        }
        result
      }

  rule num() -> isize
      = value:$(['0'..='9']+)
      { value.parse().unwrap() }

  rule text() -> String
      = v:$(['a'..='z'|'A'..='Z'|'_']+ ['0'..='9']*)
      { String::from(v) }

  rule _ = [' ']
});

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list = List::new();
        manager::eval("unshift u1", &mut list).unwrap();
        manager::eval("unshift u2", &mut list).unwrap();
        manager::eval("push p1", &mut list).unwrap();
        manager::eval("push p2", &mut list).unwrap();

        assert_eq!("u2", manager::eval("get 0", &mut list).unwrap());
        assert_eq!("u1", manager::eval("get 1", &mut list).unwrap());
        assert_eq!("p1", manager::eval("get 2", &mut list).unwrap());
        assert_eq!("p2", manager::eval("get 3", &mut list).unwrap());

        assert_eq!("u2\nu1\np1\np2", manager::eval("ls", &mut list).unwrap());
    }
}
