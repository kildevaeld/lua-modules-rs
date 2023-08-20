local config = require 'core.config'
local util = require 'core.util'
local crypto = require 'core.crypto'
local json = require 'core.json'

local toml = config.read("Cargo.toml")


print(util.dump(toml))


config.write("Config.json", toml)


local hash = crypto.sha256(json.encode(toml));

print("Hash " .. hash:toString("hex"))
