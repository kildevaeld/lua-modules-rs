--- @meta blua.regexp


local Module = {}

--- @class Regexp
local Regexp = {}

--- @param haystack string
--- @return boolean
function Regexp:is_match(haystack) end

--- @param haystack string
--- @return Match | nil
function Regexp:find_first(haystack) end

--- @param haystack string
--- @return Match[] | nil
function Regexp:find(haystack) end

--- @param haystack string
--- @param replacement string
--- @return string
function Regexp:replace(haystack, replacement) end

--- @param haystack string
--- @param replacement string
--- @return string
function Regexp:replace_all(haystack, replacement) end

--- @param haystack string
--- @return string[]
function Regexp:split(haystack) end

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
