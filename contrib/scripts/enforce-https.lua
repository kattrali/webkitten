function description()
  return "Coerces all URLs to HTTPS"
end

function on_request_uri()
  target = requested_uri
  log_info("Checking URI")
  if string.find(target, "http://") then
    target = target:gsub("http://", "https://")
    log_info("Redirecting to HTTPS")
    load_uri(window_index, webview_index, target)
  end
end
