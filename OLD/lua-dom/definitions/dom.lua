--- @meta

--- @class Document
--- @field select fun(this: Document, sel: string): Selection
Document = {}

--- @class Selection
--- @field text fun(this: Selection): StringList
--- @field map fun(this: Selection, cb: fun(el: Element, idx: number): any): any[] 
--- @field select fun(this: Selection, select: string): Selection
--- @field get fun(this: Selection, idx: number): Element
--- @operator index(number): string
Selection = {}

--- @class Element
--- @field attr fun(this: Element, name: string): StringRef | nil
Element = {}

--- @class StringRef
--- @operator tostring(StringRef): string

--- @class StringList
--- @field trim fun(this: StringList): StringList
--- @field join fun(this: StringList, joine: string): string

local Dom = {}

--- @param arg string
--- @return Document
function Dom.parse(arg) end


return Dom