#!/usr/bin/env -S blur run

local config = require 'blua.config'
local util = require 'blua.util'
local crypto = require 'blua.crypto'
local json = require 'blua.json'
local env = require 'blua.env'
local date = require 'blua.time'
local regxp = require 'blua.regexp'
local hbs = require 'blua.hbs'
local http = require 'blua.http'


local resp = http.get("http://google.com");




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
