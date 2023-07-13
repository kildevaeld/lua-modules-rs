local table = require 'table'
local fs = require 'fs'


local rd = fs.read_dir(".")

for n in rd do
    if n:type() ~= "file" then
        goto continue
    end
    
    print(n.path .. " " .. n:type())
    fs.read_file(n.path)

    ::continue::
end