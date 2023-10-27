--- @meta blur.regexp


local Module = {}

--- @class Regexp
local Regexp = {}

--- @param haystack string
--- @return boolean
function Regexp:is_match(haystack) end

--- @param haystack string
--- @return Captures | nil
function Regexp:captures(haystack) end

--- @class Captures
--- @meta len:number
local Captures = {}

--- @param idx integer
--- @return Match | nil
function Captures:get(idx) end

--- @class Match
--- @meta tostring:string
local Match = {}


--- @param content string
--- @return Regexp | nil
function Module.new(content) end

return Module
