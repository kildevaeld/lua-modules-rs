--- @meta stream


local Stream = {}


--- @class Pipe
local Pipe = {}


--- @generic T
--- @generic U
--- @param fn fun(item: T): U
function Pipe:map(fn) end

--- @generic T
--- @param stream fun(): T | nil
function Stream.pipe(stream) end

return Stream
