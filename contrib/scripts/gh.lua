function description()
  return "Opens a repo or performs a search on GitHub"
end

function run()
  windex = focused_window_index()
  webview_index = focused_webview_index(windex)
  if #arguments > 0 then
    owner, repo = string.match(arguments[1], "(%w+)/(%w+)")
    if owner ~= nil then
      target = string.format("https://github.com/%s/%s", owner, repo)
      load_uri(windex, webview_index, target)
    else
      query = url_encode(table.concat(arguments, " "))
      target = table.concat({"https://github.com/search?q=", query}, "")
      load_uri(windex, webview_index, target)
    end
  else
    load_uri(windex, webview_index, "https://github.com")
  end
  return true
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
