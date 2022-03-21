fn main() {
	std::process::Command::new("folder/echo.bat")
		.args(["hello"])
		.status()
		.unwrap();
}
