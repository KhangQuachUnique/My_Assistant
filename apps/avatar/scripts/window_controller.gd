class_name WindowController
extends Node

@export var screen_index: int = 0
@export var use_usable_rect: bool = true


func _ready() -> void:
	var window := get_window()
	
	_configure_window(window)
	_fit_window_to_screen(window)
	await get_tree().process_frame
	
	_print_window_info(window)
	
func _configure_window(window: Window) -> void:
	window.mode = Window.MODE_WINDOWED
	
	window.transparent = true
	window.borderless = true
	window.transparent_bg = true
	window.unresizable = true
	window.always_on_top = true
	
	Engine.max_fps = 60
	Engine.physics_ticks_per_second = 60
	
	window.content_scale_mode = Window.CONTENT_SCALE_MODE_DISABLED
	
func _fit_window_to_screen(window: Window) -> void:
	var screen_rect := DisplayServer.screen_get_usable_rect(0)
	window.position = screen_rect.position
	window.size = screen_rect.size
	
func _print_window_info(window: Window) -> void:
	print("")
	print("================ WINDOW SETUP ================")
	print("Window position : ", window.position)
	print("Window size     : ", window.size)
	print("Viewport size   : ", window.get_visible_rect().size)
	print("================================================")
