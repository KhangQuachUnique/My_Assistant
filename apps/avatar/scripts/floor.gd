class_name FloorController
extends StaticBody2D

@export var floor_height: float = 60.0
@export var bottom_margin: float = 0

func _ready() -> void:
	get_window().size_changed.connect(_update_floor)
	_update_floor()
	
func _update_floor() -> void:
	var viewport_size := get_viewport_rect().size
	var shape := $CollisionShape2D.shape as RectangleShape2D
	
	shape.size = Vector2(viewport_size.x, floor_height)
	
	position = Vector2(
		viewport_size.x * 0.5,
		viewport_size.y - bottom_margin + floor_height * 0.5
	)
