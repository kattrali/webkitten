function run()
  target = ""
  if #arguments > 0 then
    target = arguments[1]
  end
  windex = focused_window_index()
  open_webview(windex, target)
  return true
end
