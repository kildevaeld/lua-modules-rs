#!/usr/bin/env -S blur run

local config = require 'core.config'
local util = require 'core.util'
local crypto = require 'core.crypto'
local json = require 'core.json'
local env = require 'core.env'
local date = require 'core.time'
local regxp = require 'core.regexp'
local hbs = require 'core.hbs'


hbs = hbs.create()


local out = hbs:render("{{name}}", { name = "Rasmus" })

print("HBS: " .. out)

local toml = config.read("Cargo.toml")


print(util.dump(toml))


config.write("Config.json", toml)


local hash = crypto.sha256(json.encode(toml));

print("Hash " .. hash:toString("hex"))


print("CWD " .. env.cwd .. " ARGS " .. util.dump(env.args) .. " PATH " .. env.env.PWD);

local reg = regxp.new [[(\d+)]]

if reg == nil then
    print("Invalid regex")
    return
end

print("Match " .. tostring(reg:is_match("test this 2202")))

local capture = reg:captures("rapper 2303 number")

if capture == nil then
    print("could not capture")
    return
end

print("capture " .. tostring(capture:get(1)))


local all = reg:find("test is a number: 200. This is also a number: 4023")


if all == nil then
    print("Could not find a number")
    return
end


for _, n in ipairs(all) do
    print("Match " .. tostring(n))
end

-- local len = #capture

-- print("Captures " .. tostring(#capture))
