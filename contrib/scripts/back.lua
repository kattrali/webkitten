function description()
  return "Navigates back in the focused web view"
end

function run()
  local windex = focused_window_index()
  go_back(windex, focused_webview_index(windex))
  return true
end
