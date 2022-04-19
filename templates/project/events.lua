local ffi = require('ffi')
ffi.cdef [[
{{cdef}}
]]
return {
{{event_types_map}}
}
