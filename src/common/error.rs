error_chain! {
    errors {
        // Tun
        TunNameTooLong
        InvalidTunName
        InvalidTunAddress

        // Crypto
        InitCryptoFailed
        CryptoPasswordAlreadySetted
	}

    foreign_links {
        Io(::std::io::Error);
        Nul(::std::ffi::NulError);
        ParseNum(::std::num::ParseIntError);
    }
}
