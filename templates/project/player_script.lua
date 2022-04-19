function on_create (entity, event)

end

function on_move_up (entity, event)
    entity.position.y -= Engine:time_delta * Game['movement/speed']
end

function on_move_down (entity, event)
    entity.position.y += Engine:time_delta * Game['movement/speed']
end

function on_move_left (entity, event)
    entity.position.x -= Engine:time_delta * Game['movement/speed']
end

function on_move_right (entity, event)
    entity.position.x += Engine:time_delta * Game['movement/speed']
end