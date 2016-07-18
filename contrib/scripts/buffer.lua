function run()
  windex = focused_window_index()
  if #arguments > 0 then
    index = tonumber(arguments[1])
    if index >= webview_count(windex) then
      return false
    end
    focus_webview(windex, tonumber(arguments[1]))
  end
  return true
end
