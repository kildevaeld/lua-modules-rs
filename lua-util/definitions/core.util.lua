--- @meta core.util

--- @alias Encoding "hex" | "base64" | "utf8"

local Util = {}

--- @class Buffer
--- @field toString fun(encoding:Encoding):string
--- @field len integer
Buffer = {}

--- @param cmd unknown
--- @return string
function Util.dump(cmd) end

--- @param cmd unknown
--- @return boolean
function Util.is_callable(cmd) end

return Util
