-- This script's purpose
function description()
  return "An example script documenting all hooks"
end

-- Run default function. The current scope should include `arguments` as a
-- function which returns relevant options. Should return a boolean indicating
-- success or failure
function run()
  log_info("Running the example script")
  return true
end

-- Provide an array of completions given a prefix. The current scope should
-- include a `prefix` function which returns the relevant state. Should return
-- a comma-delimited list of items as a string
function complete_address()
  return ""
end

-- Provide an array of completions given a prefix. The current scope should
-- include a `prefix` function which returns the relevant state. Should return
-- a comma-delimited list of items as a string
function complete_command()
  return ""
end

-- Invoked when a URI will be loaded in a webview. The current scope includes
-- a `webview_index` and `window_index` indicating which view is active.
function will_load_uri()
end
