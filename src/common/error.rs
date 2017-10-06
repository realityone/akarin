error_chain! {
	errors {
		NameTooLong
		InvalidName
		InvalidAddress
	}

	foreign_links {
		Io(::std::io::Error);
		Nul(::std::ffi::NulError);
		ParseNum(::std::num::ParseIntError);
	}
}
