#[macro_export]
macro_rules! helper {
    (exists_or_zero;) => {
        1
    };
    (exists_or_zero;$t:tt) => {
        $t
    };
    (exists;,$then:ty) => {
        $then
    };
    (exists;$t:tt, $then:ty) => {
        Vec<$then>
    };
    (exists_flag;,$then:ty) => {
        Option<$then>
    };
    (exists_flag;$t:tt, $then:ty) => {
        Vec<$then>
    };
    (exists_add;,$var:ident, $type:ty, $d:ident, $db:ident) => {
        $var = <$type>::from($d.get($db).expect("this is an error on line 22, please report this bug to https://github.com/chickencuber/clilib").clone());
        $d.remove($db);
    };
    (exists_add;$t:tt,$var:ident, $type:ty, $d:ident, $db:ident) => {
        for _ in 0..$t {
            $var.push(<$type>::from($d.get($db).expect("this is an error on line 27, please report this bug to https://github.com/chickencuber/clilib").clone()));
            $d.remove($db);
        }
    };
    (exists_add_flag;,$var:ident, $type:ty, $d:ident, $db:expr) => {
        $var = Some(<$type>::from($d.get($db).expect("this is an error on line 32, please report this bug to https://github.com/chickencuber/clilib").clone()));
        $d.remove($db);
    };
    (exists_add_flag;$t:tt,$var:ident, $type:ty, $d:ident, $db:expr) => {
        for _ in 0..$t {
            $var.push(<$type>::from($d.get($db).expect("this is an error on line 37, please report this bug to https://github.com/chickencuber/clilib").clone()));
            $d.remove($db);
        }
    };
    (exists_declare;,$var:ident, $type:ty) => {
        let $var: $type;
    };
    (exists_declare;$t:tt,$var:ident, $type:ty) => {
        let mut $var: Vec<$type> = Vec::new();
    };
    (exists_declare_flag;,$var:ident, $type:ty) => {
        let mut $var: Option<$type> = None;
    };
    (exists_declare_flag;$t:tt,$var:ident, $type:ty) => {
        let mut $var: Vec<$type> = Vec::new();
    };
}

#[macro_export]
macro_rules! cmd {
    ($name:ident; help:$help:expr; :$default:expr; $($fname:expr=> $($str:literal)|*),*$(,)?) => {
        pub struct $name;
        impl $name {
            pub fn run(v: Vec<String>) {
                if v.len() == 0 {
                    $default(v);
                    return;
                }
                let cmd = v.get(0).expect(&format!("{}", $help)).clone();
                if cmd == "help" || cmd == "?" {
                    println!("{}", $help);
                    std::process::exit(0);
                } else
                    $(
                        if $(cmd == $str)||* {
                            $fname(v);
                        } else
                    )* {
                        $default(v);
                    }
            }
        }
    };
    ($name:ident; help:$help:expr; .$cmd:ty; :$default:expr; $($fname:expr=> $($str:literal)|*),*$(,)?) => {
        pub struct $name;
        impl $name {
            pub fn run(mut v: Vec<String>) {
                let mut cmd: Option<String> = None;
                let mut allow_flags = false;

                for i in 0..v.len() {
                    let a = &v[i];
                    if allow_flags {
                        cmd = Some(a.clone());
                        v.remove(i);
                        break;
                    }
                    if a == "--" {
                        allow_flags = true;
                        continue;
                    }
                    if a.starts_with("-") {
                        continue;
                    }
                    cmd = Some(a.clone());
                    v.remove(i);
                    break;
                }
                if let Some(cmd) = cmd {
                    if cmd == "help" || cmd == "?" {
                        println!("{}", $help);
                        std::process::exit(0);
                    } else
                        $(
                            if $(cmd == $str)||* {
                                let args = <$cmd>::from(v.clone());
                                $fname(args);
                            } else
                        )* {
                            v.insert(0, cmd);
                            let args = <$cmd>::from(v.clone());
                            $default(args);
                            }
                } else {
                    let args = <$cmd>::from(v.clone());
                    $default(args);
                }
            }
        }
    };
    ($name:ident; help:$help:expr; $(.$cmd:ty;)? $($fname:expr=> $($str:literal)|*),*$(,)?) => {
        fn _default<T>(_: T) {
            eprintln!("{}", $help);
            std::process::exit(101);
        }
        $crate::cmd!{
            $name;
            help:$help;
            $(.$cmd;)?
            :_default;
            $($fname => $($str)|*),*
        }
    };
    (help:$help:expr; $(.$cmd:ty;)? $(:$default:expr;)? $($fname:expr=> $($str:literal)|*),*$(,)?) => {
        fn main() {
            $crate::cmd!{
                _Main;
                help:$help;
                     $(.$cmd;)?
                         $(:$default;)?
                         $($fname => $($str)|*),*
            }
            _Main::run(std::env::args().skip(1).collect());
        }
    };
}

