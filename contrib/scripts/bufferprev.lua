function description()
  return "Cycles to the previous buffer in the window"
end

function run()
  windex = focused_window_index()
  target = focused_webview_index(windex) - 1
  if target < 0 then
    target = webview_count(windex) - 1
  end
  focus_webview(windex, target)
  return true
end
