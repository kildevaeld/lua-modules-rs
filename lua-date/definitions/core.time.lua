--- @meta core.time

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


--- @return DateTime | nil
function CoreTime.from_rfc2822() end

--- @return DateTime | nil
function CoreTime.from_rfc3339() end

--- @return DateTime
function CoreTime.now() end

return CoreTime
