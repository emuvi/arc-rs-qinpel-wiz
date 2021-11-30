print("Building binary...")
wiz.cmd("cargo", {"build", "--release"}, ".", true, true)
local bin_name = "qinpel-wiz" .. wiz.exe_ext
local bin_origin = "target/release/" .. bin_name
local bin_destiny = "../../" .. bin_name
local bin_old = bin_destiny .. "-old"
wiz.rm(bin_old)
wiz.mv(bin_destiny, bin_old)
wiz.cp(bin_origin, bin_destiny)
