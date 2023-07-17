--- @meta fs


local FS = {}

--- @param path string
--- @return fun(): DirEntry
function FS.read_dir(path) end

--- @class DirEntry
--- @field path string
--- @field type fun(): FileType
local DirEntry = {}


--- @alias FileType "file" | "dir"


return FS
