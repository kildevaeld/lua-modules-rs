--- @meta blua.fs



local FS = {}

--- @param path string
--- @return fun(): DirEntry
function FS.read_dir(path) end

--- @class DirEntry
--- @field path string
--- @field type fun(): FileType
local DirEntry = {}


--- @alias FileType "file" | "dir"


--- @param path string
--- @return File
function FS.open(path) end

--- @class File
--- @field path string
--- @field lines fun(): fun(): string
--- @field read fun(): string
--- @field readString fun(): string
local File = {}


return FS
