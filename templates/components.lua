local ffi = require('ffi')
ffi.cdef [[
{{cdef}}
]]
local core = require('mm_core')
core:register_components({
{{component_map}}
})