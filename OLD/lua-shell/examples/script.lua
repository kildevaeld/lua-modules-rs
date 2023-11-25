local shell = require "core.shell"


-- print("CWD " .. shell.cwd .. " Args" .. shell.args)

print(shell.cat("Cargo.toml"):toString("base64"))


local output = shell.sh("cd lua-fs; ls"):output()

-- local code = shell.exec("rofi -dmenu"):run()

-- print("status " .. code)


-- for dir in shell.ls(".") do
--     print("DIR " .. dir)
-- end


-- print(shell.cat("Cargo.toml"))


local output = shell.exec("ls -l"):pipe(shell.exec("rofi -dmenu")):output()

print("" .. output)
