local Stream = {}

--- @generic T
--- @param stream function(): T
--- @param filter function(item: T): boolean
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

return Stream
