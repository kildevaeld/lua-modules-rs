#!/usr/bin/env -S blur run

local config = require 'core.config'
local util = require 'core.util'
local crypto = require 'core.crypto'
local json = require 'core.json'
local env = require 'core.env'
local date = require 'core.time'

local toml = config.read("Cargo.toml")


print(util.dump(toml))


config.write("Config.json", toml)


local hash = crypto.sha256(json.encode(toml));

print("Hash " .. hash:toString("hex"))


print("CWD " .. env.cwd .. " ARGS " .. util.dump(env.args) .. " PATH " .. env.env.PWD);
