--- @meta core.stream


local Stream = {}


--- @class Pipe
--- @operator call:unknown
local Pipe = {}



--- @generic T
--- @generic U
--- @param fn fun(item: T): U
--- @return Pipe
function Pipe:map(fn) end

--- @generic T
--- @param fn fun(item: T): boolean
--- @return Pipe
function Pipe:filter(fn) end

--- @return Pipe
function Pipe:flatten() end

--- @generic T
--- @param stream fun(): T | nil
--- @return Pipe
function Stream.pipe(stream) end

return Stream
