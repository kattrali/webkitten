function run()
  local target = ""
  local windex = focused_window_index()
  if #arguments > 0 then
    target = arguments[1]
  end
  open_webview(windex, target)
  return true
end
