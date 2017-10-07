error_chain! {
	errors {
		TunNameTooLong
		InvalidTunName
		InvalidTunAddress

        InitCryptoFailed
	}

	foreign_links {
		Io(::std::io::Error);
		Nul(::std::ffi::NulError);
		ParseNum(::std::num::ParseIntError);


	}
}
