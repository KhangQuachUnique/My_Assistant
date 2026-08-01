class_name AvatarHitRegion
extends Area2D


signal primary_pressed(pointer_global_position: Vector2)


@onready var collision_polygon: CollisionPolygon2D = (
	$CollisionPolygon2D
)


func _ready() -> void:
	# Area2D phải pickable và có collision layer khác 0
	# để nhận input_event.
	input_pickable = true

	# Dùng layer 2 cho vùng chuột,
	# tách khỏi physics Floor/Avatar ở layer 1.
	collision_layer = 2
	collision_mask = 0

	if not input_event.is_connected(_on_input_event):
		input_event.connect(_on_input_event)


func _on_input_event(
	_viewport: Node,
	event: InputEvent,
	_shape_index: int
) -> void:
	if not event is InputEventMouseButton:
		return

	var mouse_event := event as InputEventMouseButton

	if mouse_event.button_index != MOUSE_BUTTON_LEFT:
		return

	if not mouse_event.pressed:
		return

	primary_pressed.emit(
		get_global_mouse_position()
	)


func get_window_polygon() -> PackedVector2Array:
	var window_polygon := PackedVector2Array()

	for local_point in collision_polygon.polygon:
		window_polygon.append(
			collision_polygon.to_global(local_point)
		)

	return window_polygon


func get_bounds_in(
	target_node: Node2D
) -> Rect2:
	var points := collision_polygon.polygon

	if points.is_empty():
		return Rect2()

	var first_point := target_node.to_local(
		collision_polygon.to_global(points[0])
	)

	var minimum := first_point
	var maximum := first_point

	for index in range(1, points.size()):
		var target_local_point := target_node.to_local(
			collision_polygon.to_global(points[index])
		)

		minimum.x = minf(
			minimum.x,
			target_local_point.x
		)
		minimum.y = minf(
			minimum.y,
			target_local_point.y
		)

		maximum.x = maxf(
			maximum.x,
			target_local_point.x
		)
		maximum.y = maxf(
			maximum.y,
			target_local_point.y
		)

	return Rect2(
		minimum,
		maximum - minimum
	)
