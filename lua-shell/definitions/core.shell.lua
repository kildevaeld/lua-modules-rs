--- @meta core.shell

--- @class Shell
--- @field cwd string
--- @field env Environ
--- @field args string[]
local Shell = {}


--- @class Exec
--- @field status async fun(this: Exec): integer
--- @field output async fun(this: Exec): string
--- @field pipe async fun(this: Exec, e: Exec): Pipe
local Exec = {}


--- @class Pipe
--- @field status async fun(this: Pipe): integer
--- @field output async fun(this: Pipe): string
--- @field pipe async fun(this: Pipe,e: Exec): Pipe
local Pipe = {}


--- @param cmd string
--- @return Exec
function Shell.exec(cmd) end

--- @param cmd string
--- @return Exec
function Shell.sh(cmd) end

--- @param path string
--- @return fun(): string | nil
function Shell.ls(path) end

--- Read path
--- @param path string
--- @return string
function Shell.cat(path) end

--- Write path
--- @param path string
--- @param content string
--- @return string
function Shell.write(path, content) end

--- @param path string
function Shell.mkdir(path) end

--- @param path string
--- @param type? FileType
--- @return boolean
function Shell.test(path, type) end

return Shell
