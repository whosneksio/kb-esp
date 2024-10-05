local http_lib = require("neverlose/http_lib")

local vec = vector(1, 2, 3)

local http = http_lib.new({
    task_interval = 0.5, 
    enable_debug = true, 
})

vec.x, vec.y, vec.z = 0, 0, 0

local delayed_distance = nil
local delay_timer = 0

events.render:set(function()
    local screen_center = render.screen_size() * 0.5

    local local_player = entity.get_local_player()
    if not local_player or not local_player:is_alive() then
        return
    end

    local camera_position = render.camera_position()
    local camera_angles = render.camera_angles()
    local direction = vector():angles(camera_angles)

    local closest_distance, closest_enemy = math.huge
    for _, enemy in ipairs(entity.get_players(true)) do
        local head_position = enemy:get_hitbox_position(1)
        local ray_distance = head_position:dist_to_ray(camera_position, direction)
        
        if ray_distance < closest_distance then
            closest_distance = ray_distance
            closest_enemy = enemy
        end
    end

    if not closest_enemy then
        return
    end

    local current_time = globals.realtime

    if closest_distance < 100 and delayed_distance == nil then
        delayed_distance = closest_distance
        delay_timer = current_time + 2
    end

    if delayed_distance and current_time >= delay_timer then
        http:get(string.format("http://127.0.0.1:2598/?v=%s", math.floor(closest_distance)), function(data)
        end)
        delayed_distance = nil
    end
end)