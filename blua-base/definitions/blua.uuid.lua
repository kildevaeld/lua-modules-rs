--- @meta blua.uuid

local Module = {}


--- @class Uuid
--- @field to_string fun(): string
--- @field to_bytes fun(): Buffer
Uuid = {}

--- @return Uuid
function Module.new() end

return Module
