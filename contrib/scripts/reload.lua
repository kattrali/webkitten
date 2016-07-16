function description()
  return "Reloads the current view"
end

function run()
  window_index = focused_window_index()
  webview_index = focused_webview_index(window_index)
  disable_blockers = false
  if #arguments == 0 then
    reload_webview(window_index, webview_index, false)
    return true
  elseif #arguments == 1 and (arguments[1] == "f" or arguments[1] == "force") then
    reload_webview(window_index, webview_index, true)
    return true
  end
  log_info("Invalid arguments passed to 'reload'")
  return false
end
