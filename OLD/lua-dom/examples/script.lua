local table = require 'table'
--- @module 'dom'
local dom = require 'dom'

function readAll(file)
    local f = assert(io.open(file, "rb"))
    local content = f:read("*all")
    f:close()
    return content
end


local dom = dom.parse([[
<ul>
    <li></li>
    <li class="className">
        Hello, World
        <span>Also text</span>
    </li>
</ul>
]])

local sel = dom:select(".className")

print("Test " .. sel:text():trim():join("-"))

local types = sel:map(function (el)
    return tostring(el.type)
end)

-- print("Types " .. table.concat(types, " "))

-- for k,v in pairs(sel) do
--     print(v:text():trim():join(", "))
--     print(v:classes():join(", "))
--     for _,text in pairs(v:text()) do
--         print("test " .. tostring(text))
--     end
-- end

