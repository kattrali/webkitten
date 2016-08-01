function description()
  return "Opens a repo, performs a search on GitHub, or opens a common page"
end

function run()
  local windex = focused_window_index()
  local webview_index = focused_webview_index(windex)
  if #arguments > 0 then
    if arguments[1] == "status" then
      load_uri(windex, webview_index, "https://status.github.com")
    elseif arguments[1] == "gist" then
      load_uri(windex, webview_index, "https://gist.github.com")
    elseif arguments[1] == "help" then
      load_uri(windex, webview_index, "https://help.github.com")
    elseif arguments[1] == "api" then
      load_uri(windex, webview_index, "https://developer.github.com/v3/")
    elseif arguments[1] == "dev" then
      load_uri(windex, webview_index, "https://developer.github.com")
    else
      local owner, repo = string.match(arguments[1], "([%w-]+)/([%w-]+)")
      if owner ~= nil then
        target = string.format("https://github.com/%s/%s", owner, repo)
        load_uri(windex, webview_index, target)
      else
        query = url_encode(table.concat(arguments, " "))
        target = table.concat({"https://github.com/search?q=", query}, "")
        load_uri(windex, webview_index, target)
      end
    end
  else
    load_uri(windex, webview_index, "https://github.com")
  end
  return true
end

function complete_command()
  local subcommands = {"api","dev","gist", "help", "status"}
  local query = table.concat(arguments," ")
  if #query == 0 then
    return table.concat(subcommands,",")
  end
  for i, cmd in pairs(subcommands) do
    if string.sub(cmd, 1, #query) == query then
      return cmd
    end
  end
  return ""
end

function url_encode(str)
  if (str) then
  str = string.gsub (str, "\n", "\r\n")
  str = string.gsub (str, "([^%w %-%_%.%~])",
    function (c) return string.format ("%%%02X", string.byte(c)) end)
  str = string.gsub (str, " ", "+")
  end
  return str
end
