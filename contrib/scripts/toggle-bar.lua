function description()
  return "Toggle the visibility of the command bar"
end

function run()
  local windex = focused_window_index()
  local visible = command_field_visible(windex) == false
  set_command_field_visible(windex, visible)
  if visible then
    focus_commandbar_in_window(windex)
  else
    focus_webview_in_window(windex)
  end
  return true
end
