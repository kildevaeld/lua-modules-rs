--- @meta

--- @class Document
--- @field select fun(this: Document, sel: string): Selection
Document = {}

--- @class Selection
--- @field text fun(this: Selection): StringList
--- @field map fun(this: Selection, cb: fun(): any): any[] 

--- @class String


--- @class StringList
--- @field trim fun(this: StringList): StringList
--- @field join fun(this: StringList, joine: string): string

local Dom = {}

--- @param arg string
--- @return Document
function Dom.parse(arg) end


return Dom