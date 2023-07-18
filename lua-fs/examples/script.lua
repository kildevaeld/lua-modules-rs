local fs = require 'core.fs'
local stream = require 'core.stream'


local pipe = stream.pipe(fs.read_dir(".")):filter(function(item)
    return item:type() == "file"
end):map(function(item)
    return fs.open(item.path):lines()
end):flatten()



-- local rd = stream.filter(fs.read_dir("."), function(item)
--     return item:type() == "file"
-- end)

for n in pipe do
    -- for line in n do
    --     print("line " .. type(line))
    -- end

    print("line " .. n)

    ::continue::
end
