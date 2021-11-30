print("Building binaries...")
wiz.cmd("cargo", {"build", "--release"}, ".", true, true)
local bin_name = "qinpel-wiz" .. wiz.exe_ext
print("Binary name = " .. bin_name)
wiz.mv("../../" .. bin_name, "../../" .. bin_name .. "old")
wiz.cp("target/release/" .. bin_name, "../../" .. bin_name) 
