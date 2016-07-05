function run()
  windex = focused_window_index()
  if #arguments > 0 then
    focus_webview(windex, arguments[1] - 1)
  end
  return true
end
