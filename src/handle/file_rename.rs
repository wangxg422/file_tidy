use crate::command::file::RenameArgs;

pub fn exec(args: &RenameArgs) {
    if !args.dir.exists() {
        println!("path does not exist: {}", args.dir.display());
        return;
    }

    if args.naming_rule.sha1 {

    } else if args.naming_rule.sha256 {

    } else if args.naming_rule.sha3 {

    } else if args.naming_rule.md5 {

    } else if args.naming_rule.sequence {

    }
}

pub enum NamingRule {
    SHA1,
    SHA256,
    SHA3,
    SEQUENCE,
}