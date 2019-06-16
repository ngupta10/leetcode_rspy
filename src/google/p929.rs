/*
 * Every email consists of a local name and a domain name, separated by the @ sign.
 * For example, in alice@leetcode.com, alice is the local name, and leetcode.com is the domain name.
 *
 * Besides lowercase letters, these emails may contain '.'s or '+'s.
 * If you add periods ('.') between some characters in the local name part of an email address,
 * mail sent there will be forwarded to the same address without dots in the local name.
 *
 * For example, "alice.z@leetcode.com" and "alicez@leetcode.com" forward to the same email address.
 * (Note that this rule does not apply for domain names.)
 *
 * If you add a plus ('+') in the local name, everything after the first plus sign will be ignored.
 * This allows certain emails to be filtered, for example m.y+name@email.com will be forwarded to my@email.com.
 * (Again, this rule does not apply for domain names.)
 *
 * It is possible to use both of these rules at the same time.
 * Given a list of emails, we send one email to each address in the list.  How many different addresses actually receive mails?
*/
use std::collections::HashSet;


pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut uniques: HashSet<String> = HashSet::new();

    for email in emails {
        let split = email.split("@");
        let vec: Vec<&str> = split.collect();
        match vec.as_slice() {
            [] => continue,
            [local, domain] => {
                let clean = local.replace(".", "");
                let local_vec: Vec<&str> = clean.split('+').collect();
                uniques.insert(format!("{}@{}", local_vec[0].to_string(), domain));
            }
            _ => continue
        }
    }

    return uniques.len() as i32

}

#[cfg(test)]
mod test {
    use super::num_unique_emails;

    #[test]
    fn example1() {
        let input = [
            String::from("test.email+alex@leetcode.com"),
            String::from("test.e.mail+bob.cathy@leetcode.com"),
            String::from("testemail+david@lee.tcode.com")];
        assert_eq!(num_unique_emails(input.to_vec()), 2);
    }

    #[test]
    fn example2() {
        let input = [
            String::from("fg.r.u.uzj+o.pw@kziczvh.com"),
            String::from("r.cyo.g+d.h+b.ja@tgsg.z.com"),
            String::from("fg.r.u.uzj+o.f.d@kziczvh.com"),
            String::from("r.cyo.g+ng.r.iq@tgsg.z.com"),
            String::from("fg.r.u.uzj+lp.k@kziczvh.com"),
            String::from("r.cyo.g+n.h.e+n.g@tgsg.z.com"),
            String::from("fg.r.u.uzj+k+p.j@kziczvh.com"),
            String::from("fg.r.u.uzj+w.y+b@kziczvh.com"),
            String::from("r.cyo.g+x+d.c+f.t@tgsg.z.com"),
            String::from("r.cyo.g+x+t.y.l.i@tgsg.z.com"),
            String::from("r.cyo.g+brxxi@tgsg.z.com"),
            String::from("r.cyo.g+z+dr.k.u@tgsg.z.com"),
            String::from("r.cyo.g+d+l.c.n+g@tgsg.z.com"),
            String::from("fg.r.u.uzj+vq.o@kziczvh.com"),
            String::from("fg.r.u.uzj+uzq@kziczvh.com"),
            String::from("fg.r.u.uzj+mvz@kziczvh.com"),
            String::from("fg.r.u.uzj+taj@kziczvh.com"),
            String::from("fg.r.u.uzj+fek@kziczvh.com")];
        assert_eq!(num_unique_emails(input.to_vec()), 2);
    }

    #[test]
    fn example3() {
        let input = [
            String::from("fxggfzk.fo.q+e@uxbeyetxc.com"),
            String::from("fxggfzk.fo.q+h@uxbeyetxc.com"),
            String::from("uv+mw.lkw+ybe@yppz.com")];
        assert_eq!(num_unique_emails(input.to_vec()), 2);
    }

    #[test]
    fn example4() {
        let input = [
            String::from("a@b.com"),
            String::from("x@y.com"),
            String::from("m@n.com")];
        assert_eq!(num_unique_emails(input.to_vec()), 3);
    }
}
