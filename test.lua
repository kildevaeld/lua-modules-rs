local config = require 'core.config'
local util = require 'core.util'

local toml = config.read("Cargo.toml")


print(util.dump(toml))


config.write("Config.json", toml)
