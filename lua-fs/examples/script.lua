local table = require 'table'
local fs = require 'fs'
local string = require 'string'
local class = require 'class'
local stream = require 'stream'



local rd = stream.filter(fs.read_dir("."), function(item)
    return item:type() == "file"
end)

for n in rd do
    print(n.path .. " " .. n:type())
    local file = fs.open(n.path)

    local lines = stream.map(file:lines(), function(item)
        return string.sub(item, 0, 10) or ""
    end)

    for line in lines do
        print("line " .. line)
    end

    ::continue::
end
