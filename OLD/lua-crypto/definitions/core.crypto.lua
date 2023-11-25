--- @meta core.crypto

local Crypto = {}

--- @param data string
--- @return Buffer
function Crypto.sha256(data) end

--- @param data string
--- @return Buffer
function Crypto.sha512(data) end

--- @param data string
--- @return Buffer
function Crypto.md5(data) end

return Crypto
