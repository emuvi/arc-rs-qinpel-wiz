print("Building binaries...")
wiz.cmd("cargo", {"build", "--release"}, ".", true, true)
local bin_name = "qinpel-wiz" .. wiz.exe_ext
wiz.cp("target/release/" .. bin_name, "../../" .. bin_name) 
