function run()
  target = ""
  if #arguments > 0 then
    target = arguments[1]
    if not string.find(target, "://") then
      log_debug("appending HTTPS protocol")
      target = string.format("https://%s", target)
    end
  end
  windex = focused_window_index()
  set_address_field_text(windex, target)
  open_webview(windex, target)
  return true
end
