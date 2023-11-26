--- @meta blua.http

--- @alias Method 'GET' | 'POST' | 'PUT' | 'DELETE' | 'HEAD' | 'OPTION'

local Module = {}

--- @class HttpResponse
--- @field status integer
--- @field header {[string]: string}
--- @field text fun(): string
--- @field bytes fun(): Buffer
--- @field json fun(): any
HttpResponse = {}

--- @param url string
--- @return HttpResponse
function Module.get(url) end

--- @param url string
--- @return HttpResponse
function Module.post(url) end

--- @param url string
--- @return HttpResponse
function Module.put(url) end

--- @param url string
--- @return HttpResponse
function Module.delete(url) end

return Module
