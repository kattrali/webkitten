function description()
  return "Cycles to the next buffer in the window"
end

function run()
  windex = focused_window_index()
  target = focused_webview_index(windex) + 1
  if target >= webview_count(windex) then
    target = 0
  end
  focus_webview(windex, target)
  return true
end
