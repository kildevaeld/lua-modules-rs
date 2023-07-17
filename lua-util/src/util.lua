local Util = {}

function Util.is_callable(var)
    if type(var) == 'function' then
        return true
    else
        local b = pcall(function() var() end)
        return b
    end
end

return Util
