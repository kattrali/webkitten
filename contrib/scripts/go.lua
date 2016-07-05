-- This script's purpose
function description()
  return "Open a web page"
end

-- Run default function. The current scope should include `arguments` as a
-- function which returns relevant options. Should return a boolean indicating
-- success or failure
function run()
  if #arguments > 0 then
    windex = focused_window_index()
    target = arguments[1]
    log_debug("validating URL: " .. target)
    if not string.find(target, "://") then
      log_debug("appending HTTPS protocol")
      target = string.format("https://%s", target)
    end
    log_debug("Focused window: " .. windex)
    load_uri(windex, focused_webview_index(windex), target)
    return true
  end
  log_debug("No URL specified")
  return false
end

-- Provide an array of completions given a prefix. The current scope should
-- include a `prefix` function which returns the relevant state. Should return
-- an array of strings
function complete_command()
end
