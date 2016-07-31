function description()
  return "Copy the URL"
end

function run()
  local windex = focused_window_index()
  if windex == NOT_FOUND then
    return false
  end
  copy(webview_uri(windex, focused_webview_index(windex)))
  return true
end
