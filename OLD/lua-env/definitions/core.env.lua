--- @meta core.env

--- @class Env
--- @field env Environ
--- @field cwd string
--- @field args string[]
local Env = {}

--- @class Environ
--- @field [string] string
--- @field iter fun(): (string,string) | nil


return Env
