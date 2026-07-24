extends Node2D

@onready var avatar: Sprite2D = $AvatarSprite

var speed: float = 250.0

var jump_velocity: float = 0.0
var gravity: float = 1200.0
var jump_force: float = 1000.0

var ground_y: float
var is_jumping: bool = false


func _ready() -> void:
	ground_y = avatar.position.y


func _process(delta: float) -> void:
	handle_movement(delta)
	handle_jump(delta)


func handle_movement(delta: float) -> void:
	var direction := Vector2.ZERO

	if Input.is_key_pressed(KEY_D):
		direction.x += 1.0

	if Input.is_key_pressed(KEY_A):
		direction.x -= 1.0
	
	if Input.is_key_pressed(KEY_W):
		direction.y += 1.0
	
	if Input.is_key_pressed(KEY_S):
		direction.y -= 1.0
		
	avatar.position.x += direction.x * speed * delta

	if direction.x < 0:
		avatar.flip_h = false
	elif direction.x > 0:
		avatar.flip_h = true 


func handle_jump(delta: float) -> void:
	if Input.is_key_pressed(KEY_SPACE) and not is_jumping:
		jump_velocity = -jump_force
		is_jumping = true

	if is_jumping:
		jump_velocity += gravity * delta
		avatar.position.y += jump_velocity * delta

		# Đang bay lên
		if jump_velocity < 0:
			avatar.rotation_degrees = lerp(
				avatar.rotation_degrees,
				0.0,
				8.0 * delta
			)

		# Đang rơi xuống
		else:
			avatar.rotation_degrees = lerp(
				avatar.rotation_degrees,
				180.0,
				8.0 * delta
			)

		if avatar.position.y >= ground_y:
			avatar.position.y = ground_y
			jump_velocity = 0.0
			is_jumping = false
			avatar.rotation_degrees = 0.0
