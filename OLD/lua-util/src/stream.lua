local class = require 'core.class'
local util = require 'core.util'


local Stream = {}


local Pipe = class.create(function(pipe, stream)
    pipe.stream = stream
end)

function Pipe:map(map)
    self.stream = Stream.map(self.stream, map)
    return self
end

function Pipe:filter(filter)
    self.stream = Stream.filter(self.stream, filter)
    return self
end

function Pipe:flatten()
    self.stream = Stream.flatten(self.stream)
    return self
end

function Pipe:__call()
    return self.stream()
end

function Stream.pipe(stream)
    return Pipe(stream)
end

--- @generic T
--- @param stream fun(): T
--- @param filter fun(item: T): boolean
function Stream.filter(stream, filter)
    return function()
        for n in stream do
            if n == nil then
                return nil
            end

            if filter(n) then
                return n
            end
        end
    end
end

function Stream.map(stream, map)
    return function()
        local n = stream()
        if n == nil then
            return nil
        end

        return map(n)
    end
end

function Stream.flatten(stream)
    local current = nil
    return function()
        while (true) do
            if current ~= nil then
                local next = current()
                if next ~= nil then
                    return next
                end
                current = nil
            end

            local next = stream()

            if util.is_callable(next) then
                current = next
            else
                return next
            end
        end
    end
end

return Stream
