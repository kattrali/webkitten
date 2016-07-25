function description()
  return "Coerces HTTP URLs to HTTPS"
end

-- Redirects from HTTP to HTTPS, except where excluded in the configuration
-- option `enforce-https.ignored-hosts`
function on_request_uri()
  target = requested_uri
  if string.find(target, ".local") or string.find(target, "localhost") then
    return
  end
  ignored_hosts = lookup_strings(config_file_path, "enforce-https.ignored-hosts")
  host = string.gmatch(string.match(requested_uri, "://(.*)"), "[^/]+")()
  for _, ignored_host in ipairs(ignored_hosts) do
    if ignored_host == host then
      return
    end
  end
  if string.find(target, "http://") then
    target = target:gsub("http://", "https://")
    log_info("Redirecting to HTTPS")
    load_uri(window_index, webview_index, target)
  end
end
