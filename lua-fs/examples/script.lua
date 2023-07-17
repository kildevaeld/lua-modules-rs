local table = require 'table'
local fs = require 'fs'
local string = require 'string'
local class = require 'class'
local stream = require 'stream'


local pipe = stream.pipe(fs.read_dir(".")):filter(function(item)
    return item:type() == "file"
end):map(function(item)
    return fs.open(item.path):lines()
end) --:flatten()



-- local rd = stream.filter(fs.read_dir("."), function(item)
--     return item:type() == "file"
-- end)

for n in pipe do
    -- print(n.path .. " " .. n:type())
    -- local file = fs.open(n.path)

    -- local lines = stream.map(n:lines(), function(item)
    --     return string.sub(item, 0, 10) or ""
    -- end)

    for line in n do
        print("line " .. type(line))
    end

    ::continue::
end
