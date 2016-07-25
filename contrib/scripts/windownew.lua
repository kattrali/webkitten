function description()
  return "Opens a new window with a URL or configured start page"
end

function run()
  local target = ""
  if #arguments > 0 then
    target = arguments[1]
  else
    target = lookup_string(config_file_path, "window.start-page")
  end
  open_window(target)
  return true
end

