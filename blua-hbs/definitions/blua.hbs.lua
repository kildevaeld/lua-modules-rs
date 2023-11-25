--- @meta blua.hbs


local Hbs = {}

--- @class Handlebars
--- @field render fun(this: Handlebars, template: string, ctx: unknown): string
local Handlebars = {}

--- @return Handlebars
function Hbs.create() end

return Hbs
