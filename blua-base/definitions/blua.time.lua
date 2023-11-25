--- @meta blua.time

--- @class DateTime
--- @field hour integer
--- @field minute integer
--- @field second integer
--- @field year integer
--- @field month integer
--- @field day integer
--- @field utc fun(): DateTime
DateTime = {}



local CoreTime;

--- @param year integer
--- @param month integer
--- @param day integer
--- @return DateTime | nil
function CoreTime.new(year, month, day) end

--- @param date string
--- @return DateTime | nil
function CoreTime.from_rfc2822(date) end

--- @param date string
--- @return DateTime | nil
function CoreTime.from_rfc3339(date) end

--- @return DateTime
function CoreTime.now() end

return CoreTime
