local string = require 'string'
local test = require './test2.lua'
print('Main: ' .. require.current .. " " .. string.upper(test))