#[macro_export]
macro_rules! define {
    ($name:ident; help: $help:expr; flags {
        $($fname:ident: $ftype:ty = $($flag:literal)|* $(=> [$fnum:literal])?),*$(,)?
    }; args {
        $($aname:ident: $atype:ty $(=> [$num:literal])?),*$(,)?
    };
    $(rest => $rname:ident: $rtype:ty;)?) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $fname: tt_call::tt_if!{
                condition = [{tt_equal::tt_equal}]
                    input = [{ $ftype bool }]
                    true = [{
                        bool
                    }]
                false = [{
                    $crate::helper!(exists_flag;$($fnum)?, $ftype)
                }]
            },)*
            $(
                pub $aname: $crate::helper!(exists;$($num)?, $atype),
            )*
                $(
                    pub $rname: Vec<$rtype>
                )?
        }
        impl $name {
            pub fn from(mut __args: Vec<String>) -> Self{
                let mut __handle_flags = true;
                $(
                    tt_call::tt_if!{
                        condition = [{tt_equal::tt_equal}]
                            input = [{ $ftype bool }]
                            true = [{
                                let mut $fname: bool = false;
                            }]
                        false = [{
                            $crate::helper!(exists_declare_flag;$($fnum)?, $fname, $ftype);
                        }]
                    }
                )*
                    $(
                        $crate::helper!(exists_declare;$($num)?,$aname, $atype);
                    )*
                    $(
                        let mut $rname: Vec<$rtype> = Vec::new();
                    )?
                    if
                        __args.contains(&"-help".to_string()) ||
                        __args.contains(&"--help".to_string()) ||
                        __args.contains(&"-?".to_string()) ||
                        __args.contains(&"--?".to_string())
                    {
                        eprintln!("{}", $help);
                        std::process::exit(0);
                    }
                let mut __i = 0;
                $(
                    if __args.len() == 0 {
                        eprintln!("missing argument: '{}'", stringify!($aname));
                        std::process::exit(101);
                    }
                    while __args.get(__i).expect("there is an error on line 146, please report this bug to https://github.com/chickencuber/clilib").starts_with("-") && __handle_flags {
                        let v = Self::has_args(__args.get(__i).expect("there is an error on line 147, please report this bug to https://github.com/chickencuber/clilib").clone());
                        __args.get_mut(__i).expect("there is an error on line 148, please report this bug to https://github.com/chickencuber/clilib").insert(0, '-');
                        if v.0 {
                            __i += v.1;
                            if __args.get(__i).expect("there is an error on line 151, please report this bug to https://github.com/chickencuber/clilib").starts_with("-") && __handle_flags {
                                eprintln!("the flag requires an argument");
                                std::process::exit(101);
                            }
                        }
                        __i += 1;
                    }

                    $crate::helper!(exists_add; $($num)?, $aname, $atype, __args, __i);
                )*
                    $(
                        while __args.len() - __i > 0 {
                            if __args.get(__i).expect("there is an error on line 163, please report this bug to https://github.com/chickencuber/clilib").starts_with("-") && __handle_flags {
                                if $rname.len() > 0 {
                                    break;
                                }
                                while __args.get(__i).unwrap_or(&String::new()).starts_with("-") && __handle_flags {
                                    let v = Self::has_args(__args.get(__i).expect("there is an error on line 168, please report this bug to https://github.com/chickencuber/clilib").clone());
                                    if __args[__i] == "--" {
                                        __handle_flags = false;
                                        __args.remove(__i);
                                        break;
                                    }
                                    __args.get_mut(__i).expect("there is an error on line 169, please report this bug to https://github.com/chickencuber/clilib").insert(0, '-');
                                    if v.0 {
                                        __i += v.1;
                                        if __args.get(__i).unwrap_or(&String::from("-")).starts_with("-") && __handle_flags {
                                            eprintln!("the flag requires an argument");
                                            std::process::exit(101);
                                        }
                                    }
                                    __i += 1;
                                }
                            }

                            if let Some(s) = __args.get(__i) {
                                $rname.push(<$rtype>::from(s.clone()));
                                __args.remove(__i);
                            }

                        }
                )?
                    while __args.len() > 0 {
                        let mut __ch = __args.get(0).expect("there is an error on line 186, please report this bug to https://github.com/chickencuber/clilib").clone();
                        if !(__ch.starts_with("-") && __handle_flags) {
                            eprintln!("too many arguments");
                            std::process::exit(101);
                        }
                        __ch.remove(0);
                        if __ch.starts_with("-") && __handle_flags {
                            __ch.remove(0);
                        }
                        $(
                            if $(__ch == $flag)||* {
                                tt_call::tt_if!{
                                    condition = [{tt_equal::tt_equal}]
                                        input = [{ $ftype bool }]
                                        true = [{
                                            $fname = true;
                                        }]
                                    false = [{
                                        if __args[1].starts_with("-") && __handle_flags {
                                            eprintln!("flags requires an argument");
                                            std::process::exit(101);
                                        }
                                        $crate::helper!(exists_add_flag;$($fnum)?, $fname, $ftype, __args, 1);
                                    }]
                                }
                                __args.remove(0);
                                continue;
                            } else
                        )*
                            if (__ch == "-") {
                                __handle_flags = false;
                                __args.remove(0);
                                break
                            }
                            eprintln!("invalid flag {}", __ch);
                        std::process::exit(101);
                    }
                return Self {
                    $($fname,)*
                        $($aname,)*
                        $($rname)?
                }
            }
            fn has_args(v: String) -> (bool, usize) {
                $(
                    if $(v == concat!("-", $flag) || v == concat!("--", $flag))||* {
                        tt_call::tt_if!{
                            condition = [{tt_equal::tt_equal}]
                                input = [{ $ftype bool }]
                                true = [{
                                    return (false, 0);
                                }]
                            false = [{
                                return (true, $crate::helper!(exists_or_zero;$($fnum)?));
                            }]
                        }
                    } else
                )*
                    if(v == "--") {
                        return (false, 0);
                    }
                eprintln!("invalid flag {}", v);
                std::process::exit(101);
            }
        }
    };
}
