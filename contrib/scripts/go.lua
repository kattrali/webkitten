function description()
  return "Open a web page"
end

function run()
  if #arguments > 0 then
    windex = focused_window_index()
    target = arguments[1]
    load_uri(windex, focused_webview_index(windex), target)
    return true
  end
  log_debug("No URL specified")
  return false
end
