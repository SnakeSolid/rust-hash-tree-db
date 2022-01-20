macro_rules! illegal_argument {
    ($env:ident, $message:expr) => {{
        if let Err(error) = $env.throw_new(ILLEGAL_ARGUMENT, $message) {
            eprint!("{}", error);
        }

        return;
    }};
    ($env:ident, $message:expr, $result:expr) => {{
        if let Err(error) = $env.throw_new(ILLEGAL_ARGUMENT, $message) {
            eprint!("{}", error);
        }

        return $result;
    }};
}

macro_rules! database {
    ($env:ident, $handle:ident) => {
        match unsafe { ($handle as *mut JavaDatabase).as_mut() } {
            Some(database) => database,
            None => illegal_argument!($env, "Invalid database handle"),
        }
    };
    ($env:ident, $handle:ident, $result:expr) => {
        match unsafe { ($handle as *mut JavaDatabase).as_mut() } {
            Some(database) => database,
            None => illegal_argument!($env, "Invalid database handle", $result),
        }
    };
}

macro_rules! unwrap {
    ($env:ident, $expression:expr) => {
        match $expression {
            Ok(value) => value,
            Err(error) => {
                let message = format!("{}", error);

                if let Err(error) = $env.throw(message) {
                    eprint!("{}", error);
                }

                return;
            }
        }
    };
    ($env:ident, $expression:expr, $result:expr) => {
        match $expression {
            Ok(value) => value,
            Err(error) => {
                let message = format!("{}", error);

                if let Err(error) = $env.throw(message) {
                    eprint!("{}", error);
                }

                return $result;
            }
        }
    };
}
