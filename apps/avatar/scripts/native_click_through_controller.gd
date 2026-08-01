class_name NativeClickThroughController
extends Node


@export_category("Native Click Through")

@export var enabled: bool = true
@export var avatar_path: NodePath = NodePath("../Avatar")

## Tần suất kiểm tra vị trí chuột.
## 30 Hz tương đương khoảng 33 ms mỗi lần.
@export_range(10.0, 120.0, 1.0)
var update_rate: float = 30.0


@onready var avatar: AvatarController = (
	get_node_or_null(avatar_path) as AvatarController
)


var _native: Object = null
var _update_timer: Timer = null

var _polygon_id: int = 0
var _polygon_registered: bool = false


func _ready() -> void:
	if not enabled:
		return

	await get_tree().process_frame
	await get_tree().process_frame

	if avatar == null:
		push_error(
			"NativeClickThroughController: Không tìm thấy Avatar."
		)
		return

	if not ClassDB.class_exists(&"NativeClickThrough"):
		push_error(
			"NativeClickThroughController: "
			+ "Class NativeClickThrough chưa được load."
		)
		return

	_native = ClassDB.instantiate(&"NativeClickThrough")

	if _native == null:
		push_error(
			"NativeClickThroughController: "
			+ "Không tạo được NativeClickThrough."
		)
		return

	if not _validate_native_api():
		_native = null
		return

	if not bool(_native.call("is_supported")):
		push_error(
			"NativeClickThroughController: "
			+ "Nền tảng hiện tại không được hỗ trợ."
		)
		_native = null
		return

	if not bool(_native.call("enable")):
		push_error(
			"NativeClickThroughController: enable() thất bại."
		)
		_native = null
		return

	_polygon_id = avatar.get_instance_id()

	if not avatar.hit_region_changed.is_connected(
		_update_avatar_polygon
	):
		avatar.hit_region_changed.connect(
			_update_avatar_polygon
		)

	_update_avatar_polygon()
	_create_update_timer()

	print(
		"NativeClickThrough initialized | Polygon ID: ",
		_polygon_id,
		" | Update rate: ",
		update_rate,
		" Hz"
	)


func _exit_tree() -> void:
	if _update_timer != null:
		_update_timer.stop()

	if _native == null:
		return

	if _polygon_registered:
		_native.call(
			"remove_polygon",
			_polygon_id
		)

	_native.call("disable")

	_polygon_registered = false
	_native = null


func _create_update_timer() -> void:
	_update_timer = Timer.new()
	_update_timer.name = "NativeUpdateTimer"

	_update_timer.wait_time = 1.0 / maxf(update_rate, 1.0)
	_update_timer.one_shot = false
	_update_timer.autostart = true

	# Không phụ thuộc time scale của game.
	_update_timer.ignore_time_scale = true

	_update_timer.timeout.connect(
		_on_update_timer_timeout
	)

	add_child(_update_timer)


func _on_update_timer_timeout() -> void:
	if _native == null:
		return

	# Kiểm tra con trỏ và bật/tắt WS_EX_TRANSPARENT.
	_native.call("update")


func _update_avatar_polygon() -> void:
	if _native == null:
		return

	var polygon := avatar.get_window_hit_polygon()

	if polygon.size() < 3:
		push_error(
			"NativeClickThroughController: "
			+ "Polygon cần ít nhất 3 điểm."
		)
		return

	# Cùng ID thì plugin C++ tự ghi đè polygon cũ.
	_native.call(
		"set_polygon",
		_polygon_id,
		polygon
	)

	_polygon_registered = true


func _validate_native_api() -> bool:
	var required_methods: Array[StringName] = [
		&"is_supported",
		&"enable",
		&"disable",
		&"update",
		&"set_polygon",
		&"remove_polygon",
		&"clear"
	]

	for method_name in required_methods:
		if _native.has_method(method_name):
			continue

		push_error(
			"NativeClickThrough thiếu method: "
			+ String(method_name)
		)

		return false

	return true
