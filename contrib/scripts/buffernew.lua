function description()
  return "Opens a new buffer with a URL or configured start page"
end

function run()
  local target = ""
  local windex = focused_window_index()
  if #arguments > 0 then
    target = arguments[1]
  else
    target = lookup_string(config_file_path, "window.start-page")
  end
  open_webview(windex, target)
  return true
end
