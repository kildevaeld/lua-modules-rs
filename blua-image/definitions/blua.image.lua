--- @meta blua.image

local Module = {}

--- @alias ImageFormat 'png' | 'jpeg' | 'jpg' | 'webp'

--- @alias FilterType 'nearest' | 'triangle' | 'catmullrom' | 'gaussian' | 'lanczos3'


--- @class Image
--- @field size integer
--- @field width integer
--- @field height integer
--- @field write fun(this: Image, path: string, format?: ImageFormat)
--- @field thumbnail fun(this: Image, width: integer, height: integer, exact?: boolean): Image
--- @field resize fun(this: Image, width: integer, height: integer, type: FileType, exact?: boolean): Image
--- @field blur fun(this: Image, sigma: number): Image
Image = {}

--- @param path string
--- @return Image
function Module.open(path) end

--- @param bytes Buffer
--- @return Image
function Module.new(bytes) end

return Module
