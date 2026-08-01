class_name AvatarController
extends CharacterBody2D


signal hit_region_changed


enum State {
	IDLE,
	SPEAKING
}


@export_category("Physics")

@export var gravity: float = 2500.0
@export var max_fall_speed: float = 2500.0

## Càng lớn, avatar càng nhanh mất quán tính ngang.
@export var horizontal_damping: float = 2.0

## Vận tốc nhỏ hơn mức này sẽ được đặt về 0.
@export var stop_speed: float = 10.0


@export_category("Spawn")

## Tọa độ Y của gốc Avatar khi spawn.
@export var spawn_height: float = 150.0


@export_category("Drag")

@export var drag_enabled: bool = true

## Hệ số lực ném khi thả chuột.
@export_range(0.0, 2.0, 0.05)
var throw_strength: float = 1.0

## Vận tốc ném tối đa.
@export var max_throw_speed: float = 2200.0

## Độ nhạy khi tính vận tốc kéo.
@export var throw_smoothing: float = 18.0


@export_category("Screen Bounds")

## Khoảng cách giữa hit region và cạnh màn hình.
@export var horizontal_margin: float = 10.0

## 0: không bật lại.
## 1: giữ nguyên toàn bộ vận tốc.
@export_range(0.0, 1.0, 0.05)
var wall_bounce: float = 0.8


@onready var animated_sprite: AnimatedSprite2D = (
	$AnimatedSprite2D
)

@onready var hit_region: AvatarHitRegion = (
	$HitRegion
)


var state: State = State.IDLE

var is_dragging: bool = false
var drag_offset: Vector2 = Vector2.ZERO

var _drag_target: Vector2 = Vector2.ZERO
var _previous_drag_position: Vector2 = Vector2.ZERO
var _throw_velocity: Vector2 = Vector2.ZERO


func _ready() -> void:
	var viewport_size := get_viewport_rect().size

	global_position = Vector2(
		viewport_size.x * 0.5,
		spawn_height
	)

	if not hit_region.primary_pressed.is_connected(
		_on_hit_region_primary_pressed
	):
		hit_region.primary_pressed.connect(
			_on_hit_region_primary_pressed
		)

	set_state(State.IDLE)

	# Sau khi tất cả node sẵn sàng,
	# báo cho native controller lấy polygon lần đầu.
	hit_region_changed.emit()


func _physics_process(delta: float) -> void:
	var previous_position := global_position

	if is_dragging:
		_update_drag(delta)
		_handle_horizontal_bounds(false)
	else:
		_apply_gravity(delta)
		_apply_horizontal_damping(delta)

		move_and_slide()

		_handle_horizontal_bounds(true)

	if not global_position.is_equal_approx(previous_position):
		hit_region_changed.emit()


func _input(event: InputEvent) -> void:
	if not is_dragging:
		return

	if not event is InputEventMouseButton:
		return

	var mouse_event := event as InputEventMouseButton

	if mouse_event.button_index != MOUSE_BUTTON_LEFT:
		return

	if not mouse_event.pressed:
		_stop_drag()


func _on_hit_region_primary_pressed(
	pointer_global_position: Vector2
) -> void:
	if not drag_enabled:
		return

	_start_drag(pointer_global_position)


func _start_drag(
	pointer_global_position: Vector2
) -> void:
	if is_dragging:
		return

	is_dragging = true

	drag_offset = (
		pointer_global_position
		- global_position
	)

	_drag_target = global_position
	_previous_drag_position = global_position
	_throw_velocity = Vector2.ZERO

	velocity = Vector2.ZERO


func _stop_drag() -> void:
	if not is_dragging:
		return

	is_dragging = false

	velocity = (
		_throw_velocity
		* throw_strength
	)

	velocity = velocity.limit_length(
		max_throw_speed
	)


func _update_drag(delta: float) -> void:
	_drag_target = (
		get_global_mouse_position()
		- drag_offset
	)

	var safe_delta := maxf(delta, 0.0001)

	var current_drag_velocity := (
		_drag_target
		- _previous_drag_position
	) / safe_delta

	var smoothing_weight := (
		1.0
		- exp(-throw_smoothing * delta)
	)

	_throw_velocity = _throw_velocity.lerp(
		current_drag_velocity,
		smoothing_weight
	)

	global_position = _drag_target
	_previous_drag_position = global_position

	velocity = Vector2.ZERO


func _apply_gravity(delta: float) -> void:
	if is_on_floor():
		if velocity.y > 0.0:
			velocity.y = 0.0

		return

	velocity.y = minf(
		velocity.y + gravity * delta,
		max_fall_speed
	)


func _apply_horizontal_damping(delta: float) -> void:
	if absf(velocity.x) <= stop_speed:
		velocity.x = 0.0
		return

	var damping_amount := (
		horizontal_damping
		* absf(velocity.x)
		* delta
	)

	velocity.x = move_toward(
		velocity.x,
		0.0,
		damping_amount
	)


func _handle_horizontal_bounds(
	allow_bounce: bool
) -> void:
	var viewport_width := get_viewport_rect().size.x

	var local_bounds := hit_region.get_bounds_in(self)

	if local_bounds.size.x <= 0.0:
		return

	var left_extent := local_bounds.position.x
	var right_extent := local_bounds.end.x

	var minimum_avatar_x := (
		horizontal_margin
		- left_extent
	)

	var maximum_avatar_x := (
		viewport_width
		- horizontal_margin
		- right_extent
	)

	if global_position.x < minimum_avatar_x:
		global_position.x = minimum_avatar_x

		if allow_bounce and velocity.x < 0.0:
			velocity.x = (
				-velocity.x
				* wall_bounce
			)

	elif global_position.x > maximum_avatar_x:
		global_position.x = maximum_avatar_x

		if allow_bounce and velocity.x > 0.0:
			velocity.x = (
				-velocity.x
				* wall_bounce
			)


func set_state(new_state: State) -> void:
	if (
		state == new_state
		and animated_sprite.is_playing()
	):
		return

	state = new_state

	match state:
		State.IDLE:
			animated_sprite.play("idle")

		State.SPEAKING:
			animated_sprite.play("speaking")


func set_drag_enabled(enabled: bool) -> void:
	drag_enabled = enabled

	if not drag_enabled and is_dragging:
		_stop_drag()


func get_window_hit_polygon() -> PackedVector2Array:
	return hit_region.get_window_polygon()
