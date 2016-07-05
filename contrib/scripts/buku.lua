-- Bookmark management using buku (https://github.com/jarun/Buku)
-- and jq (https://stedolan.github.io/jq/)
function description()
  return "Save and open bookmarks"
end

function run()
  if #arguments > 0 then
    command = arguments[1]
    table.remove(arguments, 1)
    window_index = focused_window_index()
    webview_index = focused_webview_index(window_index)
    if command == "open" then
      return open_bookmark(window_index, webview_index)
    elseif command == "save" then
      return save_bookmark(window_index, webview_index)
    else
      log_info(string.format("Unknown command provided: %s", command))
    end
  end
  return false
end

-- Open the first bookmark matching arguments
function open_bookmark(window_index, webview_index)
  query = table.concat(arguments, " ")
  command = string.format("buku -j --noprompt -s '%s' | jq 'map(.uri)[0]'", query)
  handle = io.popen(command)
  uri = handle:read("*a"):gsub("\"", ""):gsub("%s+$", "")
  if #uri > 1 then
    load_uri(window_index, webview_index, uri)
    return true
  end
  return false
end

-- Save a bookmark using the webview title and URI and arguments as tags
function save_bookmark(window_index, webview_index)
  title = webview_title(window_index, webview_index)
  uri = webview_uri(window_index, webview_index)
  if #uri > 0 then
    tags = table.concat(arguments, ",")
    command = string.format("buku --noprompt -a %s --title '%s' --tag '%s'", uri, title, tags)
    log_info(string.format("Running command: %s", command))
    return os.execute(command) == 0
  end
  return false
end

-- Provide completions from the titles of bookmarks
function complete_command()
  if #arguments > 0 and arguments[1] == "open" then
    query = string.format("buku -j -s '%s' | jq 'map(.title)[]'", prefix:gsub("buku open ", ""))
    handle = io.popen(query)
    text = handle:read("*a")
    return text:gsub("\"", ""):gsub("\n", ",")
  end
  return ""
end
