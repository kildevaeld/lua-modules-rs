--- @meta core.shell

local Shell = {}

--- @class Exec
--- @field run async fun(this: Exec): integer
--- @field status async fun(this: Exec): string
--- @field pipe async fun(this: Exec, e: Exec): Pipe
local Exec = {}


--- @class Pipe
--- @field run async fun(this: Pipe): integer
--- @field status async fun(this: Pipe): string
--- @field pipe async fun(this: Pipe,e: Exec): Pipe
local Pipe = {}


--- @param cmd string
--- @return Exec
function Shell.exec(cmd) end

--- @param path string
--- @return fun(): string | nil
function Shell.ls(path) end

--- @param path string
--- @return string
function Shell.cat(path) end

return Shell
