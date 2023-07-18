local fs = require 'core.fs'
local stream = require 'core.stream'
local http = require 'core.http'

local client = http.client()

local resp = client:get("https://jsonplaceholder.typicode.com/todos/1")()
