print("Building binaries...")
wiz.cmd("cargo", {"build", "--release"}, ".", true, true)