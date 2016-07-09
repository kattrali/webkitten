function description()
  return "Copy the URL"
end

function run()
  windex = focused_window_index()
  copy(webview_uri(windex, focused_webview_index(windex)))
  return true
end
